-- Your SQL goes here
CREATE TABLE todos (
    id VARCHAR PRIMARY KEY,
    label VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 'f'
)