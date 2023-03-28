// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Int4,
        ban -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        description -> Text,
        extra -> Nullable<Text>,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Text,
        image_url -> Text,
        location -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        link -> Nullable<Text>,
    }
}

diesel::table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    events,
    login_history,
    users,
);
