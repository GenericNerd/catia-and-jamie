-- Add up migration script here
CREATE TABLE user_sessions (
    user_id BIGINT NOT NULL,
    token TEXT NOT NULL,
    PRIMARY KEY (user_id, token)
);