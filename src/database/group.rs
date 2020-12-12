use crate::error::Error;
use deadpool_postgres::Pool;
use deadpool_postgres::tokio_postgres::Row;

pub type GroupID = i32;

/// Create a new group.
///
/// Returns Ok(None) if the name is not unique.
/// Returns Err if a database error occurred.
pub async fn create_group(pool: Pool, name: String)
    -> Result<Option<GroupID>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO Groop (name)
        SELECT $1
        WHERE NOT EXISTS (SELECT * FROM Groop WHERE name = $1)
        RETURNING group_id
    ").await?;
    Ok(conn.query_opt(&stmt, &[&name]).await?.map(|row| row.get(0)))
}

pub struct GroupInfo {
    pub name: String,
    pub picture: String
}

/// Get information about a group
///
/// Returns Ok(None) if the group_id is invalid.
/// Returns Err if a database error occurred
pub async fn group_info(pool: Pool, group_id: GroupID)
    -> Result<Option<GroupInfo>, Error> {
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
pub async fn group_channels(pool: Pool, group_id: GroupID) -> Result<Vec<Row>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT channel_id, name
        FROM Channel
        WHERE group_id = $1
    ").await?;
    conn.query(&stmt, &[&group_id]).await.map_err(|e| e.into())
}
