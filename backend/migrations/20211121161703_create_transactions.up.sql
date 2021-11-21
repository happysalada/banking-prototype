-- Add up migration script here
CREATE TABLE IF NOT EXISTS transactions
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    from_id     VARCHAR(26) NOT NULL REFERENCES users(id), 
    to_id       VARCHAR(26) NOT NULL REFERENCES users(id), 
    amount      INTEGER NOT NULL,
    note        TEXT,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);
