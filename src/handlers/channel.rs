use askama::Template;
use deadpool_postgres::Pool;
use crate::database::{UserID, ChannelID, channel_name, SessionID, session_user_id};

#[derive(Template)]
#[template(path = "../client/dist/channel.html")]
struct ChannelTemplate {
    user_id: UserID,
    channel_id: ChannelID,
    channel_name: String
}

pub async fn channel(channel_id: ChannelID, session_id: SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let channel_name = match channel_name(pool.clone(), channel_id).await? {
        Some(name) => name,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let user_id = match session_user_id(pool, session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/channel/{}", channel_id)
                .parse::<warp::http::Uri>().unwrap()
        )))
    };

    Ok(Box::new(ChannelTemplate {
        user_id, channel_id, channel_name
    }))
}
