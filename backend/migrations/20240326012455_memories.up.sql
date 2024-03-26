-- Add up migration script here
CREATE TABLE memories (
    memory_id BIGINT NOT NULL,
    table_name TEXT NOT NULL,
    url TEXT NOT NULL,
    approved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (memory_id)
);