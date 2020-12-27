use crate::database as db;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(tag="type")]
enum Response {
    #[serde(rename="error")]
    Error { message: &'static str },
    #[serde(rename="success")]
    Success { group_id: db::GroupID },
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
    db::create_channel(
        pool.clone(), group_id, &"general".to_owned()
    ).await.map_err(|e| crate::error::Error::Database(e))?.unwrap();

    Ok(Box::new(warp::reply::json(
        &Response::Success {
            group_id
        }
    )))
}
