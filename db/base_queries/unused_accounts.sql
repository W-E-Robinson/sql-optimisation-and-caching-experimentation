-- Accounts that have never been used.
SELECT 
    account.id AS account_id,
FROM
    accounts accs
WHERE NOT EXISTS(
    SELECT 1
    FROM transactions
    WHERE transactions.account_id = accs.id
)
AND NOT EXISTS(
    SELECT 1
    FROM payments
    WHERE payments.account_id = accs.id
)
AND NOT EXISTS(
    SELECT 1
    FROM transfers
    WHERE transfers.sender_account_id = accs.id
    OR transfers.receiver_account_id = accs.id
);
