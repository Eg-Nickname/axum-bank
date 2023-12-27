CREATE TABLE IF NOT EXISTS currencies (
    id bigserial PRIMARY KEY UNIQUE,
    -- Eg. EUR
    code varchar(5) NOT NULL UNIQUE ,
    -- Eg .Euro
    name varchar(32) NOT NULL UNIQUE,
    -- item id corresponding to this currency
    item INT NOT NULL
);