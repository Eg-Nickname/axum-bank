-- Add migration script here
CREATE TABLE IF NOT EXISTS transaction_orders (
    id bigserial PRIMARY KEY UNIQUE,
    sender_id BIGINT,
    FOREIGN KEY (sender_id) REFERENCES users (id) ON DELETE CASCADE,

    reciver_id BIGINT,
    FOREIGN KEY (reciver_id) REFERENCES users (id) ON DELETE CASCADE,

    amount BIGINT NOT NULL,
    currency_id BIGINT NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currencies (id) ON DELETE CASCADE,

    last_transaction TIMESTAMPTZ NOT NULL DEFAULT now(),

    transaction_delay INT NOT NULL,
    title TEXT NOT NULL DEFAULT 'Tytul'
);