use warp::Filter;
use super::handlers;
use deadpool_postgres::Pool;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::fs::dir("client/dist"))
}

pub fn socket(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let conns = handlers::Connections::default();

    warp::ws()
        .and(warp::path!("api" / "socket"))
        .map(move |ws: warp::ws::Ws| {
            handlers::upgrade(ws, conns.clone(), pool.clone())
        })
}

pub fn auth_success() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cert_cache = handlers::CertificateCache::default();

    warp::get()
        .and(warp::path!("api" / "auth"))
        .map(move || {
            cert_cache.clone()
        })
        .and(warp::query::<handlers::AuthSuccess>())
        .and_then(handlers::auth_success)
}

pub fn auth_fail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "auth"))
        .and(warp::query::<handlers::AuthFail>())
        .and_then(handlers::auth_fail)
}
