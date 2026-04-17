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
    use uuid::Uuid;

    #[test]
    fn test_crud_lifecycle() {
        let mut connection = test_establish_connection();
        let json = "{\r\n  \"type_\": 1,\r\n  \"comment\": \"Test text comment\",\r\n  \"page_id\": \"12321343134rr33\",\r\n  \"user_name\": \"User Name\",\r\n  \"user_id\": \"1232123\",\r\n  \"client_id\": \"dsa1232123232dsa\"\r\n}";

        let comment = web_request::create_comment(json, &mut connection).unwrap();
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

        let deleted_comment_id =
            web_request::remove_comment(&comment.id.to_string(), &mut connection).unwrap();
        assert_eq!(
            comment.id, deleted_comment_id,
            "Unable to find a match for the deleted comment id"
        );
    }
}
