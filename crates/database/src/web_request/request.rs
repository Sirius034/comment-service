use diesel::pg::PgConnection;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

pub type Filter = HashMap<String, FilterValue<serde_json::Value>>;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    _Eq,
    _Neq
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterValue<T> {
    _Eq(T),
    _Neq(T)
}

impl<T> FilterValue<T>  {    
    pub fn get_operator_and_value(self) -> (Operator, T) {
      match self {
          FilterValue::_Eq(value) => (Operator::_Eq, value),
          FilterValue::_Neq(value) => (Operator::_Neq, value),
      }
    } 
}

pub trait WebRequest {
    type ResultLoad;

    fn filter(self, filter: Filter) -> Self;

    fn sort(self) -> Self;

    fn init_sql_guery_from_json(self, json: Option<&str>) -> Self;

    fn load(self, connection: &mut PgConnection) -> Self::ResultLoad;
}
