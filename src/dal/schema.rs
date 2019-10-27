table! {
    article (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Varchar,
        raw_content -> Text,
        rendered_content -> Text,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        article_type -> Varchar,
        category -> Varchar,
        tag -> Varchar,
        page_view -> Int4,
        thumb_up -> Int4,
        published -> Bool,
        enabled_comment -> Bool,
        slug_url -> Varchar,
    }
}

table! {
    comment (id) {
        id -> Int4,
        user_id -> Int4,
        article_id -> Int4,
        content -> Text,
        reply_content -> Nullable<Text>,
        create_time -> Timestamp,
        published -> Bool,
    }
}

table! {
    jotting (id) {
        id -> Int4,
        content -> Text,
        weather -> Varchar,
        mood -> Varchar,
        create_time -> Timestamp,
        published -> Bool,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        hashed_password -> Varchar,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        starred -> Bool,
        email -> Varchar,
        personal_site -> Nullable<Varchar>,
        hobby -> Nullable<Varchar>,
        hometown -> Nullable<Varchar>,
    }
}

table! {
    visitor_log (id) {
        id -> Int4,
        ip -> Inet,
        access_time -> Timestamp,
        user_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    article,
    comment,
    jotting,
    user,
    visitor_log,
);
