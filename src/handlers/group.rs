use crate::socket;
use crate::database as db;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(tag="type")]
#[serde(rename_all="snake_case")]
enum Response {
    Error { message: &'static str },
    Success { group_id: db::GroupID },
}

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    name: String,
    picture: String,
}

pub const CREATE_GROUP_LIMIT: u64 =
    ("{'name':'','picture':''}".len() + 4 * db::MAX_GROUP_NAME_LENGTH + 4 * db::MAX_URL_LENGTH) as u64;

// use status codes to differentiate between success and failure
// 400 bad request
// 201 created

fn error_response(message: &'static str) -> Box<dyn warp::Reply> {
    Box::new(warp::reply::json(
        &Response::Error { message }
    ))
}

pub async fn create_group(session_id: String, request: CreateGroupRequest, pool: Pool)
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

pub async fn delete_group(group_id: db::GroupID, session_id: db::SessionID, pool: Pool, socket_ctx: socket::Context)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(warp::http::StatusCode::UNAUTHORIZED)
    };

    if !db::group_member(pool.clone(), user_id, group_id).await? {
        return Ok(warp::http::StatusCode::FORBIDDEN);
    }

    let users = db::group_user_ids(pool.clone(), group_id).await.map_err(|e| crate::error::Error::Database(e))?;
    db::delete_group(pool.clone(), group_id).await?;
    socket_ctx.delete_group(users, group_id).await;
    Ok(warp::http::StatusCode::NO_CONTENT)
}
