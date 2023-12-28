-- Set correct timezone
SET timezone = 'CET';
CREATE EXTENSION IF NOT EXISTS citext;  

CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY UNIQUE,
    username CITEXT UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);