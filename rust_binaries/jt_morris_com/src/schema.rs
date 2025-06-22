// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    auth_state (id) {
        id -> Uuid,
        started -> Timestamptz,
        return_url -> Nullable<Text>,
        scope -> Text,
        redirect_url -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int8,
        expires -> Timestamptz,
        data -> Jsonb,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        total_cost -> Numeric,
        vendor -> Text,
        user_id -> Uuid,
        date -> Date,
    }
}

diesel::table! {
    transactions_items (id) {
        id -> Uuid,
        transaction_id -> Uuid,
        price_usd_cents -> Numeric,
        quantity -> Numeric,
        item_name -> Text,
    }
}

diesel::table! {
    transactions_items_categories (id) {
        id -> Uuid,
        transaction_item_id -> Uuid,
        category -> Nullable<Text>,
    }
}

diesel::table! {
    transactions_items_users (transactions_item_id, user_id) {
        transactions_item_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        auth0_user_id -> Text,
        full_name -> Text,
        email -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        picture -> Nullable<Text>,
        profile -> Nullable<Text>,
        username -> Text,
    }
}

diesel::table! {
    vehicles (vin) {
        #[max_length = 20]
        vin -> Varchar,
        make -> Text,
        model -> Text,
        color -> Text,
        year -> Text,
    }
}

diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(transactions_items -> transactions (transaction_id));
diesel::joinable!(transactions_items_categories -> transactions_items (transaction_item_id));
diesel::joinable!(transactions_items_users -> transactions_items (transactions_item_id));
diesel::joinable!(transactions_items_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    auth_state,
    sessions,
    transactions,
    transactions_items,
    transactions_items_categories,
    transactions_items_users,
    users,
    vehicles,
);
