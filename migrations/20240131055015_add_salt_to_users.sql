-- Add migration script here
ALTER TABLE users ADD COLUMN salt text NOT NULL;
