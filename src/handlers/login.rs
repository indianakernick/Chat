use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "../client/dist/login.html")]
struct LoginTemplate {
    redirect: String
}

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: String
}

pub async fn login(query: LoginQuery) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(LoginTemplate {
        redirect: serde_json::to_string(&query.redirect).unwrap()
    })
}
