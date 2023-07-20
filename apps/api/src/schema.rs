// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        name -> Nullable<Text>,
        nip -> Nullable<Text>,
        country -> Nullable<Text>,
        city -> Nullable<Text>,
        address -> Nullable<Text>,
        zip_code -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        birthday -> Nullable<Date>,
        organization -> Nullable<Text>,
        role -> Nullable<Text>,
        department -> Nullable<Text>,
        joining_date -> Nullable<Date>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        username -> Varchar,
        password -> Text,
        role -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    people,
    users,
);
