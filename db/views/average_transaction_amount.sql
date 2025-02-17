CREATE MATERIALIZED VIEW average_transaction_amount AS
SELECT 
    accounts.id AS account_id,
    AVG(transactions.amount) AS average_transaction
FROM 
    accounts
JOIN 
    transactions ON accounts.id = transactions.account_id
GROUP BY accounts.id;
