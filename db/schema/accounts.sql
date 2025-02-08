CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    account_type VARCHAR(10) CHECK (account_type IN ('checking', 'savings', 'credit', 'business')),
    balance DECIMAL(10,2) DEFAULT 0.00,
    currency CHAR(3) DEFAULT 'GBP',
    created_at TIMESTAMP DEFAULT NOW()
);
