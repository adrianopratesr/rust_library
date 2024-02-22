-- Add migration script here
CREATE TYPE genre AS ENUM(
    'ROMANCE',
    'MYSTERY',
    'SCIENCE_FICTION',
    'FANTASY',
    'NONFICTION',
    'HISTORY',
    'BIOGRAPHY',
    'POETRY',
    'THRILLER',
    'SELF_HELP'
);

CREATE TYPE lang AS ENUM(
    'ENGLISH',
    'PORTUGUESE',
    'SPANISH',
    'GERMAN'
);

CREATE TABLE IF NOT EXISTS books(
book_id UUID PRIMARY KEY,
name VARCHAR(255) NOT NULL, 
author_id UUID REFERENCES authors,
genre genre NOT NULL,
rating DOUBLE PRECISION NOT NULL DEFAULT 0,
lang lang NOT NULL,
collection_id UUID DEFAULT NULL
);