use deadpool_postgres::Pool;
use crate::database::{UserID, user_info};

pub async fn user(user_id: UserID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let user_info = match user_info(pool, user_id).await? {
        Some(info) => info,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    Ok(Box::new(
        warp::reply::with_header(
            warp::reply::json(&user_info),
            "Cache-Control",
            "public,max-age=86400,immutable" // 24 hours
        )
    ))
}
