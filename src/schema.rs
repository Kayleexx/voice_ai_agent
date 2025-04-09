// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Nullable<Integer>,
        user_input -> Text,
        bot_response -> Text,
        timestamp -> Nullable<Timestamp>,
    }
}
