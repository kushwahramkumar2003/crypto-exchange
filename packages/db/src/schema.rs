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

diesel::table! {
    wallet (id) {
        id -> Uuid,
        user_id -> Uuid,
        balance -> Numeric,
        locked_balance -> Numeric,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(wallet -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, wallet,);
