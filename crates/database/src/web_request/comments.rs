use super::request::{Filter, Operator, WebRequest};
use crate::models::Comment;
use crate::schema::comments;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::result::Error as DieselResultError;
use serde::Deserialize;
use serde_json::{Error as SerdeJsonError, Value, from_str};
use std::error::Error;
use std::fmt;

macro_rules! apply_filter {
    ($query:expr, $col:ident, $operator:expr, $value:expr) => {
        match $operator {
            Operator::_Eq => $query.filter($col.eq($value)),
            Operator::_Neq => $query.filter($col.ne($value)),
        }
    };
}

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

#[derive(Deserialize, Debug)]
struct RequestParams {
    filter: Option<Filter>,
}

pub struct Comments {
    box_query: comments::BoxedQuery<'static, Pg>,
}

impl Comments {
    pub fn new() -> Self {
        Comments {
            box_query: comments::table.into_boxed(),
        }
    }
}

impl WebRequest for Comments {
    type ResultLoad = Result<Vec<Comment>, ErrorRequest>;
    type Error = ErrorRequest;

    fn filter(mut self, filter: Filter) -> Result<Self, Self::Error> {
        use crate::schema::comments::dsl::*;
        use log::warn;
        use uuid::Uuid;

        for (field_name, filter_value) in filter {
            let (operator, value) = filter_value.get_operator_and_value();

            self.box_query = match (field_name.as_str(), value) {
                ("id", Value::String(str)) => {
                    let uuid = Uuid::parse_str(str.as_str()).map_err(|error| {
                        ErrorRequest::InvalidUuid {
                            field: str,
                            source: error,
                        }
                    })?;
                    apply_filter!(self.box_query, id, operator, uuid)
                }
                ("pinned", Value::Bool(val)) => {
                    apply_filter!(self.box_query, pinned, operator, val)
                }
                ("comment", Value::String(str)) => {
                    apply_filter!(self.box_query, comment, operator, str)
                }
                ("page_id", Value::String(str)) => {
                    apply_filter!(self.box_query, page_id, operator, str)
                }
                ("user_name", Value::String(str)) => {
                    apply_filter!(self.box_query, user_name, operator, str)
                }
                ("user_id", Value::String(str)) => {
                    apply_filter!(self.box_query, user_id, operator, str)
                }
                (_, _) => {
                    warn!("Unknown filter field: {field_name}");
                    self.box_query
                }
            };
        }

        Ok(self)
    }

    fn sort(self) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn init_sql_query_from_json(self, json: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(json) = json {
            let request_params = from_str::<RequestParams>(json)
                .map_err(ErrorRequest::InvalidRequest)?;

            if let Some(filter) = request_params.filter {
                return self.filter(filter);
            }
        }

        Ok(self)
    }

    fn load(self, connection: &mut PgConnection) -> Self::ResultLoad {
        self.box_query.load::<Comment>(connection).map_err(ErrorRequest::Database)
    }
}
