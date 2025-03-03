-- Accounts that have never been used.
SELECT 
    ac.id AS account_id
FROM
    accounts ac
WHERE NOT EXISTS(
    SELECT 1
    FROM transactions t
    WHERE t.account_id = ac.id
)
AND NOT EXISTS(
    SELECT 1
    FROM payments p
    WHERE p.account_id = ac.id
)
AND NOT EXISTS(
    SELECT 1
    FROM transfers tr
    WHERE tr.sender_account_id = ac.id
    OR tr.receiver_account_id = ac.id
);
