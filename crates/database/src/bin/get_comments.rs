use database::read;

fn main() {
    // let json = "{\n\"filter\": {\n\"user_name\": {\n\"_eq\": \"User\"\n},\n\"comment\": {\n\"_neq\": \"Hello\"\n},\n\"type_\": {\n\"_eq\": 1\n},\n\"pinned\": {\n\"_eq\": false\n}\n}\n}";
    // let json = "{\n\"filter\": {\n\"id\": {\n\"_eq\": \"cfcf6310-f589-49f8-ae98-1d1b60b5c6c7\"\n}\n}\n}";
    // let json = "{\n\"filter\": {\n\"comment\": {\n\"_eq\": \"Hello\"\n}\n}\n}";
    
    let json = "{\n\"sort\": [ \"-pinned\" ]\n}";
    
    let comments = read::get_comments(Some(json));

    println!("Schema: {:#?}", comments);
}
