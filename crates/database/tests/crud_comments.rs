use diesel::pg::PgConnection;
use diesel::prelude::*;

use database::establish_connection;
use database::web_request;

fn test_establish_connection() -> PgConnection {
    let mut connection = establish_connection();

    connection
        .begin_test_transaction()
        .unwrap_or_else(|error| panic!("Failed to begin test transaction. Error: {error}"));

    connection
}

#[cfg(test)]
mod test {
    use super::*;
    use database::models::NewComment;
    use uuid::Uuid;
    use web_request::PayloadUpdateComment;

    #[test]
    fn test_crud_lifecycle() {
        let mut connection = test_establish_connection();
        let new_comment = NewComment {
            type_: 1,
            comment: Some("Сomment Text".to_string()),
            page_id: Some("1".to_string()),
            user_name: Some("User Name".to_string()),
            user_id: Some("1".to_string()),
            client_id: Some("1".to_string()),
        };

        let comment = web_request::create_comment(new_comment, &mut connection).unwrap();
        assert_eq!(
            comment.user_name,
            Some("User Name".to_string()),
            "Username matching failed"
        );
        assert_eq!(
            Uuid::parse_str(&comment.id.to_string()).is_ok(),
            true,
            "Parsing of identifier failed"
        );

        let identical_comment =
            web_request::read_comment(&comment.id.to_string(), &mut connection).unwrap();
        assert_eq!(
            identical_comment.id, comment.id,
            "Unable to match comment id"
        );
        assert_eq!(
            identical_comment.user_name, comment.user_name,
            "Unable to match comment user_name"
        );

        let new_comment_content = "New text";
        let payload_update_comment = PayloadUpdateComment {
            comment: Some(new_comment_content.to_string()),
            user_name: None,
        };
        let update_comment = web_request::update_comment(
            &comment.id.to_string(),
            payload_update_comment,
            &mut connection,
        )
        .unwrap();
        assert_eq!(
            update_comment.id, comment.id,
            "The update failed, the resulting identifiers do not match"
        );
        assert_eq!(
            update_comment.comment,
            Some(new_comment_content.to_string()),
            "Update failed, message content was not updated"
        );

        let deleted_comment_id =
            web_request::remove_comment(&comment.id.to_string(), &mut connection).unwrap();
        assert_eq!(
            comment.id, deleted_comment_id,
            "Unable to find a match for the deleted comment id"
        );
    }
}
