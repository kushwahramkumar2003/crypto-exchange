// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 512]
        password -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
