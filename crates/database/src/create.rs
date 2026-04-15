use crate::establish_connection;
use crate::models::Comment;
use crate::web_request;

pub fn create_comment(json: &str) -> Result<Comment, web_request::ErrorRequest> {
    let connection = &mut establish_connection();

    web_request::create_comment(json, connection)
}
