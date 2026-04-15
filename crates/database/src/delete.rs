use crate::establish_connection;
use crate::web_request::{ErrorRequest, remove_comment};
use uuid::Uuid;

pub fn delete_comment(comment_id: &str) -> Result<Uuid, ErrorRequest> {
    let connection = &mut establish_connection();

    remove_comment(comment_id, connection)
}
