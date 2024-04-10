ALTER TABLE user_requests
ADD COLUMN request_timestamp TIMESTAMPTZ NOT NULL DEFAULT now();