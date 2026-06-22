use crate::errors::Errors;
use crate::models::Comment;
use crate::web_request::{Comments, WebRequest, read_comment};
use diesel::pg::PgConnection;

pub fn web_get_comments(
    json: Option<&str>,
    connection: &mut PgConnection,
) -> Result<Vec<Comment>, Errors> {
    let comments_req = Comments::new();

    comments_req
        .init_sql_query_from_json(json)?
        .load(connection)
}

pub fn web_get_comment(comment_id: &str, connection: &mut PgConnection) -> Result<Comment, Errors> {
    read_comment(comment_id, connection)
}
