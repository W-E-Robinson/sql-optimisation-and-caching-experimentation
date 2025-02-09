CREATE MATERIALIZED VIEW average_transaction_amount AS
SELECT 
    account.id AS account_id,
    AVG(transaction.amount) AS average_transaction
FROM 
    accounts
JOIN 
    transactions ON accounts.id = transactions.account_id;
