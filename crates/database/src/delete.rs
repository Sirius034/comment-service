use crate::errors::Errors;
use crate::web_request::remove_comment;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn web_delete_comment(comment_id: &str, connection: &mut PgConnection) -> Result<Uuid, Errors> {
    remove_comment(comment_id, connection)
}
