use crate::establish_connection;
use crate::models::Comment;
use crate::schema::comments;
use crate::web_request::{Comments, WebRequest};
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_comments(json: Option<&str>) -> Result<Vec<Comment>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let commens_req = Comments::new();
    
    commens_req
      .init_sql_guery_from_json(json)
      .load(connection)
}

pub fn get_comment(comment_id: &str) -> Result<Comment, diesel::result::Error> {
    let connection = &mut establish_connection();

    let id = Uuid::parse_str(comment_id).expect("Invalid ID");

    comments::table
        .find(id)
        .select(Comment::as_select())
        .first(connection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comments() {
        match get_comments(None) {
            Ok(comments) => {
                println!("Comments: {comments:#?}");
            }
            Err(error) => panic!("Test get_comments failed: {error}"),
        }
    }
}
