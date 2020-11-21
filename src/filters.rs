use warp::Filter;
use super::handlers;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("hello" / String))
        .map(move |name| {
            handlers::HelloTemplate::new(name)
        })
}

pub fn get_messages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "messages"))
        .map(move || {
            match handlers::read_database() {
                Ok(val) => warp::reply::json(&val),
                Err(e) => warp::reply::json(&format!("{:?}", e))
            }
        })
}

pub fn post_message() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "post_message"))
        .and(warp::body::json())
        .map(move |message: String| {
            match handlers::append_to_database(message) {
                Ok(_) => String::new(),
                Err(e) => format!("{:?}", e)
            }
        })
}
