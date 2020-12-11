use serde::Serialize;
use crate::error::Error;
use deadpool_postgres::Pool;

pub type UserID = i32;

#[derive(Serialize)]
pub struct UserInfo {
    pub name: String,
    pub picture: String
}

// TODO: Maybe add overloads that take database connections

pub async fn user_info(pool: Pool, user_id: UserID) -> Result<Option<UserInfo>, Error> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT name, picture
        FROM Usr
        WHERE user_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[&user_id]).await?.map(|row| {
        UserInfo {
            name: row.get(0),
            picture: row.get(1)
        }
    }))
}

// TODO: Maybe use a struct named GoogleUserInfo

pub async fn user_from_google(pool: Pool, claims: &crate::handlers::Claims) -> Result<UserID, Error> {
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
