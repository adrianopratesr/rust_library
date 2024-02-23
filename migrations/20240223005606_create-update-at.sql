-- Add migration script here
ALTER TABLE authors 
    ADD COLUMN
    updated_at TIMESTAMPTZ DEFAULT NULL;

