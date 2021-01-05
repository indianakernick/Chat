use askama::Template;
use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;
use crate::{utils::cache_long, socket};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    redirect_url: String,
    google_auth_url: String,
}

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: String,
}

pub async fn login(query: LoginQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let mut google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?redirect_uri=https://localhost/api/auth&response_type=code&scope=profile&client_id={}&state=",
        include_str!("../../api/client_id.txt")
    );
    google_auth_url.extend(form_urlencoded::byte_serialize(query.redirect.as_bytes()));
    Ok(cache_long(LoginTemplate {
        redirect_url: query.redirect,
        google_auth_url,
    }))
}

pub async fn logout(pool: Pool, socket_ctx: socket::Context, session_id: db::SessionID)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if let Some(user_id) = db::session_user_id(pool.clone(), &session_id).await? {
        db::delete_user_sessions(pool, user_id).await?;
        socket_ctx.kick(user_id).await;
    }
    Ok(login(LoginQuery { redirect: "/".to_owned() }).await?)
}
