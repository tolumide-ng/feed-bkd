table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        slug -> Text,
    }
}
