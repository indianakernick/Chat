use super::{ChannelID, UserID};
use deadpool_postgres::{Pool, PoolError};
use deadpool_postgres::tokio_postgres::Row;

// Currently not needed
// pub type MessageID = i32;

pub async fn recent_messages(pool: Pool, channel_id: ChannelID) -> Result<Vec<Row>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT timestamp, COALESCE(author, 0), content
        FROM (
            SELECT *
            FROM Message
            WHERE channel_id = $1
            ORDER BY message_id DESC
            LIMIT 50
        ) Temp
        ORDER BY message_id ASC
    ").await?;
    conn.query(&stmt, &[&channel_id]).await.map_err(|e| e.into())
}

pub async fn create_message(
    pool: Pool,
    time: std::time::SystemTime,
    user_id: UserID,
    content: String,
    channel_id: ChannelID
) -> Result<(), PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Message (timestamp, author, content, channel_id)
        VALUES ($1, $2, $3, $4)
    ").await?;
    conn.execute(&stmt, &[&time, &user_id, &content, &channel_id]).await?;
    Ok(())
}
