// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 128]
        title -> Varchar,
        year -> Int4,
        is_current -> Bool,
        #[max_length = 128]
        program_title -> Nullable<Varchar>,
        program_text -> Nullable<Text>,
        image_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    galleries (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        featured_image_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    gallery_images (gallery_id, image_id) {
        gallery_id -> Int4,
        image_id -> Int4,
    }
}

diesel::table! {
    images (id) {
        id -> Int4,
        image_url -> Text,
        image_key -> Text,
        width -> Int4,
        height -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    news (id) {
        id -> Int4,
        #[max_length = 256]
        title -> Varchar,
        message -> Text,
        image_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 64]
        phone_number -> Varchar,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        #[max_length = 64]
        type_ -> Varchar,
        #[max_length = 64]
        role -> Varchar,
        event_id -> Nullable<Int4>,
        #[max_length = 64]
        request -> Varchar,
        image_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(events -> images (image_id));
diesel::joinable!(galleries -> images (featured_image_id));
diesel::joinable!(gallery_images -> galleries (gallery_id));
diesel::joinable!(gallery_images -> images (image_id));
diesel::joinable!(news -> images (image_id));
diesel::joinable!(users -> events (event_id));
diesel::joinable!(users -> images (image_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    galleries,
    gallery_images,
    images,
    news,
    users,
);