-- Average time for a transfer, payment, or transaction to happen on a new account.
SELECT 
    AVG(EXTRACT(EPOCH FROM (action.created_at - accounts.created_at))) AS average_time_seconds
FROM 
    accounts
JOIN (
    SELECT created_at, account_id FROM transactions
    UNION ALL
    SELECT created_at, account_id FROM transfers
    UNION ALL
    SELECT created_at, account_id FROM payments
) AS action 
ON accounts.id = action.account_id
WHERE action.created_at >= accounts.created_at;
