use crate::database as db;
use deadpool_postgres::Pool;
use crate::utils::cache_short;

pub async fn get_group_info(group_id: db::GroupID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let group_info = match db::group_info(pool, group_id).await? {
        Some(info) => info,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    Ok(Box::new(cache_short(warp::reply::json(&group_info))))
}

// Would it make sense for this endpoint to also return group info?
pub async fn get_group_channels(group_id: db::GroupID, pool: Pool)
    -> Result<impl warp::Reply, warp::Rejection>
{
    Ok(cache_short(warp::reply::json(
        &db::group_channels(pool, group_id).await?
    )))
}
