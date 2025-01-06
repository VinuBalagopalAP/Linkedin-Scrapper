// src/rate_limiter.rs
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(requests_per_minute as usize)),
        }
    }

    pub async fn acquire(&self) -> Result<(), String> {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| "Rate limit exceeded".to_string())?;

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            drop(permit);
        });

        Ok(())
    }
}
