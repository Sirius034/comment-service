use diesel::prelude::*;
use crate::establish_connection;
use crate::models::{Comment, NewComment};
use crate::schema::comments;

pub fn create_comment(new_comment: &NewComment) -> Result<Comment, diesel::result::Error> {
    let connecton = &mut establish_connection();

    diesel::insert_into(comments::table)
        .values(new_comment)
        .returning(Comment::as_returning())
        .get_result(connecton)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_comment() {
    let payload_comment = NewComment {
      type_: 1,
      comment: Some("Test comment".to_string()),
      page_id: None,
      user_name: None,
      user_id: None,
      client_id: None,
    };
    
    match create_comment(&payload_comment) {
      Ok(comment) => {
        println!("Comment: {comment:?}");
      }
      Err(error) => panic!("Test create_comment failed: {error}")
    }
  }
}
