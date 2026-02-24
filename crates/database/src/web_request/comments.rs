use super::request::{Filter, Operator, WebRequest};
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

#[derive(Deserialize, Debug)]
struct RequestParams {
    filter: Option<Filter>,
}

pub struct Comments<'a> {
    box_query: comments::BoxedQuery<'a, Pg>,
}

impl<'a> Comments<'a> {
    pub fn new() -> Self {
        Comments {
            box_query: comments::table.into_boxed(),
        }
    }
}

impl<'a> WebRequest for Comments<'a> {
    type ResultLoad = Result<Vec<Comment>, diesel::result::Error>;

    fn sort(self) -> Self {
        self
    }

    fn filter(mut self, filter: Filter) -> Self {
        use crate::schema::comments::dsl::*;
        use uuid::Uuid;

        for (field_name, filter_value) in filter {
            let (operator, value) = filter_value.get_operator_and_value();

            self.box_query = match (field_name.as_str(), value) {
                ("id", Value::String(str)) => {
                    let uuid = Uuid::parse_str(str.as_str()).unwrap();
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
                (_, _) => self.box_query,
            };
        }

        self
    }

    fn init_sql_guery_from_json(self, json: Option<&str>) -> Self {
        if let Some(json) = json {
            let request_params = from_str::<RequestParams>(json).unwrap();

            if let Some(filter) = request_params.filter {
                return self.filter(filter);
            }
        }

        self
    }

    fn load(self, connection: &mut PgConnection) -> Self::ResultLoad {
        self.box_query.load(connection)
    }
}
