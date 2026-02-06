use crate::establish_connection;
use crate::models::Comment;
use crate::schema::comments;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_comments() -> Result<Vec<Comment>, diesel::result::Error> {
    let connection = &mut establish_connection();

    comments::table
        .select(Comment::as_select())
        .load(connection)
}

pub fn get_comment(comment_id: &str) -> Result<Option<Comment>, diesel::result::Error> {
    let connection = &mut establish_connection();

    let id = Uuid::parse_str(comment_id).expect("Invalid ID");

    comments::table
        .find(id)
        .select(Comment::as_select())
        .first(connection)
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comments() {
        match get_comments() {
            Ok(comments) => {
                println!("Comments: {comments:#?}");
            }
            Err(error) => panic!("Test get_comments failed: {error}"),
        }
    }
}
