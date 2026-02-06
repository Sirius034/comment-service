// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        data_created -> Timestamp,
        #[sql_name = "type"]
        type_ -> Int4,
        comment -> Nullable<Text>,
        page_id -> Nullable<Text>,
        user_name -> Nullable<Text>,
        user_id -> Nullable<Text>,
        client_id -> Nullable<Text>,
        pinned -> Bool,
    }
}
