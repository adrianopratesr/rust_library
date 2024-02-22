-- Add migration script here
CREATE INDEX IF NOT EXISTS books_author_id on books(
    author_id
);

CREATE TABLE IF NOT EXISTS collections(
collection_id UUID PRIMARY KEY,
name VARCHAR(255) NOT NULL,
book_ids UUID[] NOT NULL
);