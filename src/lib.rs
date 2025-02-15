// NOTE: lib.rs? or another file? - as not a library
// think on general repo org oo
use chrono::{DateTime, Duration, Utc};
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::name::{en::FirstName, en::LastName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserRowAccount {
    public_id: Uuid,
    given_name: String,
    family_name: String,
    username: String,
    email: String,
    phone: String,
    created_at: DateTime<Utc>,
}

// pub struct AccountRowInsertion {
//     user_id: i32,
//     account_type: String,
//     balance: f64,
//     created_at: DateTime<Utc>,
//     num_active_cards: i32,
// }

enum AuditLogAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    TransferCreated,
    TransferUpdated,
    TransactionCreated,
    TransactionDeleted,
    PaymentCreated,
    PaymentUpdated,
    LoanCreated,
    LoanUpdated,
    CardCreated,
    CardUpdated,
    CardDeleted,
    AccountCreated,
    AccountUpdated,
    AccountDeleted,
}

impl AuditLogAction {
    fn to_string(&self) -> &'static str {
        match self {
            Self::UserCreated => "user created",
            Self::UserUpdated => "user updated",
            Self::UserDeleted => "user deleted",
            Self::TransferCreated => "transfer created",
            Self::TransferUpdated => "transfer updated",
            Self::TransactionCreated => "transaction created",
            Self::TransactionDeleted => "transaction deleted",
            Self::PaymentCreated => "payment created",
            Self::PaymentUpdated => "payment updated",
            Self::LoanCreated => "loan created",
            Self::LoanUpdated => "loan updated",
            Self::CardCreated => "card created",
            Self::CardUpdated => "card updated",
            Self::CardDeleted => "card deleted",
            Self::AccountCreated => "account created",
            Self::AccountUpdated => "account updated",
            Self::AccountDeleted => "account deleted",
        }
    }
}

pub struct AuditLogs {
    action: AuditLogAction,
    text: String,
    timestamp: DateTime<Utc>,
}

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

            let user = UserRowAccount {
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

    // async fn insert_accounts(&self) {
    //     let mut accounts_count = 1;
    //     loop {
    //         if accounts_count > 2000 {
    //             break;
    //         }

    //         let created_at = self.random_date(9, 8);

    //         let account = AccountRowInsertion {
    //             user_id: accounts_count % 3,
    //             account_type: 'something' one of each?,
    //             balance: random number,
    //             created_at,
    //             num_active_cards: 2, depends on type right?
    //         };
    //         if let Err(e) = sqlx::query(
    //             "
    //             insert into public.accounts
    //             (user_id, account_type, balance, created, num_active_cards)
    //             values ($1, $2, $3, $4, $5);
    //             ",
    //         )
    //         .bind(account.user_id)
    //         .bind(account.account_type)
    //         .bind(account.balance)
    //         .bind(account.created_at)
    //         .bind(account.num_active_cards)
    //         .execute(&self.db)
    //         .await
    //         {
    //             println!(
    //                 "Error: failed to insert row into 'accounts' - <id = {}> - <error = {:?}>",
    //                 accounts_count, e
    //             );
    //         } else {
    //             self.insert_audit_log(
    //                 accounts_count,
    //                 AuditLogAction::AccountCreated.to_string(),
    //                 format!("account_id id <{}>", accounts_count),
    //                 created_at,
    //             )
    //             .await
    //         }

    //         accounts_count += 1;
    //     }
    // }

    async fn insert_data(&self) {
        self.insert_users().await;
        // self.insert_accounts().await;
    }
}

fn main() {
    // BankSystemManager.build_schema() = not needed as in a docker file? = even needed if all
    // testing?
    // BankSystemManager.insert_data().await
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

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

        Ok(())
    }
}
