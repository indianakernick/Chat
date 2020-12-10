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

fn redirect_static(location: &'static str) -> impl warp::Reply {
    warp::redirect(warp::http::Uri::from_static(location))
}

pub async fn channel(channel_id: ChannelID, session_id: String, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let channel_name = match get_channel_name(pool.clone(), channel_id).await? {
        Some(name) => name,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let user_id = match super::get_session_user_id(pool, session_id).await? {
        Some(id) => id,
        None => return Ok(Box::new(redirect_static("/login")))
    };

    Ok(Box::new(RootTemplate { user_id, channel_name }))
}
