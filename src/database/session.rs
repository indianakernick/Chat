use rand::Rng;
use super::UserID;
use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;

pub const SESSION_ID_LENGTH: usize = 16;

pub type SessionID = String;

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

macro_rules! creation_timeout {
  () => {"INTERVAL '7 days'"}
}

pub async fn create_session(pool: Pool, user_id: UserID) -> Result<SessionID, Error> {
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

pub async fn session_user_id(pool: Pool, session_id: SessionID) -> Result<Option<UserID>, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT user_id
        FROM Session
        WHERE session_id = $1
        AND creation_time > NOW() - ", creation_timeout!()
    )).await?;

    Ok(conn.query_opt(&stmt, &[&session_id]).await?.map(|row| row.get(0)))
}

pub async fn valid_session(pool: Pool, session_id: SessionID) -> Result<bool, Error> {
    // This would work...
    // Ok(session_user_id(pool, session_id).await?.is_some())
    // but it's a slightly less efficient query

    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(false);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT 1
        FROM Session
        WHERE session_id = $1
        AND creation_time > NOW() - ", creation_timeout!()
    )).await?;

    Ok(conn.query_opt(&stmt, &[&session_id]).await?.is_some())
}

#[derive(Serialize)]
pub struct SessionInfo {
    pub user_id: UserID,
    pub name: String,
    pub picture: String,
}

pub async fn session_info(pool: Pool, session_id: SessionID) -> Result<Option<SessionInfo>, Error> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT Usr.user_id, name, picture
        FROM Usr
        JOIN Session ON Session.user_id = Usr.user_id
        WHERE session_id = $1
        AND creation_time > NOW() - ", creation_timeout!()
    )).await?;

    Ok(conn.query_opt(&stmt, &[&session_id]).await?.map(|row| {
        SessionInfo {
            user_id: row.get(0),
            name: row.get(1),
            picture: row.get(2)
        }
    }))
}
