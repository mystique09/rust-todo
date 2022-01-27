table! {
    todos_table (id) {
        id -> Int4,
        author -> Int4,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        completed -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        role -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    todos_table,
    users,
);
