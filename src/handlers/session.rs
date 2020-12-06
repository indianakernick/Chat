use rand::Rng;
use deadpool_postgres::Pool;

/// Generates a 16 character, [base64url][1] encoded, cryptographically secure,
/// random string.
///
/// [1]: https://tools.ietf.org/html/rfc4648#page-7
fn generate_session_id() -> String {
    const SESSION_ID_LENGTH: usize = 16;

    let mut rng = rand::thread_rng();
    let mut bytes = vec![0; SESSION_ID_LENGTH];

    for i in 0..SESSION_ID_LENGTH {
        let num: u8 = rng.gen_range(0, 64);
        if num < 26 {
            bytes[i] = b'A' + num;
        } else if num < 2 * 26 {
            bytes[i] = b'a' + num - 26;
        } else if num < 2 * 26 + 10 {
            bytes[i] = b'0' + num - 2 * 26;
        } else if num == 2 * 26 + 10 {
            bytes[i] = b'-';
        } else {
            bytes[i] = b'_';
        }
    }

    unsafe {
        return String::from_utf8_unchecked(bytes);
    }
}

async fn initialize_session(pool: Pool, session_id: &String) -> Result<(), deadpool_postgres::PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("INSERT INTO Session (session_id) VALUES ($1)").await?;
    conn.query_opt(&stmt, &[session_id]).await?;
    Ok(())
}

pub async fn create_session(pool: Pool, claims: super::Claims) -> Result<impl warp::Reply, warp::Rejection> {
    let session_id = generate_session_id();
    if let Err(e) = initialize_session(pool, &session_id).await {
        return Err(warp::reject()); // TODO: Use warp::reject::custom
    }

    Ok(
        warp::reply::with_header(
            warp::redirect(warp::http::Uri::from_static("/")),
            "Set-Cookie",
            format!("session_id={}; HttpOnly; Secure", session_id)
        )
    )
}
