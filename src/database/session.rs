use rand::Rng;
use crate::error::Error;
use deadpool_postgres::Pool;

pub const SESSION_ID_LENGTH: usize = 16;

pub type SessionID = String;
// TODO: Maybe make the session ID a byte array
// pub type SessionID = [u8; SESSION_ID_LENGTH];

/// Generates a [base64url][1] encoded, cryptographically secure,
/// random string.
///
/// [1]: https://tools.ietf.org/html/rfc4648#page-7
fn generate_session_id() -> SessionID {
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

pub async fn create_session(pool: Pool, user_id: super::UserID) -> Result<SessionID, Error> {
    let mut session_id = generate_session_id();

    let conn = pool.get().await?;
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

pub async fn session_user_id(pool: Pool, session_id: SessionID) -> Result<Option<super::UserID>, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT user_id
        FROM Session
        WHERE session_id = $1
        AND creation_time > NOW() - INTERVAL '7 days'
    ").await?;

    Ok(conn.query_opt(&stmt, &[&session_id]).await?.map(|row| row.get(0)))
}

use serde::Serialize;

#[derive(Serialize)]
pub struct SessionInfo {
    pub user_id: super::UserID,
    pub name: String,
    pub picture: String
}

pub async fn session_info(pool: Pool, session_id: SessionID) -> Result<Option<SessionInfo>, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    // TODO: maybe use concat! to define INTERVAL '7 days' in one place
    let stmt = conn.prepare("
        SELECT Usr.user_id, name, picture
        FROM Usr
        JOIN Session ON Session.user_id = Usr.user_id
        WHERE session_id = $1
        AND creation_time > NOW() - INTERVAL '7 days'
    ").await?;

    Ok(conn.query_opt(&stmt, &[&session_id]).await?.map(|row| {
        SessionInfo {
            user_id: row.get(0),
            name: row.get(1),
            picture: row.get(2)
        }
    }))
}