use bytes::Buf;
use crate::database as db;
use futures::TryStreamExt;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};
use warp::filters::multipart::{Part, FormData};

#[derive(Serialize)]
#[serde(tag="type")]
enum Response {
    #[serde(rename="error")]
    Error { message: &'static str },
    #[serde(rename="success")]
    Success { group_id: db::GroupID },
}

fn valid_request(parts: &Vec<Part>) -> bool {
    if parts.len() != 2 {
        return false;
    }
    if parts[0].name() != "name" {
        return false;
    }
    if parts[1].name() != "picture" {
        return false;
    }
    true
}

fn error_response(message: &'static str) -> Box<dyn warp::Reply> {
    Box::new(warp::reply::json(
        &Response::Error { message }
    ))
}

pub async fn create_group(form: FormData, pool: Pool, session_id: String)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let parts = form.try_collect::<Vec<_>>().await;
    let mut parts = match parts {
        Ok(parts) => parts,
        Err(_) => return Ok(error_response("request_invalid"))
    };

    if !valid_request(&parts) {
        return Ok(error_response("request_invalid"));
    }

    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
    };

    let name_buf = match parts[0].data().await {
        Some(Ok(buf)) => buf,
        Some(Err(_)) | None => return Ok(error_response("name_invalid"))
    };

    let name = match String::from_utf8(name_buf.bytes().into()) {
        Ok(name) => name,
        Err(_) => return Ok(error_response("name_invalid"))
    };

    if !db::valid_group_name(&name) {
        return Ok(error_response("name_invalid"));
    }

    let picture_buf = match parts[1].data().await {
        Some(Ok(buf)) => buf,
        Some(Err(_)) | None => return Ok(error_response("picture_invalid"))
    };

    let opt_picture_buf = match crate::utils::optimize_png(picture_buf.bytes()) {
        Some(picture) => picture,
        None => return Ok(error_response("picture_invalid"))
    };

    let group_id = match db::create_group(pool.clone(), name).await? {
        Some(id) => id,
        None => return Ok(error_response("name_exists"))
    };

    let path = format!("img/group/{}_64.png", group_id);
    if let Err(_) = tokio::fs::write(path, opt_picture_buf.as_slice()).await {
        return Ok(error_response("fs"));
    }

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
