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
    match conn.query_opt(&stmt, &[&user_id]).await? {
        Some(row) => {
            Ok(UserInfo {
                name: row.get(0),
                picture: row.get(1)
            })
        },
        None => Err(Error::InvalidUserID)
    }
}

pub async fn user(user_id: UserID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let user_info = match get_user_info(user_id, pool).await {
        Ok(info) => info,
        Err(Error::InvalidUserID) => {
            return Ok(Box::new(
                warp::http::StatusCode::NOT_FOUND
            ));
        },
        Err(e) => return Err(e.into())
    };
    Ok(Box::new(
        warp::reply::with_header(
            warp::reply::json(&user_info),
            "Cache-Control",
            "public,max-age=86400,immutable" // 24 hours
        )
    ))
}
