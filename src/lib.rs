// nneed a full catch up with testing too pre card shenaginics
pub mod enums;
pub mod models;

use chrono::{DateTime, Duration, Utc};
use enums::account_type::AccountType;
use enums::audit_log_action::AuditLogAction;
// use enums::card_status::CardStatus;
// use enums::card_type::CardType;
// use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::name::{en::FirstName, en::LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use models::account::AccountRowInsertion;
// use models::card::CardRowInsertion;
use models::user::UserRowInsertion;
use rand::Rng;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

struct BankSystemManager {
    db: Pool<Postgres>,
}

impl BankSystemManager {
    fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    fn random_date(&self, lower_limit: i64, upper_limit: i64) -> DateTime<Utc> {
        let now = Utc::now();
        let lower = now - Duration::weeks(lower_limit);
        let upper = now - Duration::weeks(upper_limit);
        let random_seconds =
            rand::random::<i64>() % (upper.signed_duration_since(lower).num_seconds());
        lower + Duration::seconds(random_seconds)
    }

    async fn insert_audit_log(
        &self,
        user_id: i32,
        action: &str,
        details: String,
        created_at: DateTime<Utc>,
    ) {
        if let Err(e) = sqlx::query(
            "
            INSERT INTO public.audit_logs 
            (user_id, action, details, created_at)
            VALUES ($1, $2, $3, $4);
            ",
        )
        .bind(user_id)
        .bind(action)
        .bind(details)
        .bind(created_at)
        .execute(&self.db)
        .await
        {
            println!(
                 "Error: failed to insert row into 'audit_logs' - <user_id = {}> - <action = {}> - <error = {:?}>",
                 user_id, action, e
             );
        }
    }

    async fn insert_users(&self) {
        let mut users_count = 1;
        loop {
            if users_count > 1000 {
                break;
            }

            let created_at = self.random_date(10, 9);

            let user = UserRowInsertion {
                public_id: Uuid::new_v4(),
                given_name: FirstName().fake(),
                family_name: LastName().fake(),
                username: Username().fake(),
                email: SafeEmail().fake(),
                phone: PhoneNumber().fake(),
                created_at,
            };
            if let Err(e) = sqlx::query(
                "
                INSERT INTO public.users 
                (public_id, given_name, family_name, username, email, phone, created_at) 
                VALUES ($1, $2, $3, $4, $5, $6, $7);
                ",
            )
            .bind(user.public_id)
            .bind(user.given_name)
            .bind(user.family_name)
            .bind(user.username)
            .bind(user.email)
            .bind(user.phone)
            .bind(user.created_at)
            .execute(&self.db)
            .await
            {
                println!(
                    "Error: failed to insert row into 'users' - <id = {}> - <error = {:?}>",
                    users_count, e
                );
            } else {
                self.insert_audit_log(
                    users_count,
                    AuditLogAction::UserCreated.to_string(),
                    format!("user id <{}>", users_count),
                    created_at,
                )
                .await
            }

            users_count += 1;
        }
    }

    async fn insert_accounts(&self) {
        let mut accounts_count = 1;
        let mut current_user_id = 1;
        let mut accounts_per_user = 0;
        loop {
            if accounts_count > 4000 {
                break;
            }

            let account_type = match accounts_per_user {
                0 => AccountType::Checking,
                1 => AccountType::Savings,
                2 => AccountType::Credit,
                3 => AccountType::Business,
                _ => AccountType::Checking,
            };
            let created_at = self.random_date(9, 8);
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
            if let Err(e) = sqlx::query(
                "
                insert into public.accounts
                (user_id, account_type, balance, created_at, num_active_cards)
                values ($1, $2, $3, $4, $5);
                ",
            )
            .bind(account.user_id)
            .bind(account.account_type)
            .bind(account.balance)
            .bind(account.created_at)
            .bind(account.num_active_cards)
            .execute(&self.db)
            .await
            {
                println!(
                    "Error: failed to insert row into 'accounts' - <id = {}> - <error = {:?}>",
                    accounts_count, e
                );
            } else {
                self.insert_audit_log(
                    current_user_id,
                    AuditLogAction::AccountCreated.to_string(),
                    format!("account id <{}>", accounts_count),
                    created_at,
                )
                .await
            }

            accounts_count += 1;
            accounts_per_user += 1;
            if accounts_per_user == 4 {
                accounts_per_user = 0;
                current_user_id += 1;
            }
        }
    }

    // async fn insert_cards(&self) {
    //     let mut current_account_id = 1;
    //     let mut current_card_type = CardType::Debit;
    //     loop {
    //         if current_account_id > 4000 {
    //             break;
    //         }

    //         let account_type = match current_account_id % 4 {
    //             0 => AccountType::Checking,
    //             1 => AccountType::Savings,
    //             2 => AccountType::Credit,
    //             0 => AccountType::Business,
    //             _ => AccountType::Checking, // NOTE: correct?
    //         };
    //         let created_at = self.random_date(-6, -12);

    //         let card = CardRowInsertion {
    //             account_id: current_account_id,
    //             card_number: CreditCardNumber().fake(),
    //             card_type: current_card_type.to_string(),
    //             expiration_date: created_at,
    //             status: CardStatus::Active.to_string(),
    //         };
    //         if let Err(e) = sqlx::query(
    //             "
    //             insert into public.cards
    //             (account_id, card_number, card_type, expiration_date, status)
    //             values ($1, $2, $3, $4, $5);
    //             ",
    //         )
    //         .bind(card.account_id)
    //         .bind(card.card_number)
    //         .bind(card.card_type)
    //         .bind(card.expiration_date)
    //         .bind(card.status)
    //         .execute(&self.db)
    //         .await
    //         {
    //             println!(
    //                 "Error: failed to insert row into 'card' - <card_id = {}> - <error = {:?}>",
    //                 // NOTE: showing id is silly! it won't be the id as it won't have been entered?
    //                 1,
    //                 e
    //             );
    //         } else {
    //             // self.insert_audit_log( // Audit always links to user_id? = no? = all feeds into
    //             //                        // audit = how keep track if something that audits that
    //             //                        then disapears? how keep track?
    //             //     current_user_id,
    //             //     AuditLogAction::AccountCreated.to_string(),
    //             //     format!("account id <{}>", accounts_count),
    //             //     created_at,
    //             // )
    //             // .await
    //         }

    //         if account_type == AccountType::Checking {
    //             account_type = AccountType::Credit;
    //         }
    //     }
    // }

    async fn insert_data(&self) {
        self.insert_users().await;
        self.insert_accounts().await;
        // self.insert_cards().await;
    }
}

#[cfg(test)]
mod test {
    use sqlx::{PgPool, Row};

    use super::*;

    #[sqlx::test(fixtures("../db/schema/users.sql", "../db/schema/audit_logs.sql"))]
    async fn test_users_created(pool: PgPool) -> sqlx::Result<()> {
        let bank_system_manager = BankSystemManager::new(pool.clone());

        bank_system_manager.insert_users().await;

        let mut conn = pool.acquire().await?;

        let users = sqlx::query("SELECT * FROM public.users")
            .fetch_all(&mut *conn)
            .await?;

        let audit_logs = sqlx::query("SELECT * FROM public.audit_logs")
            .fetch_all(&mut *conn)
            .await?;

        assert_eq!(users.len(), 1000);
        assert_eq!(audit_logs.len(), 1000);
        let user_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == AuditLogAction::UserCreated
            })
            .collect();
        assert_eq!(user_insertions.len(), 1000);

        Ok(())
    }

    #[sqlx::test(fixtures(
        "../db/schema/users.sql",
        "../db/schema/accounts.sql",
        "../db/schema/audit_logs.sql"
    ))] // NOTE: check num cards + type of accounts?
    async fn test_accounts_created(pool: PgPool) -> sqlx::Result<()> {
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

        assert_eq!(accounts.len(), 4000);
        assert_eq!(audit_logs.len(), 5000); // NOTE: need to clear down audits between tests? -
                                            // also after all to allow tests run on after another
                                            // no docker restar
        let account_insertions: Vec<_> = audit_logs
            .iter()
            .filter(|log| {
                AuditLogAction::from_string(log.get::<String, _>("action").as_str())
                    == AuditLogAction::AccountCreated
            })
            .collect();
        assert_eq!(account_insertions.len(), 4000);

        Ok(())
    }
}
