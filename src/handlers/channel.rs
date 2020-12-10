use askama::Template;
use crate::error::Error;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "../client/dist/with_session.html")]
struct RootTemplate {
    user_id: super::UserID,
    channel_name: String
}

pub type ChannelID = i32;

async fn get_channel_name(pool: Pool, channel_id: ChannelID) -> Result<String, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name
        FROM Channel
        WHERE channel_id = $1
        LIMIT 1
    ").await?;
    match conn.query_opt(&stmt, &[&channel_id]).await? {
        Some(row) => Ok(row.get(0)),
        None => Err(Error::InvalidChannelID)
    }
}

pub async fn channel(channel_id: ChannelID, session_id: String, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let channel_name = match get_channel_name(pool.clone(), channel_id).await {
        Ok(name) => name,
        Err(Error::InvalidChannelID) => {
            return Ok(Box::new(
                warp::reply::with_status(warp::reply(), warp::http::StatusCode::NOT_FOUND)
            ));
        },
        Err(e) => return Err(e.into())
    };

    let user_id = match super::get_session_user_id(pool, session_id).await {
        Ok(id) => id,
        Err(Error::InvalidSessionID) => {
            return Ok(Box::new(
                warp::redirect(warp::http::Uri::from_static("/login"))
            ))
        },
        Err(e) => return Err(e.into())
    };

    Ok(Box::new(RootTemplate { user_id, channel_name }))
}
