-- write likely outpaces read so not good mat view candidate, but good exercise anyway.
CREATE MATERIALIZED VIEW loans_outstanding AS
SELECT 
    users.id AS user_id,
    SUM(loans.amount) AS sum_loans_outstanding
FROM 
    users
JOIN 
    loans ON users.id = loans.user_id AND loans.status = 'active'
GROUP BY 
    users.id;
