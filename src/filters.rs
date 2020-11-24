use warp::Filter;
use super::handlers;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .and_then(handlers::hello)
}

pub fn get_messages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "messages"))
        .and_then(handlers::get_messages)
}

pub fn post_message() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "post_message"))
        .and(warp::body::json())
        .and_then(handlers::post_message)
}
