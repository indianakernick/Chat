use askama::Template;
use serde::Deserialize;
use crate::utils::cache_long;

#[derive(Template)]
#[template(path = "../client/dist/login.html")]
struct LoginTemplate {
    google_auth_url: String
}

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: String
}

pub async fn login(query: LoginQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let mut google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?redirect_uri=https://localhost/api/auth&response_type=code&scope=profile&client_id={}&state=",
        include_str!("../../api/client_id.txt")
    );
    google_auth_url.extend(form_urlencoded::byte_serialize(query.redirect.as_bytes()));
    Ok(cache_long(LoginTemplate { google_auth_url }))
}
