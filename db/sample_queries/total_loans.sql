-- Total value of loans outstanding per user.
SELECT 
    users.id AS user_id,
    SUM(loans.amount) AS loans_outstanding
FROM 
    users
JOIN 
    loans ON users.id = loans.user_id AND loans.status = 'active'
GROUP BY 
    users.id;
