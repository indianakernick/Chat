use lexical_core::Number;
use crate::database as db;
use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};

pub async fn accept_invite(invite_id: db::InviteID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/invite/{}", invite_id)
                .parse::<warp::http::Uri>().unwrap()
        )))
    };

    let group_id = match db::invitation_group_id(pool.clone(), invite_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    // This returns false if the user is already a member of the group but that
    // doesn't matter because either way, we should take the user to the group.
    db::join_group(pool.clone(), user_id, group_id).await?;

    super::channel(group_id, 0, session_id, pool).await
}

#[derive(Serialize)]
struct Response {
    invite_id: db::InviteID
}

#[derive(Deserialize)]
pub struct CreateInviteRequest {
    group_id: db::GroupID
}

pub const CREATE_INVITE_LIMIT: u64 =
    ("{'group_id':}".len() + db::GroupID::FORMATTED_SIZE_DECIMAL) as u64;

pub async fn create_invite(pool: Pool, session_id: db::SessionID, request: CreateInviteRequest)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user_id = match db::session_user_id(pool.clone(), &session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::http::StatusCode::UNAUTHORIZED))
    };

    if !db::group_member(pool.clone(), user_id, request.group_id).await? {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    Ok(Box::new(warp::reply::json(&Response {
        invite_id: db::create_invitation(pool.clone(), request.group_id).await?
    })))
}
