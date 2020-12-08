use serde::Serialize;
use deadpool_postgres::Pool;
use super::get_session_user_info;

#[derive(Serialize)]
struct Login {
    error: String
}

pub async fn me(session_id: String, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    match get_session_user_info(pool, session_id).await {
        Ok(info) => Ok(warp::reply::json(&info)),
        Err(crate::error::Error::InvalidSessionID) =>
            Ok(warp::reply::json(&Login { error: String::from("Need to log in") })),
        Err(_) => Err(warp::reject())
    }
}
