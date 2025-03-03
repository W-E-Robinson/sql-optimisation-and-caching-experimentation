-- Total balance for each user across all accounts, user needs at least one count.
SELECT 
    users.id AS user_id,
    SUM(accounts.balance) AS total_balance
FROM 
    users
JOIN 
    accounts ON users.id = accounts.user_id
GROUP BY 
    users.id;
