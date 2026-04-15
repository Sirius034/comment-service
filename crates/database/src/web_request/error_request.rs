use diesel::result::Error as DieselResultError;
use serde_json::Error as SerdeJsonError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorRequest {
    InvalidUuid { field: String, source: uuid::Error },
    InvalidRequest(SerdeJsonError),
    Database(DieselResultError),
}

impl fmt::Display for ErrorRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorRequest::InvalidUuid { field, source } => {
                write!(
                    f,
                    "Invalid value uuid in the field {}. Source: {}",
                    field, source
                )
            }
            ErrorRequest::InvalidRequest(error) => {
                write!(f, "Invalid request: {}", error)
            }
            ErrorRequest::Database(error) => {
                write!(f, "Database error: {}", error)
            }
        }
    }
}

impl Error for ErrorRequest {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorRequest::InvalidUuid { source, .. } => Some(source),
            ErrorRequest::InvalidRequest(error) => Some(error),
            ErrorRequest::Database(error) => Some(error),
        }
    }
}
