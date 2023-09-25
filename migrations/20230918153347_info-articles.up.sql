-- Add up migration script here
CREATE TABLE IF NOT EXISTS articles(
    id UUID PRIMARY KEY UNIQUE NOT NULL,
    title VARCHAR(255) UNIQUE NOT NULL,
    article TEXT,
    modified TIMESTAMP NOT NULL
);