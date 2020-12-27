use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;
use super::{Channel, UserID};

pub type GroupID = i32;

/// Create a new group.
///
/// Returns Ok(None) if the name is not unique.
/// Returns Err if a database error occurred.
pub async fn create_group(pool: Pool, name: String, picture: String)
    -> Result<Option<GroupID>, Error>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Groop (name, picture)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT *
            FROM Groop
            WHERE name = $1
        )
        RETURNING group_id
    ").await?;
    Ok(conn.query_opt(&stmt, &[&name, &picture]).await?.map(|row| row.get(0)))
}

/// Get the channels in a group
///
/// Returns an empty vector if the group is invalid (or if there are no channels
/// in a valid group)
pub async fn group_channels(pool: Pool, group_id: GroupID)
    -> Result<Vec<Channel>, Error>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT channel_id, name
        FROM Channel
        WHERE group_id = $1
    ").await?;
    Ok(conn.query(&stmt, &[&group_id])
        .await?
        .iter()
        .map(|row| Channel {
            channel_id: row.get(0),
            name: row.get(1),
        })
        .collect())
}

/// Check whether a group ID is valid
pub async fn valid_group(pool: Pool, group_id: GroupID) -> Result<bool, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT 1
        FROM Groop
        WHERE group_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&group_id]).await?.is_some())
}

#[derive(Serialize)]
pub struct Group {
    pub group_id: GroupID,
    pub name: String,
    pub picture: String,
}

/// Get the list of groups that a user is a member of.
pub async fn group_list(pool: Pool, user_id: UserID) -> Result<Vec<Group>, Error> {
    // Group membership isn't currently stored in the database
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT group_id, name, COALESCE(picture, '')
        FROM Groop
    ").await?;
    Ok(conn.query(&stmt, &[]).await?.iter().map(|row| Group {
        group_id: row.get(0),
        name: row.get(1),
        picture: row.get(2),
    }).collect())
}
