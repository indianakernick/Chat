use super::GroupID;
use serde::Serialize;
use deadpool_postgres::{Pool, PoolError};

pub type ChannelID = i32;

#[derive(Serialize)]
pub struct Channel {
    pub channel_id: ChannelID,
    pub name: String,
}

/// Create a new channel.
///
/// Assumes that the group_id is valid (because verifying it would require an
/// additional query). Also assumes that the name is valid
///
/// Returns Ok(None) if the channel name is not unique
pub async fn create_channel(pool: Pool, group_id: GroupID, name: &String)
    -> Result<Option<ChannelID>, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Channel (name, group_id)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT *
            FROM Channel
            WHERE name = $1
            AND group_id = $2
        )
        RETURNING channel_id
    ").await?;
    Ok(conn.query_opt(&stmt, &[name, &group_id]).await?.map(|row| row.get(0)))
}

/// Delete a channel.
///
/// Returns true if the channel was actually deleted.
pub async fn delete_channel(pool: Pool, channel_id: ChannelID)
    -> Result<bool, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        DELETE FROM Channel
        WHERE channel_id = $1
    ").await?;
    Ok(conn.execute(&stmt, &[&channel_id]).await? > 0)
}

/// Rename a channel.
///
/// Returns true if the channel was actually renamed.
pub async fn rename_channel(pool: Pool, group_id: GroupID, channel_id: ChannelID, name: &String)
    -> Result<bool, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        UPDATE Channel
        SET name = $3
        WHERE channel_id = $2
        AND NOT EXISTS (
            SELECT *
            FROM Channel
            WHERE name = $3
            AND group_id = $1
        )
    ").await?;
    Ok(conn.execute(&stmt, &[&group_id, &channel_id, name]).await? > 0)
}
