table! {
    client (id) {
        name -> Nullable<Text>,
        password -> Nullable<Text>,
        profession -> Text,
        id -> Int4,
        count -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    product (id) {
        id -> Int4,
        uid -> Nullable<Int4>,
        count -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        date_of_birth -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    client,
    posts,
    product,
    users,
);
