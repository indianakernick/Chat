use deadpool_postgres::Pool;
use crate::database::{create_session as db_create_session, user_from_google};

pub async fn create_session(claims: super::Claims, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = user_from_google(pool.clone(), &claims).await?;
    let session_id = db_create_session(pool.clone(), user_id).await?;

    Ok(warp::reply::with_header(
        warp::redirect(claims.redirect.parse::<warp::http::Uri>().unwrap()),
        "Set-Cookie",
        format!("session_id={};Path=/;HttpOnly;Secure", session_id)
    ))
}
