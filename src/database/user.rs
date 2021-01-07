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

// TODO: Maybe use a struct named GoogleUserInfo

pub async fn user_id_from_google(pool: Pool, claims: &crate::handlers::Claims) -> Result<UserID, Error> {
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
    Ok(conn.query_one(&stmt, &[&claims.sub, &claims.name, &claims.picture]).await?.get(0))
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
