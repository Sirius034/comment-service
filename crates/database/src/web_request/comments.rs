use super::ErrorRequest;
use super::request::{Filter, Operator, Sort, WebRequest};
use crate::models::Comment;
use crate::schema::comments;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::{Value, from_str};

macro_rules! apply_filter {
    ($query:expr, $col:ident, $operator:expr, $value:expr) => {
        match $operator {
            Operator::_Eq => $query.filter($col.eq($value)),
            Operator::_Neq => $query.filter($col.ne($value)),
        }
    };
}

macro_rules! apply_sort {
    ($query:expr, $col:ident, $is_sort_desc:expr) => {
        if $is_sort_desc {
            $query.order_by($col.desc())
        } else {
            $query.order_by($col.asc())
        }
    };
}

#[derive(Deserialize, Debug)]
struct RequestParams {
    filter: Option<Filter>,
    sort: Option<Sort>,
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

    fn sort(mut self, sort_list: Sort) -> Result<Self, Self::Error> {
        use crate::schema::comments::dsl::*;

        for sort in sort_list {
            let is_sord_desc = sort.starts_with("-");
            let col_name = if is_sord_desc {
                sort.get(1..)
            } else {
                sort.get(0..)
            };

            self.box_query = match col_name {
                Some("data_created") => apply_sort!(self.box_query, data_created, is_sord_desc),
                Some("pinned") => apply_sort!(self.box_query, data_created, is_sord_desc),
                _ => self.box_query,
            };
        }

        Ok(self)
    }

    fn init_sql_query_from_json(mut self, json: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(json) = json {
            let request_params =
                from_str::<RequestParams>(json).map_err(ErrorRequest::InvalidRequest)?;

            if let Some(filter) = request_params.filter {
                self = self.filter(filter)?;
            }

            if let Some(sort_list) = request_params.sort {
                self = self.sort(sort_list)?;
            }
        }

        Ok(self)
    }

    fn load(self, connection: &mut PgConnection) -> Self::ResultLoad {
        self.box_query
            .load::<Comment>(connection)
            .map_err(ErrorRequest::Database)
    }
}
