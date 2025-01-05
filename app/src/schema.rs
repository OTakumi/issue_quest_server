diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}
