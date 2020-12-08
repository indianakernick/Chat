use log::error;
use warp::Filter;
use super::handlers;
use deadpool_postgres::Pool;
use std::convert::Infallible;

fn with_pool(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
        .recover(rejection)
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // TODO: Consider setting cache-control header for static files
    warp::get()
        .and(warp::fs::dir("client/dist"))
        .recover(rejection)
}

pub fn me_with_session(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "me"))
        .and(warp::cookie("session_id"))
        .and(with_pool(pool))
        .and_then(handlers::me)
        .recover(rejection)
}

pub fn me_without_session() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "me"))
        .map(|| warp::http::StatusCode::UNAUTHORIZED)
        .recover(rejection)
}

pub fn socket(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let conns = handlers::Connections::default();

    warp::ws()
        .and(warp::path!("api" / "socket"))
        .and(warp::cookie("session_id"))
        .and(with_pool(pool))
        .and(warp::any().map(move || conns.clone()))
        .map(handlers::upgrade)
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
async fn rejection(error: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = error.find::<crate::error::Error>() {
        error!("{}", e);
        Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(error)
    }
}
