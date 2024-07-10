-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
);
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    file_data BYTEA NOT NULL
);
