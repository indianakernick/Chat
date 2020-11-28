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
