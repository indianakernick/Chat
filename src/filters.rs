use warp::Filter;
use super::handlers;
use log::{debug, error};
use crate::error::Error;
use deadpool_postgres::Pool;
use std::convert::Infallible;
use crate::utils::cache_long;
use crate::database::{ChannelID, UserID, GroupID, InviteID, SessionID};

fn with_pool(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and(warp::query::<handlers::LoginQuery>())
        .and_then(handlers::login)
        .recover(rejection)
}

fn session_id() -> impl Filter<Extract = (SessionID,), Error = Infallible> + Clone {
    warp::any()
        .and(warp::cookie::optional("session_id"))
        .map(|session_id: Option<String>| session_id.unwrap_or(String::new()))
}

pub fn channel(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("channel" / GroupID / ChannelID)
        .and(warp::get())
        .and(session_id())
        .and(with_pool(pool))
        .and_then(handlers::channel)
        .recover(rejection)
}

pub fn invite(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("invite" / InviteID)
        .and(warp::get())
        .and(session_id())
        .and(with_pool(pool))
        .and_then(handlers::accept_invite)
        .recover(rejection)
}

pub fn create_group(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "group" / "create")
        .and(warp::post())
        .and(with_pool(pool))
        .and(warp::cookie("session_id"))
        .and(warp::body::content_length_limit(handlers::CREATE_GROUP_LIMIT))
        .and(warp::body::json())
        .and_then(handlers::create_group)
        .recover(rejection)
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("favicon.ico")
        .and(warp::get())
        .and(warp::fs::file("client/dist/favicon.ico"))
        .map(cache_long)
        .recover(rejection)
}

pub fn js() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("js")
        .and(warp::get())
        .and(warp::fs::dir("client/dist/js"))
        .map(cache_long)
        .recover(rejection)
}

pub fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("css")
        .and(warp::get())
        .and(warp::fs::dir("client/dist/css"))
        .map(cache_long)
        .recover(rejection)
}

pub fn user(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "user" / UserID)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::user)
        .recover(rejection)
}

pub fn socket(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ctx = crate::socket::SocketContext::new(pool);

    warp::path!("api" / "socket" / GroupID)
        .and(warp::ws())
        .and(warp::cookie("session_id"))
        .and(warp::any().map(move || ctx.clone()))
        .and_then(crate::socket::upgrade)
        .recover(rejection)
}

pub fn auth_success(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let client = reqwest::Client::new();
    let cert_cache = handlers::CertificateCache::default();

    warp::path!("api" / "auth")
        .and(warp::get())
        .and(warp::any().map(move || client.clone()))
        .and(warp::any().map(move || cert_cache.clone()))
        .and(warp::query::<handlers::AuthSuccess>())
        .and_then(handlers::auth_success)
        .and(with_pool(pool))
        .and_then(handlers::initialize_session)
        .recover(rejection)
}

pub fn auth_fail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "auth")
        .and(warp::get())
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
