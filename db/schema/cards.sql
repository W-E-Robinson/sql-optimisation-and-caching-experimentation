CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    account_id INT REFERENCES accounts(id) ON DELETE CASCADE,
    card_number VARCHAR(16) UNIQUE NOT NULL,
    card_type VARCHAR(10) CHECK (card_type IN ('debit', 'credit')) NOT NULL,
    expiration_date DATE NOT NULL,
    status VARCHAR(10) CHECK (status IN ('active', 'blocked', 'expired')) DEFAULT 'active'
);
