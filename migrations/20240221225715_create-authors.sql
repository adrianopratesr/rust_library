-- Add migration script here
CREATE TABLE IF NOT EXISTS authors(
author_id UUID PRIMARY KEY,
name VARCHAR(255) NOT NULL, 
date_of_birth DATE ,
creation_time TIMESTAMPTZ NOT NULL DEFAULT now(),
deletion_time TIMESTAMPTZ DEFAULT NULL
);