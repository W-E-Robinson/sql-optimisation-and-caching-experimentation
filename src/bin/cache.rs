use moka::future::Cache;
use std::time::Duration;

pub struct FinanceCache {
    account_balance: Cache<u32, f32>,
    user_outstanding_loans: Cache<u32, f32>,
}

impl FinanceCache {
    pub fn new() -> Self {
        Self {
            account_balance: Cache::builder()
                .time_to_live(Duration::from_secs(3_600)) // 1 hour
                .time_to_idle(Duration::from_secs(1_800)) // 30 mins
                .max_capacity(5000)
                .build(),
            user_outstanding_loans: Cache::builder()
                .time_to_live(Duration::from_secs(86_400)) // 1 day
                .time_to_idle(Duration::from_secs(43_200)) // 12 hours
                .max_capacity(1000)
                .build(),
        }
    }

    pub async fn get_account_balance(&self, account_id: &u32) -> Option<f32> {
        self.account_balance.get(account_id).await
    }

    pub async fn get_user_outstanding_loans(&self, user_id: &u32) -> Option<f32> {
        self.user_outstanding_loans.get(user_id).await
    }

    pub async fn set_account_balance(&self, account_id: u32, value: f32) {
        self.account_balance.insert(account_id, value).await
    }

    pub async fn set_user_outstanding_loans(&self, user_id: u32, value: f32) {
        self.user_outstanding_loans.insert(user_id, value).await
    }

    pub async fn invalidate_account_balance(&self, account_id: &u32) -> Result<(), String> {
        let account_balance = self.account_balance.get(account_id).await;
        if account_balance.is_none() {
            return Err(format!(
                "Error: There is no account_balance cache entry for <account_id={}> to invalidate.",
                account_id
            ));
        }
        self.account_balance.invalidate(account_id).await;

        Ok(())
    }

    pub async fn invalidate_user_outstanding_loans(&self, user_id: &u32) -> Result<(), String> {
        let user_outstanding_loans = self.user_outstanding_loans.get(user_id).await;
        if user_outstanding_loans.is_none() {
            return Err(format!("Error: There is no user_outstanding_loans cache entry for <user_id={}> to invalidate.", user_id));
        }
        self.user_outstanding_loans.invalidate(user_id).await;

        Ok(())
    }
}

#[cfg(test)]
mod test_account_balance {
    use super::*;

    #[tokio::test]
    async fn test_it_caches_account_balance() {
        let cache = FinanceCache::new();
        cache.set_account_balance(1, 1000.00).await;

        assert_eq!(Some(1000.00), cache.get_account_balance(&1).await)
    }

    #[tokio::test]
    async fn test_it_gets_empty_account_balance() {
        let cache = FinanceCache::new();

        assert_eq!(None, cache.get_account_balance(&1).await)
    }

    #[tokio::test]
    async fn test_it_invalidates_account_balance() {
        let cache = FinanceCache::new();
        cache.set_account_balance(1, 1000.00).await;

        cache.invalidate_account_balance(&1).await;

        assert_eq!(None, cache.get_account_balance(&1).await)
    }

    #[tokio::test]
    async fn test_it_errors_on_invalidation_when_incorrect_account_id() {
        let cache = FinanceCache::new();

        let result = cache.invalidate_account_balance(&1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Error: There is no account_balance cache entry for <account_id=1> to invalidate."
                .to_string()
        );
    }
}

#[cfg(test)]
mod test_user_outstanding_loans {
    use super::*;

    #[tokio::test]
    async fn test_it_caches_user_outstanding_loans() {
        let cache = FinanceCache::new();
        cache.set_user_outstanding_loans(1, 5000.00).await;

        assert_eq!(Some(5000.00), cache.get_user_outstanding_loans(&1).await)
    }

    #[tokio::test]
    async fn test_it_gets_empty_user_outstanding_loans() {
        let cache = FinanceCache::new();

        assert_eq!(None, cache.get_user_outstanding_loans(&1).await)
    }

    #[tokio::test]
    async fn test_it_invalidates_user_outstanding_loans() {
        let cache = FinanceCache::new();
        cache.set_user_outstanding_loans(1, 5000.00).await;

        cache.invalidate_user_outstanding_loans(&1).await;

        assert_eq!(None, cache.get_user_outstanding_loans(&1).await)
    }

    #[tokio::test]
    async fn test_it_errors_on_invalidation_when_incorrect_user_id() {
        let cache = FinanceCache::new();

        let result = cache.invalidate_user_outstanding_loans(&1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Error: There is no user_outstanding_loans cache entry for <user_id=1> to invalidate."
                .to_string()
        );
    }
}

// NOTE
// still need to apply all this and do manual invalidation
