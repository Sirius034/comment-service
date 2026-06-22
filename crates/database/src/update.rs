use crate::errors::Errors;
use crate::models::Comment;
use crate::web_request::{PayloadUpdateComment, update_comment as request_update_comment};
use diesel::pg::PgConnection;
use serde_json::from_str;

pub fn web_update_comment(
    id: &str,
    json: &str,
    connection: &mut PgConnection,
) -> Result<Comment, Errors> {
    let payload = from_str::<PayloadUpdateComment>(json).map_err(Errors::InvalidRequest)?;
    request_update_comment(id, payload, connection)
}
