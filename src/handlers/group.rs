use crate::database as db;
use deadpool_postgres::Pool;
use crate::utils::cache_short;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize)]
#[serde(tag="type")]
enum Response {
    #[serde(rename="error")]
    Error { message: &'static str },
    #[serde(rename="success")]
    Success { group_id: db::GroupID, channel_id: db::ChannelID }
}

#[derive(Deserialize)]
pub struct Request {
    name: String,
    picture: String,
}

// {"name":"","picture":""}
pub const CREATE_GROUP_LIMIT: u64 =
    (4 * db::MAX_GROUP_NAME_LENGTH + 4 * db::MAX_URL_LENGTH + 24) as u64;

pub async fn create_group(pool: Pool, request: Request)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if !db::valid_group_name(&request.name) {
        return Ok(Box::new(warp::reply::json(
            &Response::Error {
                message: "Invalid group name"
            }
        )));
    }

    // TODO: Should we also GET the url and check if we get a 200 status and an
    // image file?
    if !db::valid_url(&request.picture) {
        return Ok(Box::new(warp::reply::json(
            &Response::Error {
                message: "Invalid url"
            }
        )));
    }

    let group_id = match db::create_group(pool.clone(), request.name, request.picture).await? {
        Some(id) => id,
        None => {
            return Ok(Box::new(warp::reply::json(
                &Response::Error {
                    message: "Duplicate group name"
                }
            )));
        }
    };

    // Unwrapping the Option returned by create_channel because it is None if
    // the channel name is not unique within the group. We just created the
    // group so it must be unique.
    let channel_id = db::create_channel(
        pool.clone(), group_id, &"general".to_owned()
    ).await.map_err(|e| crate::error::Error::Database(e))?.unwrap();

    Ok(Box::new(warp::reply::json(
        &Response::Success {
            group_id,
            channel_id
        }
    )))
}
