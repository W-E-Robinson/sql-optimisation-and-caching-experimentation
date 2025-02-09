CREATE MATERIALIZED VIEW suspicious_transactions AS
WITH transaction_counts AS (
    SELECT 
        a.user_id,
        t.account_id,
        t.id AS transaction_id,
        t.amount,
        COUNT(*) OVER (
            PARTITION BY t.account_id 
            ORDER BY t.created_at 
            RANGE BETWEEN INTERVAL '10 minutes' PRECEDING AND CURRENT ROW
        ) AS transaction_count_last_10_minutes
    FROM transactions t
    JOIN accounts a ON t.account_id = a.id
    WHERE t.status IN ('completed', 'pending')
)
SELECT 
    tc.user_id,
    tc.account_id,
    tc.transaction_id,
    tc.amount,
    CASE 
        WHEN tc.amount > (ata.average_transaction * 5) THEN 'high value transaction'
        WHEN tc.amount < 10 AND tc.transaction_count_last_10_minutes > 5 THEN 'potential rapid successive small transactions'
        ELSE 'normal'
    END AS risk_level
FROM transaction_counts tc
JOIN average_transaction_amount ata 
    ON tc.account_id = ata.account_id;
    add denorm num of cards
