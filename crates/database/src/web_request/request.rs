use diesel::pg::PgConnection;
use serde::Deserialize;
use serde_json;
use std::{collections::HashMap};

pub type Filter = HashMap<String, FilterValue<serde_json::Value>>;
pub type Sort = Vec<String>;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    _Eq,
    _Neq,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterValue<T> {
    _Eq(T),
    _Neq(T),
}

impl<T> FilterValue<T> {
    pub fn get_operator_and_value(self) -> (Operator, T) {
        match self {
            FilterValue::_Eq(value) => (Operator::_Eq, value),
            FilterValue::_Neq(value) => (Operator::_Neq, value),
        }
    }
}

pub trait WebRequest
where
    Self: Sized,
{
    type ResultLoad;
    type Error;

    fn filter(self, filter: Filter) -> Result<Self, Self::Error>;

    fn sort(self, sort_list: Sort) -> Result<Self, Self::Error>;

    fn init_sql_query_from_json(self, json: Option<&str>) -> Result<Self, Self::Error>;

    fn load(self, connection: &mut PgConnection) -> Self::ResultLoad;
}
