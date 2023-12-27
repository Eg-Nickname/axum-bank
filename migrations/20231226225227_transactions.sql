CREATE TABLE IF NOT EXISTS transactions (
    id bigserial PRIMARY KEY UNIQUE,

    sender_id BIGINT,
    FOREIGN KEY (sender_id) REFERENCES users (id) ON DELETE SET NULL,

    reciver_id BIGINT,
    FOREIGN KEY (reciver_id) REFERENCES users (id) ON DELETE SET NULL,

    currency_id BIGINT NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currencies (id) ON DELETE NO ACTION,
    
    ammount BIGINT NOT NULL DEFAULT 0,

    timestamp TIMESTAMPTZ NOT NULL DEFAULT now(),
    status INT4 NOT NULL DEFAULT 0,
    type INT4 NOT NULL DEFAULT 0,
    title TEXT NOT NULL DEFAULT 'Tytul'
);