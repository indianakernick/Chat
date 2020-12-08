use deadpool_postgres::Pool;
use super::get_session_user_info;

pub async fn me(session_id: String, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    match get_session_user_info(pool, session_id).await {
        Ok(info) => Ok(warp::reply::json(&info)),
        Err(crate::error::Error::InvalidSessionID) =>
            Ok(warp::reply::json(&serde_json::json!({}))),
        Err(e) => Err(e.into())
    }
}
