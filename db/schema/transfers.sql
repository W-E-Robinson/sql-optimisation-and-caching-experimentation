CREATE TABLE transfers (
    id SERIAL PRIMARY KEY,
    sender_account_id INT REFERENCES accounts(id) ON DELETE SET NULL,
    receiver_account_id INT REFERENCES accounts(id) ON DELETE SET NULL,
    amount DECIMAL(10,2) NOT NULL,
    currency CHAR(3) DEFAULT 'GBP',
    status VARCHAR(10) CHECK (status IN ('pending', 'completed', 'failed')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW()
);
