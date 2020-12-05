use headers::Header;
use headers::CacheControl;
use std::time::SystemTime;
use std::convert::Infallible;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey};

/*
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
    scope: String
}

#[derive(Deserialize)]
pub struct AuthFail {
    error: String
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: String,
    grant_type: &'a str,
    redirect_uri: &'a str
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

async fn request_id_token(client: &reqwest::Client, authorization_code: String) -> Result<TokenResponse, String> {
    let request = TokenRequest {
        client_id: include_str!("../../api/client_id.txt"),
        client_secret: include_str!("../../api/client_secret.txt"),
        code: authorization_code,
        grant_type: "authorization_code",
        redirect_uri: "https://localhost/api/auth"
    };
    let response = client.post("https://oauth2.googleapis.com/token")
        .form(&request)
        .send()
        .await;

    match response {
        Ok(r) => Ok(r.json::<TokenResponse>().await.unwrap()),
        Err(e) => Err(e.to_string())
    }
}

#[derive(Deserialize)]
struct Certificate {
    kid: String, // Key ID
    n: String, // RSA modulus
    e: String, // RSA exponent
    //alg: String,
    //kty: String,
    //r#use: String,
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

async fn update_cert_cache(client: &reqwest::Client, cached_certs: &mut Certs) -> Result<(), String> {
    let now = SystemTime::now();
    if cached_certs.expire > now {
        return Ok(());
    }

    let response = match client.get("https://www.googleapis.com/oauth2/v3/certs").send().await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };

    let headers = response.headers();
    let mut iter = headers
        .get_all(CacheControl::name())
        .iter();
    let cache_control = match CacheControl::decode(&mut iter) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };

    let certs = match response.json::<Certs>().await {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };

    cached_certs.keys = certs.keys;
    cached_certs.expire = now + cache_control.max_age().unwrap();

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Claims {
    iss: String, // Issuer
    aud: String, // Audience
    exp: usize, // Expire

    sub: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
}

fn decode_id_token(certs: &Certs, id_token: &str) -> Result<Claims, String> {
    let header = match jsonwebtoken::decode_header(id_token) {
        Ok(h) => h,
        Err(e) => return Err(e.to_string())
    };
    // the header contains a kid (key ID) field that identifies the key to use
    // from the list of keys.
    let header_kid = match header.kid {
        Some(k) => k,
        None => return Err(String::from("Header is missing KID"))
    };
    // Search the list of keys for the one with the matching ID and use that
    // for decoding.
    for cert in certs.keys.iter() {
        if cert.kid == header_kid {
            // Found the right key.
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[include_str!("../../api/client_id.txt")]);
            let key = DecodingKey::from_rsa_components(&cert.n, &cert.e);
            let token_data = match decode::<Claims>(id_token, &key, &validation) {
                Ok(t) => t,
                Err(e) => return Err(e.to_string())
            };

            // We can't set the iss field of Validation because it only accepts
            // one value but the issuer can be one of two values.
            match token_data.claims.iss.as_str() {
                "accounts.google.com" | "https://accounts.google.com" => {},
                _ => return Err(String::from("Invalid issuer"))
            };

            return Ok(token_data.claims);
        }
    }

    Err(String::from("Invalid KID"))
}

pub async fn auth_success(cache: CertificateCache, res: AuthSuccess) -> Result<impl warp::Reply, Infallible> {
    // TODO: Should create this once and reuse it.
    // It uses a connection pool internally.
    let client = reqwest::Client::new();

    let token = match request_id_token(&client, res.code).await {
        Ok(t) => t,
        Err(e) => return Ok(format!("An error has occurred: {}", e))
    };

    let claims;
    {
        let mut certs = cache.lock().await;

        if let Err(e) = update_cert_cache(&client, &mut *certs).await {
            return Ok(format!("An error has occurred: {}", e));
        }

        claims = match decode_id_token(&certs, token.id_token.as_str()) {
            Ok(c) => c,
            Err(e) => return Ok(format!("An error has occurred: {}", e))
        };
    }

    Ok(format!("id: {}, name: {}", claims.sub, claims.name))
}

pub async fn auth_fail(res: AuthFail) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("Failed authentication: {}", res.error))
}
