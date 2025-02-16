CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    subject_table VARCHAR(50) CHECK (
        subject_table IN (
            'users'
            'accounts'
            'cards'
            'transfers'
            'transactions'
            'loans'
            'payments'
        )
    ),
    subject_id INT,
    action VARCHAR(50) CHECK (
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
