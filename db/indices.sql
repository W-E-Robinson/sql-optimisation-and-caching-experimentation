CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_amount ON transactions(amount);

CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_balance ON accounts(balance);

CREATE INDEX idx_loans_user_id ON loans(user_id);
CREATE INDEX idx_loans_amount ON loans(amount);
