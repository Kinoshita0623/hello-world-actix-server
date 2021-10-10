table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        text -> Text,
    }
}

table! {
    users (id) {
        id -> Bigint,
        username -> Varchar,
        encrypted_password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
