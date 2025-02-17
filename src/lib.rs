pub mod enums;
pub mod models;

use chrono::{DateTime, Duration, Utc};
use enums::account_type::AccountType;
use enums::audit_log_action::AuditLogAction;
use enums::audit_log_subject_table::AuditLogSubjectTable;
use enums::card_status::CardStatus;
use enums::card_type::{self, CardType};
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::name::{en::FirstName, en::LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use models::account::AccountRowInsertion;
use models::card::CardRowInsertion;
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

    async fn insert_data(&self) {
        self.insert_users().await;
        self.insert_accounts().await;
        self.insert_cards().await;
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
}
