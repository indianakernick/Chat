use log::error;
use crate::error::Error;
use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::errors::ErrorKind as JWTErrorKind;

use headers::Header;
use headers::CacheControl;
use std::time::SystemTime;
use std::convert::Infallible;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, decode_header, Algorithm, Validation, DecodingKey};

/*
Full explanation
https://developers.google.com/identity/protocols/oauth2/web-server#httprest_5

The authentication flow starts when the user clicks a link:
https://accounts.google.com/o/oauth2/v2/auth?
  client_id=xxx.apps.googleusercontent.com&
  redirect_uri=https://localhost/api/auth&
  response_type=code&
  scope=profile

If the user accepts signs in, they'll be redirected to (AuthSuccess)
https://localhost/api/auth?code=xxx&scope=xxx

Otherwise, they'll be redirected to (AuthFail)
https://localhost/api/auth?error=xxx

The code parameter is an authorization code. Using this code, we can
request an id token. We do this by sending a POST to (TokenRequest)
https://oauth2.googleapis.com/token

From this, we obtain a (TokenResponse) containing the id token. The id token is
a JWT (json web token). The JWT is decoded to obtain the profile info. In order
to verify it, a certificate must be obtained.

Certificates are obtained from
https://www.googleapis.com/oauth2/v3/certs
These certificates expire so the max-age directive of the Cache-Control header
is inspected so that the certificate is only requested when the cached
certificate expires.
*/

#[derive(Deserialize)]
pub struct AuthSuccess {
    code: String,
    state: String
    // TODO: Do we need to check that scope is as expected?
    // scope: String
}

#[derive(Deserialize)]
pub struct AuthFail {
    error: String
}

#[derive(Serialize)]
struct TokenRequest {
    client_id: &'static str,
    client_secret: &'static str,
    code: String,
    grant_type: &'static str,
    redirect_uri: &'static str
}

#[derive(Deserialize)]
struct TokenResponse {
    id_token: String,
    // access_token: String,
    // expires_in: i32,
    // token_type: String,
    // scope: String,
    // refresh_token: String,
}

async fn request_id_token(client: &reqwest::Client, authorization_code: String) -> Result<TokenResponse, Error> {
    let request = TokenRequest {
        client_id: include_str!("../../api/client_id.txt"),
        client_secret: include_str!("../../api/client_secret.txt"),
        code: authorization_code,
        grant_type: "authorization_code",
        redirect_uri: "https://localhost/api/auth"
    };
    Ok(client.post("https://oauth2.googleapis.com/token")
        .form(&request)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?
    )
}

#[derive(Deserialize)]
struct Certificate {
    kid: String, // Key ID
    n: String, // RSA modulus
    e: String, // RSA exponent
    // alg: String,
    // kty: String,
    // r#use: String,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Certs {
    keys: Vec<Certificate>,
    #[serde(skip_deserializing)]
    expire: SystemTime
}

impl Default for Certs {
    fn default() -> Certs {
        Certs {
            keys: Vec::<Certificate>::default(),
            expire: std::time::UNIX_EPOCH
        }
    }
}

pub type CertificateCache = std::sync::Arc<tokio::sync::Mutex<Certs>>;

async fn update_cert_cache(client: &reqwest::Client, cached_certs: &mut Certs) -> Result<(), Error> {
    let now = SystemTime::now();
    if cached_certs.expire > now {
        return Ok(());
    }

    let response = client.get("https://www.googleapis.com/oauth2/v3/certs")
        .send()
        .await?;
    let headers = response.headers();
    let mut iter = headers
        .get_all(CacheControl::name())
        .iter();
    let cache_control = CacheControl::decode(&mut iter)?;
    let certs = response.json::<Certs>().await?;

    cached_certs.keys = certs.keys;
    cached_certs.expire = now + cache_control.max_age().unwrap();

    Ok(())
}

#[derive(Deserialize)]
pub struct Claims {
    iss: String, // Issuer
    // aud: String, // Audience
    // exp: usize, // Expire

    pub sub: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,

    // TODO: I feel like this might not belong here
    #[serde(skip)]
    pub redirect: String
}

fn decode_id_token(certs: &Certs, id_token: &str) -> Result<Claims, Error> {
    let header = decode_header(id_token)?;

    // The header contains a kid (key ID) field that identifies the key to use
    // from the list of keys.
    //
    // We're returning the InvalidAlgorithmName error here in case it isn't
    // present. This error is only used when Algorithm::from_str is called which
    // we're not using.
    let header_kid = match header.kid {
        Some(k) => k,
        None => return Err(JWTError::from(JWTErrorKind::InvalidAlgorithmName).into())
    };

    // Search the list of keys for the one with the matching ID and use that
    // for decoding.
    for cert in certs.keys.iter() {
        if cert.kid == header_kid {
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[include_str!("../../api/client_id.txt")]);
            let key = DecodingKey::from_rsa_components(&cert.n, &cert.e);
            let token_data = decode::<Claims>(id_token, &key, &validation)?;

            // We can't set the iss field of Validation because it only accepts
            // one value but the issuer can be one of two values.
            match token_data.claims.iss.as_str() {
                "accounts.google.com" | "https://accounts.google.com" => {},
                _ => return Err(JWTError::from(JWTErrorKind::InvalidIssuer).into())
            };

            return Ok(token_data.claims);
        }
    }

    Err(JWTError::from(JWTErrorKind::InvalidAlgorithmName).into())
}

pub async fn auth_success(client: reqwest::Client, cache: CertificateCache, res: AuthSuccess) -> Result<Claims, warp::Rejection> {
    let token = request_id_token(&client, res.code).await?;
    let mut certs = cache.lock().await;
    update_cert_cache(&client, &mut *certs).await?;
    let mut claims = decode_id_token(&certs, token.id_token.as_str())?;
    claims.redirect = res.state;
    Ok(claims)
}

pub async fn auth_fail(res: AuthFail) -> Result<impl warp::Reply, Infallible> {
    error!("Google auth error: {}", res.error);
    Ok(warp::redirect(warp::http::Uri::from_static("/")))
}
