use super::UserID;
use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;

#[derive(Serialize)]
struct UserInfo {
    name: String,
    picture: String,
}

async fn get_user_info(user_id: UserID, pool: Pool) -> Result<UserInfo, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name, picture
        FROM Usr
        WHERE user_id = $1
        LIMIT 1
    ").await?;
    let row = conn.query_one(&stmt, &[&user_id]).await?;
    Ok(UserInfo {
        name: row.get(0),
        picture: row.get(1)
    })
}

pub async fn user(user_id: UserID, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(
        warp::reply::with_header(
            warp::reply::json(&get_user_info(user_id, pool).await?),
            "Cache-Control",
            "public;max-age=86400" // 24 hours
        )
    )
}
