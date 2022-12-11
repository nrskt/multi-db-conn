// @generated automatically by Diesel CLI.

diesel::table! {
    sample_user (id) {
        id -> Int4,
        tenant_id -> Int4,
        value -> Text,
    }
}
