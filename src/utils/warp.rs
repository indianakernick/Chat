pub fn cache_long<R: warp::Reply>(reply: R) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        "Cache-Control",
        "public,max-age=604800,immutable" // 7 days
    )
}

pub fn cache_short<R: warp::Reply>(reply: R) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        "Cache-Control",
        "public,max-age=86400,immutable" // 24 hours
    )
}
