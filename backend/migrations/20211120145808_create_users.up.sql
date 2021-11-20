-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL, 
    email       TEXT NOT NULL,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);
