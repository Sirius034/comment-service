use crate::establish_connection;
use crate::models::Comment;
use crate::web_request::{Comments, ErrorRequest, WebRequest, read_comment};

pub fn get_comments(json: Option<&str>) -> Result<Vec<Comment>, ErrorRequest> {
    let connection = &mut establish_connection();
    let comments_req = Comments::new();

    comments_req
        .init_sql_query_from_json(json)?
        .load(connection)
}

pub fn get_comment(comment_id: &str) -> Result<Comment, ErrorRequest> {
    let connection = &mut establish_connection();

    read_comment(comment_id, connection)
}

