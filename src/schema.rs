table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        userId -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(posts -> users (userId));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
