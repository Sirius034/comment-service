use super::ErrorRequest;
use crate::models::{Comment, NewComment};
use crate::schema::comments;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde_json::from_str;
use uuid::Uuid;

fn convert_str_to_uuid(value: &str) -> Result<Uuid, ErrorRequest> {
    Uuid::parse_str(value).map_err(|error| ErrorRequest::InvalidUuid {
        field: "id".to_string(),
        source: error,
    })
}

pub fn create_comment(json: &str, connection: &mut PgConnection) -> Result<Comment, ErrorRequest> {
    let payload_data = from_str::<NewComment>(json).map_err(ErrorRequest::InvalidRequest)?;
    let new_comment = NewComment { ..payload_data };

    diesel::insert_into(comments::table)
        .values(new_comment)
        .returning(Comment::as_returning())
        .get_result(connection)
        .map_err(ErrorRequest::Database)
}

pub fn read_comment(id: &str, connection: &mut PgConnection) -> Result<Comment, ErrorRequest> {
    let uuid = convert_str_to_uuid(id)?;

    comments::table
        .find(uuid)
        .select(Comment::as_select())
        .first(connection)
        .map_err(ErrorRequest::Database)
}

pub fn remove_comment(id: &str, connection: &mut PgConnection) -> Result<Uuid, ErrorRequest> {
    use diesel;

    let uuid = convert_str_to_uuid(id)?;
    let select_comment = comments::table.filter(comments::id.eq(uuid));

    diesel::delete(select_comment)
        .returning(comments::id)
        .get_result::<Uuid>(connection)
        .map_err(ErrorRequest::Database)
}
