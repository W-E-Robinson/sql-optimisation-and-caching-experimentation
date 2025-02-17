pub mod enums;
pub mod models;

use chrono::{DateTime, Duration, Utc};
use enums::account_type::AccountType;
use enums::audit_log_action::AuditLogAction;
use enums::audit_log_subject_table::AuditLogSubjectTable;
use enums::card_status::CardStatus;
use enums::card_type::CardType;
use enums::loan_status::LoanStatus;
use enums::payment_status::PaymentStatus;
use enums::transaction_status::TransactionStatus;
use enums::transaction_type::TransactionType;
use enums::transfer_status::TransferStatus;
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::name::{en::FirstName, en::LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use models::account::AccountRowInsertion;
use models::card::CardRowInsertion;
use models::loan::LoanRowInsertion;
use models::payment::PaymentRowInsertion;
use models::transaction::TransactionRowInsertion;
use models::transfer::TransferRowInsertion;
use models::user::UserRowInsertion;
use rand::Rng;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

const NUM_USERS: i32 = 100;
const NUM_ACCOUNTS_PER_USER: i32 = 4;
const NUM_TRANSFERS_PER_ACCOUNT: i32 = 5;
const NUM_TRANSACTIONS_PER_ACCOUNT: i32 = 2;

struct BankSystemManager {
    db: Pool<Postgres>,
}

impl BankSystemManager {
    fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    fn random_date_past(&self, lower_limit: i64, upper_limit: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let lower = now - Duration::weeks(lower_limit);
        let upper = now - Duration::weeks(upper_limit);
        let random_seconds =
            rand::random::<i64>() % (upper.signed_duration_since(lower).num_seconds());
        lower + Duration::seconds(random_seconds)
    }

    async fn insert_audit_log(
        &self,
        subject_table: &str,
        subject_id: i32,
        action: &str,
        details: String,
        created_at: DateTime<Utc>,
    ) {
        if let Err(e) = sqlx::query(
            "
            INSERT INTO public.audit_logs 
            (subject_table, subject_id, action, details, created_at)
            VALUES ($1, $2, $3, $4, $5);
            ",
        )
        .bind(subject_table)
        .bind(subject_id)
        .bind(action)
        .bind(details)
        .bind(created_at)
        .execute(&self.db)
        .await
        {
            println!(
                 "Error: failed to insert row into 'audit_logs' - <subject_table={}> - <subject_id={}> - <action={}> - <error={:?}>",
                 subject_table, subject_id, action, e
             );
        }
    }

    async fn insert_users(&self) {
        let mut users_count = 1;
        loop {
            if users_count > NUM_USERS {
                break;
            }

            let created_at = self.random_date_past(10, 9);

            let user = UserRowInsertion {
                public_id: Uuid::new_v4(),
                given_name: FirstName().fake(),
                family_name: LastName().fake(),
                username: Username().fake(),
                email: SafeEmail().fake(),
                phone: PhoneNumber().fake(),
                created_at,
            };
            match sqlx::query(
                "
                INSERT INTO public.users 
                (public_id, given_name, family_name, username, email, phone, created_at) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id;
                ",
            )
            .bind(user.public_id)
            .bind(user.given_name)
            .bind(user.family_name)
            .bind(user.username)
            .bind(user.email)
            .bind(user.phone)
            .bind(user.created_at)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let user_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Users.to_string(),
                        user_id,
                        AuditLogAction::UserCreated.to_string(),
                        format!("user id <{}>", user_id),
                        user.created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'users' - <error = {:?}>",
                        e
                    );
                }
            }

            users_count += 1;
        }
    }

    async fn insert_accounts(&self) {
        let mut accounts_count = 1;
        let mut current_user_id = 1;
        let mut accounts_per_user = 0;
        loop {
            if accounts_count > NUM_USERS * NUM_ACCOUNTS_PER_USER {
                break;
            }

            let account_type = match accounts_per_user {
                0 => AccountType::Checking,
                1 => AccountType::Savings,
                2 => AccountType::Credit,
                3 => AccountType::Business,
                _ => AccountType::Checking,
            };
            let created_at = self.random_date_past(9, 8);
            let num_active_cards = match account_type {
                AccountType::Checking => 1,
                AccountType::Savings => 0,
                AccountType::Credit => 1,
                AccountType::Business => 2,
            };

            let account = AccountRowInsertion {
                user_id: current_user_id,
                account_type: account_type.to_string(),
                balance: format!("{:.2}", rand::rng().random_range(0..=1_000_000))
                    .parse()
                    .unwrap_or(0.00),
                created_at,
                num_active_cards,
            };
            match sqlx::query(
                "
                INSERT INTO public.accounts
                (user_id, account_type, balance, created_at, num_active_cards)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
                ",
            )
            .bind(account.user_id)
            .bind(account.account_type)
            .bind(account.balance)
            .bind(account.created_at)
            .bind(account.num_active_cards)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let account_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Accounts.to_string(),
                        account_id,
                        AuditLogAction::AccountCreated.to_string(),
                        format!("account id <{}>", account_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'accounts' - <user_id={}> - <error={:?}>",
                        account.user_id, e
                    );
                }
            }

            accounts_count += 1;
            accounts_per_user += 1;
            if accounts_per_user == NUM_ACCOUNTS_PER_USER {
                accounts_per_user = 0;
                current_user_id += 1;
            }
        }
    }

    async fn insert_cards(&self) {
        let mut current_account_id = 1;
        let mut is_business_account_debit_inserted = false;
        loop {
            if current_account_id > NUM_USERS * NUM_ACCOUNTS_PER_USER {
                break;
            }

            let account_type = match current_account_id % NUM_ACCOUNTS_PER_USER {
                1 => AccountType::Checking,
                2 => AccountType::Savings,
                3 => AccountType::Credit,
                0 => AccountType::Business,
                _ => {
                    println!(
                        "Warning: failed to find account type for account <id={}>, defaulting to Checking account",
                        current_account_id
                    );
                    AccountType::Checking
                }
            };

            if account_type == AccountType::Savings {
                current_account_id += 1;
                continue;
            }

            let card_type = match account_type {
                AccountType::Checking => CardType::Debit,
                AccountType::Credit => CardType::Credit,
                AccountType::Business => {
                    if !is_business_account_debit_inserted {
                        is_business_account_debit_inserted = true;
                        CardType::Debit
                    } else {
                        is_business_account_debit_inserted = false;
                        CardType::Credit
                    }
                }
                _ => {
                    println!(
                        "Warning: failed to find appropriate card type for account <type={}>, for account <id={}>, defaulting to Debit card",
                        account_type.to_string(), current_account_id,
                    );
                    CardType::Debit
                }
            };

            let created_at = self.random_date_past(-6, -12);

            let card = CardRowInsertion {
                account_id: current_account_id,
                card_number: CreditCardNumber().fake(),
                card_type: card_type.to_string(),
                expiration_date: created_at,
                status: CardStatus::Active.to_string(),
            };
            match sqlx::query(
                "
                INSERT INTO public.cards
                (account_id, card_number, card_type, expiration_date, status)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
                ",
            )
            .bind(card.account_id)
            .bind(card.card_number)
            .bind(card.card_type)
            .bind(card.expiration_date)
            .bind(card.status)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let card_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Cards.to_string(),
                        card_id,
                        AuditLogAction::CardCreated.to_string(),
                        format!("card id <{}>", card_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'cards' - <account_id={}> - <error={:?}>",
                        card.account_id, e
                    );
                }
            }
            if account_type == AccountType::Checking
                || account_type == AccountType::Credit
                || (account_type == AccountType::Business
                    && is_business_account_debit_inserted == false)
            {
                current_account_id += 1;
            }
        }
    }

    async fn insert_transfers(&self) {
        let mut current_account_id = 1;
        let mut num_transfers_each_account = 0;
        loop {
            if current_account_id > NUM_USERS * NUM_ACCOUNTS_PER_USER {
                break;
            }

            let account_type = match current_account_id % NUM_ACCOUNTS_PER_USER {
                1 => AccountType::Checking,
                2 => AccountType::Savings,
                3 => AccountType::Credit,
                0 => AccountType::Business,
                _ => {
                    println!(
                        "Warning: failed to find account type for account <id={}>, defaulting to Savings account",
                        current_account_id
                    );
                    AccountType::Savings
                }
            };

            if account_type == AccountType::Savings {
                current_account_id += 1;
                continue;
            } else if account_type == AccountType::Credit {
                current_account_id += 1;
                continue;
            }

            // Set receiver to another account
            let receiver_account_id = (current_account_id + 200) % 400 + 1;

            let created_at = self.random_date_past(6, 5);

            let transfer = TransferRowInsertion {
                sender_account_id: current_account_id,
                receiver_account_id,
                amount: format!("{:.2}", rand::rng().random_range(1..=1_000))
                    .parse()
                    .unwrap_or(1.00),
                status: TransferStatus::Completed.to_string(),
                created_at,
            };
            match sqlx::query(
                "
                INSERT INTO public.transfers
                (sender_account_id, receiver_account_id, amount, status, created_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
                ",
            )
            .bind(transfer.sender_account_id)
            .bind(transfer.receiver_account_id)
            .bind(transfer.amount)
            .bind(transfer.status)
            .bind(transfer.created_at)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let transfer_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Transfers.to_string(),
                        transfer_id,
                        AuditLogAction::TransferCreated.to_string(),
                        format!("transfers id <{}>", transfer_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'transfers' - <sender_account_id={}> - <receiver_account_id={}> - <error={:?}>",
                        transfer.sender_account_id, transfer.receiver_account_id, e
                    );
                }
            }

            num_transfers_each_account += 1;
            if num_transfers_each_account == NUM_TRANSFERS_PER_ACCOUNT {
                num_transfers_each_account = 0;
                current_account_id += 1;
            }
        }
    }

    async fn insert_transactions(&self) {
        let mut current_account_id = 1;
        let mut num_transactions_each_account = 0;
        loop {
            if current_account_id > NUM_USERS * NUM_ACCOUNTS_PER_USER {
                break;
            }

            let account_type = match current_account_id % 4 {
                1 => AccountType::Checking,
                2 => AccountType::Savings,
                3 => AccountType::Credit,
                0 => AccountType::Business,
                _ => {
                    println!(
                        "Warning: failed to find account type for account <id={}>, defaulting to Credit account",
                        current_account_id
                    );
                    AccountType::Credit
                }
            };

            if account_type == AccountType::Savings {
                current_account_id += 1;
                continue;
            } else if account_type == AccountType::Credit {
                current_account_id += 1;
                continue;
            }

            let created_at = self.random_date_past(6, 5);
            let transaction_type = match num_transactions_each_account % 2 {
                0 => TransactionType::Deposit,
                1 => TransactionType::Withdrawal,
                _ => {
                    println!(
                        "Warning: failed to create correct transaction type for account <id={}>, defaulting to Deposit",
                        current_account_id
                    );
                    TransactionType::Deposit
                }
            };

            let transaction = TransactionRowInsertion {
                account_id: current_account_id,
                transaction_type: transaction_type.to_string(),
                amount: format!("{:.2}", rand::rng().random_range(1..=1_000))
                    .parse()
                    .unwrap_or(1.00),
                status: TransactionStatus::Pending.to_string(),
                created_at,
            };
            match sqlx::query(
                "
                INSERT INTO public.transactions
                (account_id, transaction_type, amount, status, created_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
                ",
            )
            .bind(transaction.account_id)
            .bind(transaction.transaction_type)
            .bind(transaction.amount)
            .bind(transaction.status)
            .bind(transaction.created_at)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let transaction_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Transactions.to_string(),
                        transaction_id,
                        AuditLogAction::TransactionCreated.to_string(),
                        format!("transaction id <{}>", transaction_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'transactions' - <account_id={}> - <error={:?}>",
                        transaction.account_id, e
                    );
                }
            }

            num_transactions_each_account += 1;
            if num_transactions_each_account == NUM_TRANSACTIONS_PER_ACCOUNT {
                num_transactions_each_account = 0;
                current_account_id += 1;
            }
        }
    }

    async fn insert_loans(&self) {
        let mut current_user_id = 1;
        loop {
            if current_user_id > NUM_USERS {
                break;
            }

            let created_at = self.random_date_past(6, 5);

            let loan = LoanRowInsertion {
                user_id: current_user_id,
                term_months: 24,
                interest_rate: 4.50,
                amount: format!("{:.2}", rand::rng().random_range(1..=10_000))
                    .parse()
                    .unwrap_or(100.00),
                status: LoanStatus::Active.to_string(),
                created_at,
            };
            match sqlx::query(
                "
                INSERT INTO public.loans
                (user_id, term_months, interest_rate, amount, status, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id;
                ",
            )
            .bind(loan.user_id)
            .bind(loan.term_months)
            .bind(loan.interest_rate)
            .bind(loan.amount)
            .bind(loan.status)
            .bind(loan.created_at)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let loan_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Loans.to_string(),
                        loan_id,
                        AuditLogAction::LoanCreated.to_string(),
                        format!("loan id <{}>", loan_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'loans' - <user_id={}> - <error={:?}>",
                        loan.user_id, e
                    );
                }
            }

            current_user_id += 1;
        }
    }

    async fn insert_payments(&self) {
        let mut current_user_id = 1;
        let mut payments_per_loan_inserted = 0;
        loop {
            if current_user_id > NUM_USERS {
                break;
            }

            let created_at = self.random_date_past(6, 5);

            let payment = PaymentRowInsertion {
                account_id: ((current_user_id - 1) * 4) + 1,
                loan_id: current_user_id,
                amount: format!("{:.2}", rand::rng().random_range(1..=1_000))
                    .parse()
                    .unwrap_or(50.00),
                status: PaymentStatus::Completed.to_string(),
                created_at,
            };
            match sqlx::query(
                "
                INSERT INTO public.payments
                (account_id, loan_id, amount, status, created_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
                ",
            )
            .bind(payment.account_id)
            .bind(payment.loan_id)
            .bind(payment.amount)
            .bind(payment.status)
            .bind(payment.created_at)
            .fetch_one(&self.db)
            .await
            {
                Ok(row) => {
                    let payment_id: i32 = row.get::<i32, _>("id");

                    self.insert_audit_log(
                        AuditLogSubjectTable::Payments.to_string(),
                        payment_id,
                        AuditLogAction::PaymentCreated.to_string(),
                        format!("loan id <{}>", payment_id),
                        created_at,
                    )
                    .await;
                }
                Err(e) => {
                    println!(
                        "Error: failed to insert row into 'payments' - <account_id={}> - <loan_id={}> - <error={:?}>",
                        payment.account_id, payment.loan_id, e
                    );
                }
            }

            payments_per_loan_inserted += 1;
            if payments_per_loan_inserted >= 3 {
                payments_per_loan_inserted = 0;
                current_user_id += 1;
            }
        }
    }

    async fn insert_data(&self) {
        self.insert_users().await;
        self.insert_accounts().await;
        self.insert_cards().await;
        self.insert_transfers().await;
        self.insert_transactions().await;
        self.insert_loans().await;
        self.insert_payments().await;
    }
}

#[cfg(test)]
mod test {
    use sqlx::{PgPool, Row};
    use std::time::Instant;

    use super::*;

    #[sqlx::test(fixtures("../db/schema/users.sql", "../db/schema/audit_logs.sql"))]
    async fn test_users_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;

        let mut conn = pool.acquire().await?;

        let users = sqlx::query("SELECT * FROM public.users")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(users.len(), 100);
        assert_eq!(audit_logs.len(), 100);
        let user_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::UserCreated)
            })
            .collect();
        assert_eq!(user_insertions.len(), 100);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_accounts_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;

        let mut conn = pool.acquire().await?;

        let accounts = sqlx::query("SELECT * FROM public.accounts")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(accounts.len(), 400);
        assert_eq!(audit_logs.len(), 500);
        let account_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::AccountCreated)
            })
            .collect();
        assert_eq!(account_insertions.len(), 400);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_cards_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;
        bank_system_manager.insert_cards().await;

        let mut conn = pool.acquire().await?;

        let cards = sqlx::query("SELECT * FROM public.cards")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(cards.len(), 400);
        assert_eq!(audit_logs.len(), 900);
        let card_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::CardCreated)
            })
            .collect();
        assert_eq!(card_insertions.len(), 400);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/transfers.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_transfers_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;
        bank_system_manager.insert_transfers().await;

        let mut conn = pool.acquire().await?;

        let transfers = sqlx::query("SELECT * FROM public.transfers")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(transfers.len(), 1000);
        assert_eq!(audit_logs.len(), 1500);
        let transfer_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::TransferCreated)
            })
            .collect();
        assert_eq!(transfer_insertions.len(), 1000);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/transactions.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_transactions_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;
        bank_system_manager.insert_transactions().await;

        let mut conn = pool.acquire().await?;

        let transactions = sqlx::query("SELECT * FROM public.transactions")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(transactions.len(), 400);
        assert_eq!(audit_logs.len(), 900);
        let transaction_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::TransactionCreated)
            })
            .collect();
        assert_eq!(transaction_insertions.len(), 400);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/loans.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_loans_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_loans().await;

        let mut conn = pool.acquire().await?;

        let loans = sqlx::query("SELECT * FROM public.loans")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(loans.len(), 100);
        assert_eq!(audit_logs.len(), 200);
        let loan_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::LoanCreated)
            })
            .collect();
        assert_eq!(loan_insertions.len(), 100);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/loans.sql",
        "../db/schema/accounts.sql",
        "../db/schema/payments.sql",
        "../db/schema/audit_logs.sql"
    ))]
    async fn test_payments_inserted(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;
        bank_system_manager.insert_loans().await;
        bank_system_manager.insert_payments().await;

        let mut conn = pool.acquire().await?;

        let payments = sqlx::query("SELECT * FROM public.payments")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(payments.len(), 300);
        assert_eq!(audit_logs.len(), 900);
        let payment_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == Some(AuditLogAction::PaymentCreated)
            })
            .collect();
        assert_eq!(payment_insertions.len(), 300);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/audit_logs.sql",
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/transfers.sql",
        "../db/schema/transactions.sql",
        "../db/schema/loans.sql",
        "../db/schema/payments.sql",
    ))]
    async fn test_all_insertions(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_data().await;

        let mut conn = pool.acquire().await?;

        let users = sqlx::query("SELECT * FROM public.users")
            .fetch_all(&mut *conn)
            .await?;

        let accounts = sqlx::query("SELECT * FROM public.accounts")
            .fetch_all(&mut *conn)
            .await?;

        let cards = sqlx::query("SELECT * FROM public.cards")
            .fetch_all(&mut *conn)
            .await?;

        let transfers = sqlx::query("SELECT * FROM public.transfers")
            .fetch_all(&mut *conn)
            .await?;

        let transactions = sqlx::query("SELECT * FROM public.transactions")
            .fetch_all(&mut *conn)
            .await?;

        let loans = sqlx::query("SELECT * FROM public.loans")
            .fetch_all(&mut *conn)
            .await?;

        let payments = sqlx::query("SELECT * FROM public.payments")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(users.len(), 100);
        assert_eq!(accounts.len(), 400);
        assert_eq!(cards.len(), 400);
        assert_eq!(transfers.len(), 1000);
        assert_eq!(transactions.len(), 400);
        assert_eq!(loans.len(), 100);
        assert_eq!(payments.len(), 300);
        assert_eq!(audit_logs.len(), 2700);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/audit_logs.sql",
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/transfers.sql",
        "../db/schema/transactions.sql",
        "../db/schema/loans.sql",
        "../db/schema/payments.sql",
        "../db/views/average_transaction_amount.sql",
    ))]
    async fn test_average_transaction_amount_time_difference_raw_sql_materialized_view(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_data().await;

        let mut conn = pool.acquire().await?;

        sqlx::query("REFRESH MATERIALIZED VIEW public.average_transaction_amount")
            .execute(&mut *conn)
            .await?;

        let raw_start = Instant::now();
        let raw_sql = sqlx::query(
            "
            SELECT 
                accounts.id AS account_id,
                AVG(transactions.amount) AS average_transaction
            FROM 
                accounts
            JOIN 
                transactions ON accounts.id = transactions.account_id
            GROUP BY accounts.id;
            ",
        )
        .fetch_all(&mut *conn)
        .await?;
        let raw_duration = raw_start.elapsed();

        let mat_start = Instant::now();
        let mat_view =
            sqlx::query("SELECT * FROM public.average_transaction_amount")
                .fetch_all(&mut *conn)
                .await?;
        let mat_duration = mat_start.elapsed();

        assert_eq!(
            raw_sql.len(),
            mat_view.len()
        );

        println!("Time for raw SQL query: {:?}", raw_duration);
        println!("Time for materialized view query: {:?}", mat_duration);
        println!(
            "Percentage materialized view faster than raw SQL query: {:?}",
            ((raw_duration.as_secs_f64() - mat_duration.as_secs_f64())/ raw_duration.as_secs_f64())
                * 100.00
        );

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/audit_logs.sql",
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/transfers.sql",
        "../db/schema/transactions.sql",
        "../db/schema/loans.sql",
        "../db/schema/payments.sql",
        "../db/views/loans_outstanding.sql",
    ))]
    async fn test_loans_outstanding_time_difference_raw_sql_materialized_view(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_data().await;

        let mut conn = pool.acquire().await?;

        sqlx::query("REFRESH MATERIALIZED VIEW public.loans_outstanding")
            .execute(&mut *conn)
            .await?;

        let raw_start = Instant::now();
        let raw_sql = sqlx::query(
            "
            SELECT 
                users.id AS user_id,
                SUM(loans.amount) AS sum_loans_outstanding
            FROM 
                users
            JOIN 
                loans ON users.id = loans.user_id AND loans.status = 'active'
            GROUP BY 
                users.id;
            ",
        )
        .fetch_all(&mut *conn)
        .await?;
        let raw_duration = raw_start.elapsed();

        let mat_start = Instant::now();
        let mat_view =
            sqlx::query("SELECT * FROM public.loans_outstanding")
                .fetch_all(&mut *conn)
                .await?;
        let mat_duration = mat_start.elapsed();

        assert_eq!(
            raw_sql.len(),
            mat_view.len()
        );

        println!("Time for raw SQL query: {:?}", raw_duration);
        println!("Time for materialized view query: {:?}", mat_duration);
        println!(
            "Percentage materialized view faster than raw SQL query: {:?}",
            ((raw_duration.as_secs_f64() - mat_duration.as_secs_f64())/ raw_duration.as_secs_f64())
                * 100.00
        );

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/audit_logs.sql",
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/transfers.sql",
        "../db/schema/transactions.sql",
        "../db/schema/loans.sql",
        "../db/schema/payments.sql",
        "../db/views/average_transaction_amount.sql",
        "../db/views/suspicious_transactions.sql",
    ))]
    async fn test_suspicious_transactions_time_difference_raw_sql_materialized_view(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_data().await;

        let mut conn = pool.acquire().await?;

        sqlx::query("REFRESH MATERIALIZED VIEW public.suspicious_transactions")
            .execute(&mut *conn)
            .await?;

        let raw_start = Instant::now();
        let raw_sql = sqlx::query(
            "
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
            ",
        )
        .fetch_all(&mut *conn)
        .await?;
        let raw_duration = raw_start.elapsed();

        let mat_start = Instant::now();
        let mat_view =
            sqlx::query("SELECT * FROM public.suspicious_transactions")
                .fetch_all(&mut *conn)
                .await?;
        let mat_duration = mat_start.elapsed();

        assert_eq!(
            raw_sql.len(),
            mat_view.len()
        );

        println!("Time for raw SQL query: {:?}", raw_duration);
        println!("Time for materialized view query: {:?}", mat_duration);
        println!(
            "Percentage materialized view faster than raw SQL query: {:?}",
            ((raw_duration.as_secs_f64() - mat_duration.as_secs_f64())/ raw_duration.as_secs_f64())
                * 100.00
        );

        Ok(())
    }
}
