use super::GroupID;
use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::{Pool, PoolError};

pub type UserID = i32;

#[derive(Serialize)]
pub struct User {
    pub user_id: UserID,
    pub name: String,
    pub picture: String,
}

#[derive(Serialize)]
pub struct AnonUser {
    pub name: String,
    pub picture: String,
}

pub struct GoogleUser {
    pub google_id: String,
    pub name: String,
    pub picture: String,
}

pub async fn user(pool: Pool, user_id: UserID) -> Result<Option<AnonUser>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name, picture
        FROM Usr
        WHERE user_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&user_id]).await?.map(|row| {
        AnonUser {
            name: row.get(0),
            picture: row.get(1)
        }
    }))
}

pub async fn user_id_from_google(pool: Pool, user: &GoogleUser) -> Result<UserID, Error> {
    let conn = pool.get().await?;
    // https://stackoverflow.com/a/6722460/4093378
    let stmt = conn.prepare("
        WITH Temp AS (
            INSERT INTO Usr (google_id, name, picture)
            SELECT $1, $2, $3
            WHERE NOT EXISTS (SELECT * FROM Usr WHERE google_id = $1)
            RETURNING user_id
        )
        SELECT user_id FROM Temp
        UNION ALL
        SELECT user_id FROM Usr WHERE google_id = $1
        LIMIT 1
    ").await?;
    Ok(conn.query_one(&stmt, &[&user.google_id, &user.name, &user.picture]).await?.get(0))
}

pub async fn group_users(pool: Pool, group_id: GroupID) -> Result<Vec<User>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT Usr.user_id, name, picture
        FROM Usr
        JOIN Membership ON Membership.user_id = Usr.user_id
        WHERE Membership.group_id = $1
        ORDER BY Usr.user_id
    ").await?;
    Ok(conn.query(&stmt, &[&group_id]).await?.iter().map(|row| User {
        user_id: row.get(0),
        name: row.get(1),
        picture: row.get(2),
    }).collect())
}

pub async fn group_user_ids(pool: Pool, group_id: GroupID) -> Result<Vec<UserID>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT Usr.user_id
        FROM Usr
        JOIN Membership ON Membership.user_id = Usr.user_id
        WHERE Membership.group_id = $1
        ORDER BY Usr.user_id
    ").await?;
    Ok(conn.query(&stmt, &[&group_id]).await?.iter().map(|row| row.get(0)).collect())
}

pub async fn rename_user(pool: Pool, user_id: UserID, name: &String, picture: &String) -> Result<bool, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        UPDATE Usr
        SET name = $2, picture = $3
        WHERE user_id = $1
        AND NOT EXISTS (
            SELECT 1
            FROM Usr
            WHERE name = $2
            AND user_id != $1
        )
    ").await?;
    Ok(conn.execute(&stmt, &[&user_id, name, picture]).await? > 0)
}

pub async fn delete_user(pool: Pool, user_id: UserID) -> Result<bool, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        DELETE FROM Usr
        WHERE user_id = $1
    ").await?;
    Ok(conn.execute(&stmt, &[&user_id]).await? > 0)
}

pub async fn anonymize_messages(pool: Pool, user_id: UserID, group_id: GroupID) -> Result<bool, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        UPDATE Message
        SET author = NULL
        WHERE author = $1
        AND channel_id IN (
            SELECT channel_id
            FROM Channel
            WHERE group_id = $2
        )
    ").await?;
    Ok(conn.execute(&stmt, &[&user_id, &group_id]).await? > 0)
}
