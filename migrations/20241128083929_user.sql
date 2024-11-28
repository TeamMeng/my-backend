-- Add migration script here
CREATE TABLE users (
    id bigserial PRIMARY KEY,
    name varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    -- hashed argon2 password, length 97
    password_hash varchar(97) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
