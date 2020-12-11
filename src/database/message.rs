use deadpool_postgres::{Pool, PoolError};
use deadpool_postgres::tokio_postgres::Row;

// Currently not needed
// pub type MessageID = i32;

pub async fn recent_messages(pool: Pool, channel_id: super::ChannelID) -> Result<Vec<Row>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT timestamp, COALESCE(author, 0), content
        FROM Message
        WHERE channel_id = $1
    ").await?;
    conn.query(&stmt, &[&channel_id]).await.map_err(|e| e.into())
}

// TODO: Maybe use a struct here
pub async fn create_message(
    pool: Pool,
    time: std::time::SystemTime,
    user_id: super::UserID,
    content: String,
    channel_id: super::ChannelID
) -> Result<(), PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Message (timestamp, author, content, channel_id)
        VALUES ($1, $2, $3, $4)
    ").await?;
    conn.execute(&stmt, &[&time, &user_id, &content, &channel_id]).await?;
    Ok(())
}
