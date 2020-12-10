use warp::Filter;
use super::handlers;
use log::{debug, error};
use crate::error::Error;
use deadpool_postgres::Pool;
use std::convert::Infallible;

fn with_pool(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn cache_static<R: warp::Reply>(reply: R) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        "Cache-Control",
        "public,max-age=604800,immutable" // 7 days
    )
}

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
        .map(cache_static)
        .recover(rejection)
}

// TODO: Might need a query parameter to redirect back to after logging in
pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("login"))
        .and(warp::fs::file("client/dist/without_session.html"))
        .recover(rejection)
}

pub fn channel(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use handlers::ChannelID;

    let session_id = warp::any()
        .and(warp::cookie::optional("session_id"))
        .map(|session_id: Option<String>| session_id.unwrap_or(String::new()));

    warp::get()
        .and(warp::path!("channel" / ChannelID))
        .and(session_id)
        .and(with_pool(pool))
        .and_then(handlers::channel)
        .recover(rejection)
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("favicon.ico"))
        .and(warp::fs::file("client/dist/favicon.ico"))
        .map(cache_static)
        .recover(rejection)
}

pub fn js() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("js"))
        .and(warp::fs::dir("client/dist/js"))
        .map(cache_static)
        .recover(rejection)
}

pub fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("css"))
        .and(warp::fs::dir("client/dist/css"))
        .map(cache_static)
        .recover(rejection)
}

pub fn user(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    use handlers::UserID;

    warp::get()
        .and(warp::path!("api" / "user" / UserID))
        .and(with_pool(pool))
        .and_then(handlers::user)
        .recover(rejection)
}

pub fn socket(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let conns = handlers::Connections::default();

    warp::path!("api" / "socket")
        .and(warp::ws())
        .and(warp::cookie("session_id"))
        .and(with_pool(pool))
        .and(warp::any().map(move || conns.clone()))
        .and_then(handlers::upgrade)
        .recover(rejection)
}

pub fn auth_success(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cert_cache = handlers::CertificateCache::default();

    warp::get()
        .and(warp::path!("api" / "auth"))
        .map(move || cert_cache.clone())
        .and(warp::query::<handlers::AuthSuccess>())
        .and_then(handlers::auth_success)
        .and(with_pool(pool))
        .and_then(handlers::create_session)
        .recover(rejection)
}

pub fn auth_fail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "auth"))
        .and(warp::query::<handlers::AuthFail>())
        .and_then(handlers::auth_fail)
        .recover(rejection)
}

// This is technically a handler so maybe it doesn't belong in this file.
async fn rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = rejection.find::<Error>() {
        error!("{}", error);
        Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(rejection)
    }
}

pub async fn leaked_rejection(rejection: warp::Rejection) -> Result<warp::http::StatusCode, warp::Rejection> {
    debug!("Leaked: {:?}", rejection);
    Err(rejection)
}
