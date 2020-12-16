use crate::database as db;
use deadpool_postgres::Pool;

pub async fn initialize_session(claims: super::Claims, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = db::user_from_google(pool.clone(), &claims).await?;
    let session_id = db::create_session(pool, user_id).await?;

    Ok(warp::reply::with_header(
        warp::redirect(claims.redirect.parse::<warp::http::Uri>().unwrap()),
        "Set-Cookie",
        format!("session_id={};Path=/;HttpOnly;Secure", session_id)
    ))
}
