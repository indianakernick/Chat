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
    JSON(JSONError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Database(e) => e.fmt(f),
            Error::Request(e) => e.fmt(f),
            Error::JWT(e) => e.fmt(f),
            Error::Header(e) => e.fmt(f),
            Error::JSON(e) => e.fmt(f)
        }
    }
}

impl std::error::Error for Error {}

impl warp::reject::Reject for Error {}

// TODO: Converting this error to a rejection might not be the right move.
// Should instead convert these types of errors to replies.
// Maybe a try! macro that checks for Err and returns a 500 reply
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
