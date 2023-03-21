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

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    events,
);
