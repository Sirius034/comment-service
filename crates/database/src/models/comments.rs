use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub data_created: NaiveDateTime,
    pub type_: i32,
    pub comment: Option<String>,
    pub page_id: Option<String>,
    pub user_name: Option<String>,
    pub user_id: Option<String>,
    pub client_id: Option<String>,
    pub pinned: bool,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
  pub type_: i32,
  pub comment: Option<String>,
  pub page_id: Option<String>,
  pub user_name: Option<String>,
  pub user_id: Option<String>,
  pub client_id: Option<String>,
}
