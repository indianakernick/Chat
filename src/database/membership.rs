use crate::error::Error;
use deadpool_postgres::Pool;
use super::{UserID, GroupID};
use crate::utils::generate_random_base64url;

// This value is duplicated in the column type of Invitation.invite_id
pub const INVITE_ID_LENGTH: usize = 16;

pub type InviteID = String;

macro_rules! creation_timeout {
  () => {"INTERVAL '24 hours'"}
}

pub async fn create_invitation(pool: Pool, group_id: GroupID)
    -> Result<InviteID, Error>
{
    // This function is nearly identical to create_session
    let mut invite_id = generate_random_base64url(INVITE_ID_LENGTH);

    let conn = pool.get().await?;
    let stmt = conn.prepare("
         INSERT INTO Invitation (invite_id, group_id, creation_time)
         VALUES ($1, $2, NOW())
         ON CONFLICT (invite_id) DO NOTHING
    ").await?;

    while conn.execute(&stmt, &[&invite_id, &group_id]).await? == 0 {
        invite_id = generate_random_base64url(INVITE_ID_LENGTH);
    }

    Ok(invite_id)
}

pub async fn invitation_group_id(pool: Pool, invite_id: InviteID)
    -> Result<Option<GroupID>, Error>
{
    // This function is nearly identical to session_user_id
    if invite_id.len() != INVITE_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT group_id
        FROM Invitation
        WHERE invite_id = $1
        AND creation_time > NOW() - ", creation_timeout!()
    )).await?;
    Ok(conn.query_opt(&stmt, &[&invite_id]).await?.map(|row| row.get(0)))
}

pub async fn join_group(pool: Pool, user_id: UserID, group_id: GroupID)
    -> Result<bool, Error>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Membership (user_id, group_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING;
    ").await?;
    Ok(conn.execute(&stmt, &[&user_id, &group_id]).await? > 0)
}
