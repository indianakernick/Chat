use rand::Rng;
use deadpool_postgres::Pool;

const SESSION_ID_LENGTH: usize = 16;

/// Generates a 16 character, [base64url][1] encoded, cryptographically secure,
/// random string.
///
/// [1]: https://tools.ietf.org/html/rfc4648#page-7
fn generate_session_id() -> String {
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

async fn initialize_session(pool: Pool) -> Result<String, deadpool_postgres::PoolError> {
    let mut session_id = generate_session_id();
    let conn = pool.get().await?;

    // TODO: Consider using https://github.com/dtolnay/indoc
    let stmt = conn.prepare("
         INSERT INTO Session (session_id)
         VALUES ($1)
         ON CONFLICT (session_id) DO NOTHING
    ").await?;

    while conn.execute(&stmt, &[&session_id]).await? == 0 {
        session_id = generate_session_id();
    }

    Ok(session_id)
}

pub async fn create_session(pool: Pool, claims: super::Claims) -> Result<impl warp::Reply, warp::Rejection> {
    let session_id = match initialize_session(pool).await {
        Ok(s) => s,
        Err(_) => return Err(warp::reject()) // TODO: Use warp::reject::custom
    };

    Ok(
        warp::reply::with_header(
            warp::redirect(warp::http::Uri::from_static("/")),
            "Set-Cookie",
            format!("session_id={}; HttpOnly; Secure", session_id)
        )
    )
}
