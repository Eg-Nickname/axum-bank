-- Set correct timezone
SET timezone = 'CET';

CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY UNIQUE,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);