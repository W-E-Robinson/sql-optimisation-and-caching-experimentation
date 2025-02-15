CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(255) CHECK (
        action IN (
            'user created',
            'user updated',
            'user deleted',
            'transfer created',
            'transfer updated',
            'transaction created',
            'transaction deleted',
            'payment created',
            'payment updated',
            'loan created',
            'loan updated',
            'card created',
            'card updated',
            'card deleted',
            'account created',
            'account updated',
            'account deleted'
        )
    ),
    details TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
