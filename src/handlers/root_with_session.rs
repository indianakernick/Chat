use askama::Template;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "../client/dist/with_session.html")]
struct RootTemplate {
    user_id: super::UserID,
}

pub async fn root_with_session(session_id: String, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(RootTemplate {
        user_id: super::get_session_user_id(pool, session_id).await?
    })
}
