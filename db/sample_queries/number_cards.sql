-- Active cards per user.
WITH cards_per_account AS (
    SELECT 
        accounts.user_id AS account_user_id,
        COUNT(CASE WHEN cards.status = 'active' THEN cards.id END) AS cards_count
    FROM 
        accounts
    LEFT JOIN 
        cards ON accounts.id = cards.account_id
    GROUP BY 
        accounts.user_id
)
SELECT 
    users.id AS user_id,
    COALESCE(SUM(cards_per_account.cards_count), 0 ) AS cards_total
FROM users
LEFT JOIN cards_per_account
ON users.id = cards_per_account.account_user_id
GROUP BY users.id;
