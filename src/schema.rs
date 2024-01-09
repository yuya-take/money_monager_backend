// @generated automatically by Diesel CLI.

diesel::table! {
    user_activation (code_id) {
        code_id -> Int8,
        user_id -> Int8,
        #[max_length = 6]
        onetime_code -> Bpchar,
        is_used -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int8,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 50]
        oauth_provider -> Nullable<Varchar>,
        #[max_length = 255]
        oauth_provider_id -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(user_activation -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_activation,
    users,
);
