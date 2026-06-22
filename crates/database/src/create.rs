use crate::errors::Errors;
use crate::models::{Comment, NewComment};
use crate::web_request;
use diesel::pg::PgConnection;
use serde_json::from_str;

pub fn web_create_comment(json: &str, connection: &mut PgConnection) -> Result<Comment, Errors> {
    let new_comment = from_str::<NewComment>(json).map_err(Errors::InvalidRequest)?;
    web_request::create_comment(new_comment, connection)
}
