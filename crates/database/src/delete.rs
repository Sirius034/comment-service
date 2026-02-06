use crate::establish_connection;
use crate::schema::comments;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete_comment(comment_id: &str) -> Result<Uuid, diesel::result::Error> {
    let connection = &mut establish_connection();
    let id = Uuid::parse_str(comment_id).expect("Invalid ID");
    let select_comment = comments::table.filter(comments::id.eq(id));

    diesel::delete(select_comment)
        .returning(comments::id)
        .get_result::<Uuid>(connection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_comment() {
        use crate::create;
        use crate::models::NewComment;

        let new_comment = NewComment {
            type_: 2,
            comment: Some("Content text".to_string()),
            page_id: None,
            user_id: None,
            user_name: None,
            client_id: None,
        };

        if let Ok(comment) = create::create_comment(&new_comment) {
            println!("Сomment {} successfully created", comment.id);

            match delete_comment(&comment.id.to_string()) {
                Ok(id) => println!("Сomment {id} successfully deleted"),
                Err(error) => panic!("Test delete_comment failed: {error}"),
            }
        } else {
            panic!("Failed to create a comment for the test");
        }
    }
}
