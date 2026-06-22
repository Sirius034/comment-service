use diesel::r2d2::PoolError as R2D2PoolError;
use diesel::result::Error as DieselResultError;
use serde_json::Error as SerdeJsonError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    InvalidUuid { field: String, source: uuid::Error },
    InvalidRequest(SerdeJsonError),
    Database(DieselResultError),
    Pool(R2D2PoolError),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidUuid { field, source } => write!(
                f,
                "Invalid value uuid in the field {field}. Source: {source}"
            ),
            Errors::InvalidRequest(error) => write!(f, "Invalid request: {error}"),
            Errors::Database(error) => write!(f, "Database error: {error}"),
            Errors::Pool(error) => write!(f, "Pool error: {error}"),
        }
    }
}

impl Error for Errors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Errors::InvalidUuid { source, .. } => Some(source),
            Errors::InvalidRequest(error) => Some(error),
            Errors::Database(error) => Some(error),
            Errors::Pool(error) => Some(error),
        }
    }
}
