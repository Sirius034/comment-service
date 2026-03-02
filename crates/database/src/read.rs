use crate::establish_connection;
use crate::models::Comment;
use crate::schema::comments;
use crate::web_request::{Comments, ErrorRequest, WebRequest};

pub fn get_comments(json: Option<&str>) -> Result<Vec<Comment>, ErrorRequest> {
    let connection = &mut establish_connection();
    let comments_req = Comments::new();

    comments_req
        .init_sql_query_from_json(json)?
        .load(connection)
}

pub fn get_comment(comment_id: &str) -> Result<Comment, diesel::result::Error> {
    use diesel::prelude::*;
    use uuid::Uuid;

    let connection = &mut establish_connection();

    let id = Uuid::parse_str(comment_id).unwrap();

    comments::table
        .find(id)
        .select(Comment::as_select())
        .first(connection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comment() {
        use crate::create::create_comment;
        use crate::delete::delete_comment;
        use crate::models::NewComment;

        let payload_comment = NewComment {
            type_: 1,
            comment: Some("".to_string()),
            page_id: None,
            user_name: None,
            user_id: None,
            client_id: None,
        };

        let new_comment = create_comment(&payload_comment);

        if let Ok(comment) = new_comment {
            let id = comment.id.to_string();
            let result = get_comment(&id);

            assert!(result.is_ok());

            delete_comment(&id).unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "Record not found")]
    fn test_comment_not_found() {
        let result = get_comment("cfcf6310-f589-49f8-ae98-1d1b60b5c6c8");

        if let Err(error) = result {
            panic!("{error}")
        }
    }

    #[test]
    fn test_get_comments() {
        match get_comments(None) {
            Ok(comments) => {
                println!("Comments: {comments:#?}");
            }
            Err(error) => panic!("Test get_comments failed: {error}"),
        }
    }

    #[test]
    fn test_get_comments_with_json() {
        let json = "{\n\"filter\": {\n\"user_name\": {\n\"_eq\": \"User\"\n},\n\"comment\": {\n\"_neq\": \"Hello\"\n},\n\"type_\": {\n\"_eq\": 1\n},\n\"pinned\": {\n\"_eq\": false\n}\n}\n}";

        match get_comments(Some(json)) {
            Ok(comments) => {
                println!("Comments with parameters: {comments:#?}");
            }
            Err(error) => panic!("Test test_get_comments_with_json failed: {error}"),
        }
    }

    #[test]
    #[should_panic]
    fn test_get_comments_with_no_valid_json() {
        let json = "{\n\"filter\": {\n\"comment\": {\n\"_eq\": \"false\"\n}\n}\n}}";

        if let Err(error) = get_comments(Some(json)) {
            panic!("{error}")
        }
    }
}
