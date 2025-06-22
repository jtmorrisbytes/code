// @generated automatically by Diesel CLI.

diesel::table! {
    auth_state (id) {
        id -> Uuid,
        started -> Timestamptz,
        scope -> Text,
        redirect_url -> Text,
        return_url -> Nullable<Text>,
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
        date -> Date,
        user_id -> Uuid,
    }
}

diesel::table! {
    transactions_items (id) {
        id -> Uuid,
        transaction_id -> Uuid,
        price_usd_cents -> Numeric,
        quantity -> Numeric,
        item_name -> Text,
        user_id -> Uuid,
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
    users (id) {
        id -> Uuid,
        full_name -> Text,
        auth0_user_id -> Text,
        email -> Nullable<Text>,
        picture -> Nullable<Text>,
        profile -> Nullable<Text>,
        #[max_length = 256]
        username -> Varchar,
        phone_number -> Nullable<Text>,
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
diesel::joinable!(transactions_items -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_state,
    sessions,
    transactions,
    transactions_items,
    transactions_items_categories,
    users,
    vehicles,
);
