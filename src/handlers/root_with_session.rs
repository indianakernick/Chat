use askama::Template;
use deadpool_postgres::Pool;
use super::get_session_user_info;

// TODO: Is this worth it?
// We'll probably still need /api/me in case the user changes their personal
// details.

#[derive(Template)]
#[template(path = "../client/dist/with_session.html")]
struct RootTemplate {
    info: String,
}

pub async fn root_with_session(session_id: String, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let info = get_session_user_info(pool, session_id).await?;
    Ok(RootTemplate {
        info: serde_json::to_string(&info).unwrap().replace("</script>", "<\\/script>")
    })
}
