use askama::Template;
use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;
use crate::{utils::cache_long, socket};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
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
    Ok(cache_long(LoginTemplate { google_auth_url }))
}

pub async fn logout(pool: Pool, socket_ctx: socket::SocketContext, session_id: db::SessionID)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let redirect = warp::redirect(warp::http::Uri::from_static("/login?redirect=/"));
    match db::session_user_id(pool.clone(), &session_id).await? {
        Some(user_id) => {
            socket::kick(socket_ctx, user_id).await;
            db::delete_session(pool, &session_id).await?;
            Ok(Box::new(warp::reply::with_header(
                redirect,
                "Set-Cookie",
                "session_id=;Path=/;HttpOnly;Secure;Expires=Thu, 01 Jan 1970 00:00:00 GMT"
            )))
        },
        None => {
            Ok(Box::new(redirect))
        }
    }
}
