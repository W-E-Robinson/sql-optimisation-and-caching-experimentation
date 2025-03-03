-- write likely outpaces read so not good mat view candidate, but good exercise anyway.
CREATE MATERIALIZED VIEW suspicious_transactions AS
WITH transaction_counts AS (
    SELECT 
        t.account_id,
        t.id AS transaction_id,
        t.amount,
        COUNT(*) OVER (
            PARTITION BY t.account_id 
            ORDER BY t.created_at 
            RANGE BETWEEN INTERVAL '10 minutes' PRECEDING AND CURRENT ROW
        ) AS transaction_count_last_10_minutes
    FROM transactions t
    WHERE t.status IN ('completed', 'pending')
)
SELECT 
    tc.account_id,
    tc.transaction_id,
    tc.amount,
    CASE 
        WHEN tc.amount > (ata.average_transaction * 1.5) THEN 'high value transaction' -- average_transaction multiplier so low to force output
        WHEN tc.amount < 10 AND tc.transaction_count_last_10_minutes > 5 THEN 'potential rapid successive small transactions'
        ELSE 'not suspicious'
    END AS risk_level
FROM transaction_counts tc
JOIN average_transaction_amount ata 
    ON tc.account_id = ata.account_id
WHERE 
    (tc.amount > (ata.average_transaction * 1.5) OR
    (tc.amount < 10 AND tc.transaction_count_last_10_minutes > 5));
