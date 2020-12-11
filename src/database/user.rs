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
    // TODO: I really don't like this. Should do it in one statement.
    let conn = pool.get().await?;
    let login_stmt = conn.prepare("
        SELECT user_id
        FROM Usr
        WHERE google_id = $1
        LIMIT 1
    ").await?;
    let signup_stmt = conn.prepare("
        INSERT INTO Usr (name, picture, google_id)
        VALUES ($1, $2, $3)
        RETURNING user_id
    ").await?;

    if let Some(user_id) = conn.query_opt(&login_stmt, &[&claims.sub]).await? {
        return Ok(user_id.get(0));
    }

    Ok(conn.query_one(&signup_stmt, &[&claims.name, &claims.picture, &claims.sub]).await?.get(0))
}
