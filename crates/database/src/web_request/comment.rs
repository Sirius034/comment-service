use crate::errors::Errors;
use crate::models::{Comment, NewComment};
use crate::schema::comments;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = comments)]
pub struct PayloadUpdateComment {
    pub comment: Option<String>,
    pub user_name: Option<String>,
}

fn convert_str_to_uuid(value: &str) -> Result<Uuid, Errors> {
    Uuid::parse_str(value).map_err(|error| Errors::InvalidUuid {
        field: "id".to_string(),
        source: error,
    })
}

pub fn create_comment(
    new_comment: NewComment,
    connection: &mut PgConnection,
) -> Result<Comment, Errors> {
    diesel::insert_into(comments::table)
        .values(new_comment)
        .returning(Comment::as_returning())
        .get_result(connection)
        .map_err(Errors::Database)
}

pub fn read_comment(id: &str, connection: &mut PgConnection) -> Result<Comment, Errors> {
    let uuid = convert_str_to_uuid(id)?;

    comments::table
        .find(uuid)
        .select(Comment::as_select())
        .first(connection)
        .map_err(Errors::Database)
}

pub fn remove_comment(id: &str, connection: &mut PgConnection) -> Result<Uuid, Errors> {
    let uuid = convert_str_to_uuid(id)?;
    let select_comment = comments::table.filter(comments::id.eq(uuid));

    diesel::delete(select_comment)
        .returning(comments::id)
        .get_result::<Uuid>(connection)
        .map_err(Errors::Database)
}

pub fn update_comment(
    id: &str,
    payload: PayloadUpdateComment,
    connection: &mut PgConnection,
) -> Result<Comment, Errors> {
    let uuid = convert_str_to_uuid(id)?;

    diesel::update(comments::table.find(uuid))
        .set(payload)
        .get_result::<Comment>(connection)
        .map_err(Errors::Database)
}
