-- Add migration script here
CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,
    short CHAR(6) NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL UNIQUE
)
