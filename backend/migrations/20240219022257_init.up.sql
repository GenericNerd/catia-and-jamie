-- Add up migration script here
CREATE TABLE users (
    id BIGINT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);