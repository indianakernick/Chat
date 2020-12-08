use rand::Rng;
use serde::Serialize;
use deadpool_postgres::Pool;
use deadpool_postgres::Client;
use crate::error::Error;

const SESSION_ID_LENGTH: usize = 16;

pub type UserID = i32;

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

async fn signup_or_login(conn: &Client, claims: super::Claims) -> Result<UserID, Error> {
    // TODO: I really don't like this. Should do it in one statement.
    let login_stmt = conn.prepare("
        SELECT user_id
        FROM Usr
        WHERE google_id = $1
        LIMIT 1
    ").await?;
    let signup_stmt = conn.prepare("
        INSERT INTO Usr (name, picture, google_id)
        VALUES ($1, $2, $3)
        RETURNING user_id
    ").await?;

    if let Some(user_id) = conn.query_opt(&login_stmt, &[&claims.sub]).await? {
        return Ok(user_id.get(0));
    }

    Ok(conn.query_one(&signup_stmt, &[&claims.name, &claims.picture, &claims.sub]).await?.get(0))
}

async fn initialize_session(conn: &Client, user_id: UserID) -> Result<String, Error> {
    let mut session_id = generate_session_id();

    // TODO: Consider using https://github.com/dtolnay/indoc
    let stmt = conn.prepare("
         INSERT INTO Session (session_id, creation_time, user_id)
         VALUES ($1, NOW(), $2)
         ON CONFLICT (session_id) DO NOTHING
    ").await?;

    while conn.execute(&stmt, &[&session_id, &user_id]).await? == 0 {
        session_id = generate_session_id();
    }

    Ok(session_id)
}

pub async fn create_session(claims: super::Claims, pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => return Err(Error::Database(e).into())
    };
    let user_id = signup_or_login(&conn, claims).await?;
    let session_id = initialize_session(&conn, user_id).await?;

    Ok(
        warp::reply::with_header(
            warp::redirect(warp::http::Uri::from_static("/")),
            "Set-Cookie",
            format!("session_id={}; HttpOnly; Secure", session_id)
        )
    )
}

pub async fn get_session_user_id(pool: Pool, session_id: String) -> Result<UserID, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Err(Error::InvalidSessionID);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT user_id
        FROM Session
        WHERE session_id = $1
        AND creation_time > NOW() - INTERVAL '7 days'
        LIMIT 1
    ").await?;

    match conn.query_opt(&stmt, &[&session_id]).await? {
        Some(row) => Ok(row.get(0)),
        None => Err(Error::InvalidSessionID)
    }
}

#[derive(Serialize)]
pub struct UserInfo {
    name: String,
    picture: String,
}

pub async fn get_session_user_info(pool: Pool, session_id: String) -> Result<UserInfo, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Err(Error::InvalidSessionID);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT Usr.name, Usr.picture
        FROM Usr
        JOIN Session ON Session.user_id = Usr.user_id
        WHERE Session.session_id = $1
        AND Session.creation_time > NOW() - INTERVAL '7 days'
        LIMIT 1
    ").await?;

    match conn.query_opt(&stmt, &[&session_id]).await? {
        Some(row) => Ok(UserInfo {
            name: row.get(0),
            picture: row.get(1)
        }),
        None => Err(Error::InvalidSessionID)
    }
}
