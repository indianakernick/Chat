use warp::Filter;
use super::handlers;
use deadpool_postgres::Pool;

fn with_pool(pool: Pool) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
}

pub fn get_messages(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "messages"))
        .and(with_pool(pool))
        .and_then(handlers::get_messages)
}

pub fn post_message(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "post_message"))
        .and(warp::body::json())
        .and(with_pool(pool))
        .and_then(handlers::post_message)
}
