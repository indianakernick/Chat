use crate::database as db;
use deadpool_postgres::Pool;
use crate::utils::cache_short;

pub async fn user(user_id: db::UserID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user = match db::user(pool, user_id).await? {
        Some(info) => info,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    Ok(Box::new(cache_short(warp::reply::json(&user))))
}
