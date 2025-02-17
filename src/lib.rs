pub mod enums;
pub mod models;

use chrono::{DateTime, Duration, Utc};
use enums::account_type::AccountType;
use enums::audit_log_action::AuditLogAction;
use enums::audit_log_subject_table::AuditLogSubjectTable;
use enums::card_status::CardStatus;
use enums::card_type::CardType;
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
use models::transactions::{self, TransactionRowInsertion};
use models::transfer::{self, TransferRowInsertion};
use models::user::UserRowInsertion;
use rand::Rng;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

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
            if users_count > 100 {
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
            if accounts_count > 400 {
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
            if accounts_per_user == 4 {
                accounts_per_user = 0;
                current_user_id += 1;
            }
        }
    }

    async fn insert_cards(&self) {
        let mut current_account_id = 1;
        let mut is_business_account_debit_inserted = false;
        loop {
            if current_account_id > 400 {
                break;
            }

            let account_type = match current_account_id % 4 {
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
            if current_account_id > 400 {
                // NOTE: constant this and all others
                break;
            }

            let account_type = match current_account_id % 4 {
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
            if num_transfers_each_account == 5 {
                // NOTE: constant this too
                num_transfers_each_account = 0;
                current_account_id += 1;
            }
        }
    }

    async fn insert_transactions(&self) {
        let mut current_account_id = 1;
        let mut num_transactions_each_account = 0;
        loop {
            if current_account_id > 400 {
                // NOTE: constant this and all others
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
                2 => TransactionType::Withdrawel,
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
            if num_transactions_each_account == 2 {
                // NOTE: constant this too
                num_transactions_each_account = 0;
                current_account_id += 1;
            }
        }
    }

    async fn insert_data(&self) {
        self.insert_users().await;
        self.insert_accounts().await;
        self.insert_cards().await;
        self.insert_transfers().await;
        self.insert_transactions().await;
    }
}

#[cfg(test)]
mod test {
    use sqlx::{PgPool, Row};

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
        "../db/schema/audit_logs.sql",
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/cards.sql",
        "../db/schema/transfers.sql",
        "../db/schema/transactions.sql",
        // "../db/schema/loans.sql",
        // "../db/schema/payments.sql",
    ))]
    async fn test_all_insertions(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;
        bank_system_manager.insert_accounts().await;
        bank_system_manager.insert_cards().await;
        bank_system_manager.insert_transfers().await;
        bank_system_manager.insert_transactions().await;

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

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(users.len(), 100);
        assert_eq!(accounts.len(), 400);
        assert_eq!(cards.len(), 400);
        assert_eq!(transfers.len(), 1000);
        assert_eq!(transactions.len(), 400);
        assert_eq!(audit_logs.len(), 2300);

        Ok(())
    }
}
