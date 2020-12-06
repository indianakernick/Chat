use serde::Serialize;
use deadpool_postgres::Pool;
use std::convert::Infallible;

// TODO: Handle errors properly.
// If something goes wrong, give the client a vague message.
// Then write a detailed message to a log.

// try_string! is not the right way of dealing with errors.

// Define one error type and use it every where perhaps?

// What does a PoolError really mean?
//   PoolError is either a timeout or a database error.
//   For timeout, tell the user that the server is experiencing heavy traffic
//   For database, tell the user that a database error has occurred
//   Should also leave detailed logs in both cases.

#[derive(Serialize, Default)]
struct UserInfo {
    picture: String,
    given_name: String,
    family_name: String,
}

async fn get_user_info(pool: Pool, session_id: &String) -> Result<UserInfo, deadpool_postgres::PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("SELECT session_id FROM Session WHERE session_id = $1").await?;
    let row = conn.query_one(&stmt, &[session_id]).await?;
    Ok(UserInfo::default())
}

pub async fn me(pool: Pool, session_id: String) -> Result<impl warp::Reply, Infallible> {
    let user_info = match get_user_info(pool, &session_id).await {
        Ok(u) => u,
        Err(e) => return Ok(e.to_string())
    };

    Ok(String::from("Success"))
}
