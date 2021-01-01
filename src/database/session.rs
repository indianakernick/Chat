use crate::error::Error;
use super::{User, UserID};
use deadpool_postgres::Pool;
use crate::utils::generate_random_base64url;

// This value is duplicated in the column type Session.session_id
pub const SESSION_ID_LENGTH: usize = 16;

pub type SessionID = String;

macro_rules! creation_timeout {
    () => { "INTERVAL '7 days'" }
}

pub async fn create_session(pool: Pool, user_id: UserID)
    -> Result<SessionID, Error>
{
    // This function is nearly identical to create_invitation
    let mut session_id = generate_random_base64url(SESSION_ID_LENGTH);

    let conn = pool.get().await?;
    let stmt = conn.prepare("
         INSERT INTO Session (session_id, creation_time, user_id)
         VALUES ($1, NOW(), $2)
         ON CONFLICT (session_id) DO NOTHING
    ").await?;

    while conn.execute(&stmt, &[&session_id, &user_id]).await? == 0 {
        session_id = generate_random_base64url(SESSION_ID_LENGTH);
    }

    Ok(session_id)
}

pub async fn session_user_id(pool: Pool, session_id: &SessionID)
    -> Result<Option<UserID>, Error>
{
    // This function is nearly identical to invitation_group_id
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

    Ok(conn.query_opt(&stmt, &[session_id]).await?.map(|row| row.get(0)))
}

pub async fn session_user(pool: Pool, session_id: &SessionID)
    -> Result<Option<User>, Error>
{
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

    Ok(conn.query_opt(&stmt, &[session_id]).await?.map(|row| {
        User {
            user_id: row.get(0),
            name: row.get(1),
            picture: row.get(2)
        }
    }))
}
