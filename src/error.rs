// TODO: Maybe use newtype structs and implement Reject for each other them

pub type DatabaseError = deadpool_postgres::PoolError;
pub type RequestError = reqwest::Error;
pub type JWTError = jsonwebtoken::errors::Error;
pub type HeaderError = headers::Error;
pub type JSONError = serde_json::error::Error;

#[derive(Debug)]
pub enum Error {
    Database(DatabaseError),
    Request(RequestError),
    JWT(JWTError),
    Header(HeaderError),
    JSON(JSONError),

    // TODO: Do these belong here?
    // Is there a better way to handle these conditions?
    // Maybe Result<Option<T>, Error> ?
    // Functions have the possibility of returning an invalid session ID error
    // even if they don't deal with session IDs.
    InvalidSessionID,
    InvalidChannelID,
    InvalidUserID
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Database(e) => e.fmt(f),
            Error::Request(e) => e.fmt(f),
            Error::JWT(e) => e.fmt(f),
            Error::Header(e) => e.fmt(f),
            Error::JSON(e) => e.fmt(f),
            Error::InvalidSessionID => f.write_str("Invalid session ID"),
            Error::InvalidChannelID => f.write_str("Invalid channel ID"),
            Error::InvalidUserID => f.write_str("Invalid user ID")
        }
    }
}

impl std::error::Error for Error {}

impl warp::reject::Reject for Error {}

impl From<Error> for warp::Rejection {
    fn from(e: Error) -> warp::Rejection {
        warp::reject::custom(e)
    }
}

impl From<deadpool_postgres::tokio_postgres::Error> for Error {
    fn from(e: deadpool_postgres::tokio_postgres::Error) -> Error {
        Error::Database(DatabaseError::Backend(e))
    }
}

impl From<DatabaseError> for Error {
    fn from(e: DatabaseError) -> Error {
        Error::Database(e)
    }
}

impl From<RequestError> for Error {
    fn from(e: RequestError) -> Error {
        Error::Request(e)
    }
}

impl From<JWTError> for Error {
    fn from(e: JWTError) -> Error {
        Error::JWT(e)
    }
}

impl From<HeaderError> for Error {
    fn from(e: HeaderError) -> Error {
        Error::Header(e)
    }
}

impl From<JSONError> for Error {
    fn from(e: JSONError) -> Error {
        Error::JSON(e)
    }
}
