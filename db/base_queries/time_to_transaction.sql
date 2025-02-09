-- Average time for a transfer, payment, or transaction to happen on a new account.
SELECT 
    AVG(EXTRACT(EPOCH FROM action.timestamp - accounts.timestamp)) AS average_time_seconds
FROM 
    accounts
JOIN (
    SELECT timestamp, account_id FROM transactions
    UNION ALL
    SELECT timestamp, account_id FROM transfers
    UNION ALL
    SELECT timestamp, account_id FROM payments
) AS action ON accounts.id = action.account_id;
