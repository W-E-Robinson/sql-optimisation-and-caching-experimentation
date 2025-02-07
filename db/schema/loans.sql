CREATE TABLE loans (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE SET NULL,
    amount DECIMAL(10,2) NOT NULL,
    interest_rate DECIMAL(3,2) NOT NULL,
    term_months INT NOT NULL,
    status VARCHAR(10) CHECK (status IN ('approved', 'rejected', 'active', 'closed')) DEFAULT 'approved',
    created_at TIMESTAMP DEFAULT NOW()
);
-- add amount paid so far? - redundant from all payments?
