table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        slug -> VarChar,
        description -> VarChar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
