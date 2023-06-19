-- Add migration script here

ALTER TABLE USERS ADD COLUMN salt TEXT NOT NULL;
