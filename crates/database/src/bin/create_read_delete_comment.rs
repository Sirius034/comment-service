use database::models::NewComment;
use database::{create, delete, read};

fn main() {
    let new_comment = NewComment {
        type_: 2,
        comment: Some("Hello!".to_string()),
        page_id: None,
        user_name: Some("User".to_string()),
        user_id: Some("0001111".to_string()),
        client_id: None,
    };

    let created_comment = create::create_comment(&new_comment).expect("Comment creation failed");

    println!("*** Created: {:#?}", created_comment.id);

    let data_comment = read::get_comment(&created_comment.id.to_string())
        .expect("Comment reading failed");

    println!("*** Full created data comment: {data_comment:#?}");

    let comments = read::get_comments(None).expect("Could not get comments");

    println!("*** Сomments: {comments:#?}");

    match delete::delete_comment(&data_comment.id.to_string()) {
        Ok(remove_comment_id) => println!("*** Comment {remove_comment_id} deleted"),
        Err(error) => panic!("Error: {error}"),
    }
}
