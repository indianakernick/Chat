use crate::error::Error;
use deadpool_postgres::Pool;

pub type ChannelID = i32;

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
