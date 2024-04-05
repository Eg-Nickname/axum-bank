-- Table for user request eg. API key permission / Add new currency
CREATE TABLE IF NOT EXISTS user_requests (
    id bigserial PRIMARY KEY UNIQUE,
    request_user BIGINT NOT NULL,
    FOREIGN KEY (request_user) REFERENCES users (id) ON DELETE CASCADE,
    request_mesage TEXT NOT NULL,
    request_type INT4 NOT NULL DEFAULT 0
);