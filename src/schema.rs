table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        text -> Text,
        user_id -> Int8,
    }
}

table! {
    user_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Varchar,
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
joinable!(user_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    user_tokens,
    users,
);
