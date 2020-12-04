use std::convert::Infallible;
use serde::{Serialize, Deserialize};

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
    access_token: String,
    // expires_in: i32,
    // token_type: String,
    // scope: String,
    // refresh_token: String
    id_token: String
}

async fn request_access_token(client: &reqwest::Client, authorization_code: String) -> Result<TokenResponse, String> {
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
struct UserinfoResponse {
    //family_name: String,
    //given_name: String,
    id: String,
    //locale: String,
    name: String,
    picture: String,
}

async fn request_userinfo(client: &reqwest::Client, access_token: String) -> Result<UserinfoResponse, String> {
    let response = client.get("https://www.googleapis.com/userinfo/v2/me")
        .header("Authorization", "Bearer ".to_owned() + access_token.as_str())
        .send()
        .await;

    match response {
        Ok(r) => Ok(r.json::<UserinfoResponse>().await.unwrap()),
        Err(e) => Err(e.to_string())
    }
}

pub async fn auth_success(res: AuthSuccess) -> Result<impl warp::Reply, Infallible> {
    // Should create this once and reuse it.
    // It uses a connection pool internally.
    let client = reqwest::Client::new();

    let token = match request_access_token(&client, res.code).await {
        Ok(t) => t,
        Err(e) => return Ok(format!("An error has occurred: {}", e))
    };

    let userinfo = match request_userinfo(&client, token.access_token).await {
        Ok(u) => u,
        Err(e) => return Ok(format!("An error has occurred: {}", e))
    };

    // By doing "cryptography stuff", we can get the id from id_token.
    // I guess that's faster because we don't need to do a request to the
    // userinfo endpoint.

    Ok(format!("id: {}, id_token: {}", userinfo.id, token.id_token))
}

pub async fn auth_fail(res: AuthFail) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("Failed authentication: {}", res.error))
}
