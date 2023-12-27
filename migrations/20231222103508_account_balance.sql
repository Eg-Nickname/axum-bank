CREATE TABLE IF NOT EXISTS account_balance (
    user_id BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,

    currency_id BIGINT NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currencies (id) ON DELETE CASCADE,
    
    balance BIGINT NOT NULL DEFAULT 0,
    
    PRIMARY KEY (user_id, currency_id)
);