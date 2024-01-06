CREATE TABLE IF NOT EXISTS currency_exchange_listings (
    id bigserial PRIMARY KEY UNIQUE,

    listing_creator BIGINT,
    FOREIGN KEY (listing_creator) REFERENCES users (id) ON DELETE CASCADE,

    currency_from_id BIGINT NOT NULL,
    FOREIGN KEY (currency_from_id) REFERENCES currencies (id) ON DELETE NO ACTION,
    amount_to BIGINT NOT NULL DEFAULT 0,
    
    currency_to_id BIGINT NOT NULL,
    FOREIGN KEY (currency_to_id) REFERENCES currencies (id) ON DELETE NO ACTION,
    amount_from BIGINT NOT NULL DEFAULT 0,


    ratio_from  BIGINT NOT NULL,
    ratio_to  BIGINT NOT NULL,

    is_fixed BOOL NOT NULL DEFAULT FALSE,
    creation_timestamp TIMESTAMPTZ NOT NULL DEFAULT now()
);