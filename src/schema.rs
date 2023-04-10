// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

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
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        login_session -> Text,
        roles -> Array<Nullable<Role>>,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    events,
    login_history,
    users,
);
