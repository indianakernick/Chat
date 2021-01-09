use warp::Filter;
use log::{debug, error};
use crate::error::Error;
use deadpool_postgres::Pool;
use std::convert::Infallible;
use crate::utils::cache_long;
use super::{handlers, socket};
use crate::database::{ChannelID, UserID, GroupID, InviteID, SessionID};

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn with_session_id() -> impl Filter<Extract = (SessionID,), Error = Infallible> + Clone {
    warp::any()
        .and(warp::cookie::optional("session_id"))
        .map(|session_id: Option<String>| session_id.unwrap_or(String::new()))
}

pub fn root(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .map(|session_id, pool| (0, 0, session_id, pool))
        .untuple_one()
        .and_then(handlers::channel)
        .recover(rejection)
}

pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and(warp::query::<handlers::LoginQuery>())
        .and_then(handlers::login)
        .recover(rejection)
}

pub fn logout(pool: Pool, socket_ctx: socket::Context)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("logout")
        .and(warp::get())
        .and(with_state(pool))
        .and(with_state(socket_ctx))
        .and(with_session_id())
        .and_then(handlers::logout)
        .recover(rejection)
}

pub fn channel(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("channel" / GroupID / ChannelID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::channel)
        .recover(rejection)
}

pub fn invite(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("invite" / InviteID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::accept_invite)
        .recover(rejection)
}

pub fn create_group(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "group")
        .and(warp::post())
        .and(warp::cookie("session_id"))
        .and(warp::body::content_length_limit(handlers::CREATE_GROUP_LIMIT))
        .and(warp::body::json())
        .and(with_state(pool))
        .and_then(handlers::create_group)
        .recover(rejection)
}

pub fn delete_group(pool: Pool, socket_ctx: socket::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "group" / GroupID)
        .and(warp::delete())
        .and(warp::cookie("session_id"))
        .and(with_state(pool))
        .and(with_state(socket_ctx))
        .and_then(handlers::delete_group)
        .recover(rejection)
}

pub fn create_invite(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "invite")
        .and(warp::post())
        .and(warp::cookie("session_id"))
        .and(warp::body::content_length_limit(handlers::CREATE_INVITE_LIMIT))
        .and(warp::body::json())
        .and(with_state(pool))
        .and_then(handlers::create_invite)
        .recover(rejection)
}

pub fn leave_group(pool: Pool, socket_ctx: socket::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "leave" / GroupID)
        .and(warp::post())
        .and(warp::cookie("session_id"))
        .and(with_state(pool))
        .and(with_state(socket_ctx))
        .and_then(handlers::leave_group)
        .recover(rejection)
}

pub fn user(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "user" / UserID)
        .and(warp::get())
        .and(with_state(pool))
        .and_then(handlers::user)
        .recover(rejection)
}

pub fn rename_user(pool: Pool, socket_ctx: socket::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "user")
        .and(warp::put())
        .and(warp::cookie("session_id"))
        .and(warp::body::content_length_limit(handlers::RENAME_USER_LIMIT))
        .and(warp::body::json())
        .and(with_state(pool))
        .and(with_state(socket_ctx))
        .and_then(handlers::rename_user)
        .recover(rejection)
}

pub fn delete_user(pool: Pool, socket_ctx: socket::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "user")
        .and(warp::delete())
        .and(warp::cookie("session_id"))
        .and(with_state(pool))
        .and(with_state(socket_ctx))
        .and_then(handlers::delete_user)
        .recover(rejection)
}

pub fn socket(socket_ctx: socket::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "socket" / GroupID)
        .and(warp::ws())
        .and(warp::cookie("session_id"))
        .and(with_state(socket_ctx))
        .and_then(socket::Context::upgrade)
        .recover(rejection)
}

pub fn auth_success(pool: Pool, client: reqwest::Client, cert_cache: handlers::CertificateCache)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("api" / "auth")
        .and(warp::get())
        .and(warp::query::<handlers::AuthSuccess>())
        .and(with_state(pool))
        .and(with_state(client))
        .and(with_state(cert_cache))
        .and_then(handlers::auth_success)
        .recover(rejection)
}

pub fn auth_fail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "auth")
        .and(warp::get())
        .and(warp::query::<handlers::AuthFail>())
        .and_then(handlers::auth_fail)
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
