// @generated automatically by Diesel CLI.

diesel::table! {
    my_todos (id) {
        id -> Int4,
        fantasy_name -> Varchar,
        real_name -> Nullable<Varchar>,
        spotted_photo -> Text,
        strength_level -> Int4,
    }
}
