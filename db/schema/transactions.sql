CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    account_id INT REFERENCES accounts(id) ON DELETE CASCADE,
    type VARCHAR(10) CHECK (type IN ('deposit', 'withdrawal', 'transfer', 'payment')),
    amount DECIMAL(10,2) NOT NULL,
    currency CHAR(3) DEFAULT 'GBP',
    status VARCHAR(10) CHECK (status IN ('pending', 'completed', 'failed')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW()
);
