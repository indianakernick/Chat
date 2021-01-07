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
pub struct CreateGroupRequest {
    name: String,
    picture: String,
}

pub const CREATE_GROUP_LIMIT: u64 =
    ("{'name':'','picture':''}".len() + 4 * db::MAX_GROUP_NAME_LENGTH + 4 * db::MAX_URL_LENGTH) as u64;

fn error_response(message: &'static str) -> Box<dyn warp::Reply> {
    Box::new(warp::reply::json(
        &Response::Error { message }
    ))
}

pub async fn create_group(pool: Pool, session_id: String, request: CreateGroupRequest)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if !db::valid_group_name(&request.name) {
        return Ok(error_response("name_invalid"));
    }

    if !db::valid_url(&request.picture) {
        return Ok(error_response("picture_invalid"));
    }

    // Someone without an account could check if a group name exists but I don't
    // see why that would be a problem.
    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
    };

    let group_id = match db::create_group(pool.clone(), request.name, request.picture).await? {
        Some(id) => id,
        None => return Ok(error_response("name_exists"))
    };

    let (channel_id, joined) = futures::future::join(
        db::create_channel(pool.clone(), group_id, &"general".to_owned()),
        db::join_group(pool.clone(), user_id, group_id)
    ).await;

    // Unwrapping the Option returned by create_channel because it is None if
    // the channel name is not unique within the group. We just created the
    // group so it must be unique.
    channel_id.map_err(|e| crate::error::Error::Database(e))?.unwrap();
    joined?;

    Ok(Box::new(warp::reply::json(
        &Response::Success { group_id }
    )))
}
