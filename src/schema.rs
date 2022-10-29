// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        telegram_id -> Int4,
        step_id -> Int2,
    }
}
