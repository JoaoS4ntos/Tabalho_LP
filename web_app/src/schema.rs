// @generated automatically by Diesel CLI.

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        totp_secret -> Varchar,
    }
}

table! {
    files (id) {
        id -> Int4,
        filename -> Varchar,
        file_data -> Bytea,
    }
}

