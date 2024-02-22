-- Add migration script here
ALTER TABLE authors 
ALTER COLUMN date_of_birth SET NOT NULL;