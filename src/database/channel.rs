use super::GroupID;
use serde::Serialize;
use deadpool_postgres::{Pool, PoolError};

pub type ChannelID = i32;

#[derive(Serialize)]
pub struct Channel {
    pub channel_id: ChannelID,
    pub name: String,
}

/*
pub async fn channel_name(pool: Pool, channel_id: ChannelID) -> Result<Option<String>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name
        FROM Channel
        WHERE channel_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&channel_id]).await?.map(|row| row.get(0)))
}

pub async fn valid_channel(pool: Pool, channel_id: ChannelID) -> Result<bool, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT 1
        FROM Channel
        WHERE channel_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&channel_id]).await?.is_some())
}
*/

pub async fn create_channel(pool: Pool, group_id: GroupID, name: &String)
    -> Result<ChannelID, PoolError> {
    // TODO: Channel name should be unique within the group
    // TODO: Channel name should be strictly validated
    // Maybe don't allow whitespace
    // Also restrict the length
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Channel (name, group_id)
        VALUES ($1, $2)
        RETURNING channel_id
    ").await?;
    Ok(conn.query_one(&stmt, &[name, &group_id]).await?.get(0))
}
