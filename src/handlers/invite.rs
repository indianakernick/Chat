use crate::database as db;
use deadpool_postgres::Pool;

pub async fn accept_invite(invite_id: db::InviteID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let user_id = match db::session_user_id(pool.clone(), session_id).await? {
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

    // TODO: group_channels returns much more than we need but I think there's a
    // better way to deal with this anyway. Perhaps make the channel ID for the
    // channel endpoint optional.
    let channel_id = db::group_channels(pool.clone(), group_id).await?[0].channel_id;

    Ok(Box::new(warp::redirect(
        format!("/channel/{}/{}", group_id, channel_id)
            .parse::<warp::http::Uri>().unwrap()
    )))
}
