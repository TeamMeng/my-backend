-- Add migration script here
CREATE TABLE urls (
    id CHAR(6) PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL UNIQUE
)
