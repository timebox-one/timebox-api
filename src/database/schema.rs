table! {
    items (id) {
        id -> Unsigned<Bigint>,
        content -> Text,
        user_id -> Unsigned<Bigint>,
        is_important -> Bool,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

joinable!(items -> users (user_id));

allow_tables_to_appear_in_same_query!(
    items,
    users,
);
