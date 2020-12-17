use super::Channel;
use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;

pub type GroupID = i32;

/// Create a new group.
///
/// Returns Ok(None) if the name is not unique.
/// Returns Err if a database error occurred.
/*
pub async fn create_group(pool: Pool, name: String)
    -> Result<Option<GroupID>, Error>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Groop (name)
        SELECT $1
        WHERE NOT EXISTS (SELECT * FROM Groop WHERE name = $1)
        RETURNING group_id
    ").await?;
    Ok(conn.query_opt(&stmt, &[&name]).await?.map(|row| row.get(0)))
}
*/

#[derive(Serialize)]
pub struct GroupInfo {
    pub name: String,
    pub picture: String,
}

/// Get information about a group
///
/// Returns Ok(None) if the group_id is invalid.
/// Returns Err if a database error occurred
pub async fn group_info(pool: Pool, group_id: GroupID)
    -> Result<Option<GroupInfo>, Error>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name, COALESCE(picture, '')
        FROM Groop
        WHERE group_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&group_id]).await?.map(|row| {
        GroupInfo {
            name: row.get(0),
            picture: row.get(1)
        }
    }))
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

/// Get the vector of ChannelIDs in a group.
///
/// Returns an empty vector if the group is invalid (or if there are no channels
/// in a valid group).
/*pub async fn group_channel_ids(pool: Pool, group_id: GroupID)
    -> Result<Vec<ChannelID>, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT channel_id
        FROM Channel
        WHERE group_id = $1
    ").await?;
    Ok(conn.query(&stmt, &[&group_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect())
}*/

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

/*
pub async fn valid_group_channel(pool: Pool, group_id: GroupID, channel_id: ChannelID)
    -> Result<bool, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT 1
        FROM Channel
        WHERE group_id = $1
        AND channel_id = $2
    ").await?;
    Ok(conn.query_opt(&stmt, &[&group_id, &channel_id]).await?.is_some())
}
*/
