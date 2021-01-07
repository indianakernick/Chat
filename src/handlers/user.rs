use crate::socket;
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct RenameUserRequest {
    name: String,
    picture: String,
}

pub const RENAME_USER_LIMIT: u64 =
    ("{'name':'','picture':''}".len() + db::MAX_USER_NAME_LENGTH + db::MAX_URL_LENGTH) as u64;

pub async fn rename_user(session_id: db::SessionID, request: RenameUserRequest, pool: Pool, socket_ctx: socket::Context)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
    };

    if !db::valid_user_name(&request.name) {
        return Ok(Box::new("name_invalid"));
    }

    if !db::valid_url(&request.picture) {
        return Ok(Box::new("picture_invalid"));
    }

    if !db::rename_user(pool.clone(), user_id, &request.name, &request.picture).await? {
        return Ok(Box::new("name_exists"));
    }

    let groups = db::user_all_groups(pool, user_id).await?;
    socket_ctx.rename_user(groups, user_id, &request.name, &request.picture).await;

    return Ok(Box::new(warp::http::StatusCode::NO_CONTENT))
}
