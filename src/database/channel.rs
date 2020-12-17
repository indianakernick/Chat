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

const MAX_CHANNEL_NAME_LENGTH: usize = 32;

pub fn valid_channel_name(name: &String) -> bool {
    // A byte limit instead of a character limit is tempting...

    if name.is_empty() {
        return false;
    }

    let mut count = 0;

    for ch in name.chars() {
        count += 1;
        if count > MAX_CHANNEL_NAME_LENGTH {
            return false;
        }
        // Not 100% sure about this.
        // I'm certain that whitespace shouldn't be allowed.
        // This handles non-ascii alphanumeric characters but that might be a
        // little tricky to mirror on the JavaScript end.
        if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
            return false;
        }
    }

    return true;
}

/// Assumes that the group_id is valid (because verifying it would require an
/// additional query). Also assumes that the name is valid because it can be
/// checked with the above function.
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
