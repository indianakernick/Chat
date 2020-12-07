use log::info;
use warp::Filter;
use super::handlers;
use deadpool_postgres::Pool;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // TODO: Consider setting cache-control header for static files
    warp::get()
        .and(warp::fs::dir("client/dist"))
}

pub fn me_with_session(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "me"))
        .map(move || pool.clone())
        .and(warp::cookie("session_id"))
        .and_then(handlers::me)
}

pub fn me_without_session() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "me"))
        .map(|| Ok(warp::http::StatusCode::UNAUTHORIZED))
}

pub fn socket(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let conns = handlers::Connections::default();

    warp::ws()
        .and(warp::path!("api" / "socket"))
        .map(move |ws: warp::ws::Ws| {
            handlers::upgrade(ws, conns.clone(), pool.clone())
        })
}

pub fn auth_success(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cert_cache = handlers::CertificateCache::default();

    warp::get()
        .and(warp::path!("api" / "auth"))
        .map(move || cert_cache.clone())
        .and(warp::query::<handlers::AuthSuccess>())
        .and_then(handlers::auth_success)
        .map(move |claims| (pool.clone(), claims))
        .untuple_one()
        .and_then(handlers::create_session)
}

pub fn auth_fail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "auth"))
        .and(warp::query::<handlers::AuthFail>())
        .and_then(handlers::auth_fail)
}

// This is technically a handler so maybe it doesn't belong in this file.
pub async fn rejection(error: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = error.find::<crate::error::Error>() {
        info!("{}", e);
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR
        ))
    } else {
        Err(error)
    }
}
