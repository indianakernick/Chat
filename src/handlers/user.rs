use super::UserID;
use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;

#[derive(Serialize)]
struct UserInfo {
    name: String,
    picture: String,
}

async fn get_user_info(user_id: UserID, pool: Pool) -> Result<Option<UserInfo>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name, picture
        FROM Usr
        WHERE user_id = $1
        LIMIT 1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&user_id]).await?.map(|row| {
        UserInfo {
            name: row.get(0),
            picture: row.get(1)
        }
    }))
}

pub async fn user(user_id: UserID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let user_info = match get_user_info(user_id, pool).await? {
        Some(info) => info,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    Ok(Box::new(
        warp::reply::with_header(
            warp::reply::json(&user_info),
            "Cache-Control",
            "public,max-age=86400,immutable" // 24 hours
        )
    ))
}
