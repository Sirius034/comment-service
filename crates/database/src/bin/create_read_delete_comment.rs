use database::models::NewComment;
use database::{create, read};

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

    println!("Created: {created_comment:#?}");

    let comments = read::get_comments().expect("Could not get comments");

    println!("Сomments: {comments:#?}");

    if let Some(comment) = comments.first() {
        let data_comment = read::get_comment(&comment.id.to_string())
            .unwrap_or_else(|error| panic!("Error: {error}"));

        println!("Full data comment: {data_comment:#?}");
    }
}
