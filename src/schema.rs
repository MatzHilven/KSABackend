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
