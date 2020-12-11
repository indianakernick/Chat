use askama::Template;
use crate::error::Error;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "../client/dist/with_session.html")]
struct ChannelTemplate {
    user_id: super::UserID,
    channel_name: String
}

pub type ChannelID = i32;

// TODO: Maybe put these sorts of functions into their own database module
async fn get_channel_name(pool: Pool, channel_id: ChannelID) -> Result<Option<String>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name
        FROM Channel
        WHERE channel_id = $1
        LIMIT 1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&channel_id]).await?.map(|row| row.get(0)))
}

pub async fn channel(channel_id: ChannelID, session_id: String, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let channel_name = match get_channel_name(pool.clone(), channel_id).await? {
        Some(name) => name,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let user_id = match super::get_session_user_id(pool, session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(warp::redirect(
            format!("/login?redirect=/channel/{}", channel_id)
                .parse::<warp::http::Uri>().unwrap()
        )))
    };

    Ok(Box::new(ChannelTemplate { user_id, channel_name }))
}
