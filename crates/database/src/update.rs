use crate::establish_connection;
use crate::models::Comment;
use crate::web_request::{ErrorRequest, update_comment as request_update_comment};

pub fn web_update_comment(id: &str, json: &str) -> Result<Comment, ErrorRequest> {
    let connection = &mut establish_connection();

    request_update_comment(id, json, connection)
}
