table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        text -> Text,
        user_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        encrypted_password -> Varchar,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
