// @generated automatically by Diesel CLI.

diesel::table! {
    results (id) {
        id -> Uuid,
        url -> Text,
        source_url -> Text,
        title -> Text,
        author -> Text,
        source -> Nullable<Text>,
        category -> Text,
        location -> Text,
        tags -> Jsonb,
        site_name -> Nullable<Text>,
        word_count -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        notes -> Text,
        published_date -> Jsonb,
        summary -> Nullable<Text>,
        image_url -> Nullable<Text>,
        parent_id -> Jsonb,
        reading_progress -> Float8,
    }
}
