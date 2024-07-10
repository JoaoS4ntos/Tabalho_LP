// @generated automatically by Diesel CLI.
use crate::table;
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        phone -> Varchar,
    }
}

table! {
    files (id) {
        id -> Int4,
        filename -> Varchar,
        file_data -> Bytea,
    }
}

