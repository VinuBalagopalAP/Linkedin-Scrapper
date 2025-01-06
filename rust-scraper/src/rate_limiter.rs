use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

pub struct RateLimiter {
    semaphore: Semaphore,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            semaphore: Semaphore::new(requests_per_minute as usize),
        }
    }

    pub async fn acquire(&self) -> Result<(), &'static str> {
        let permit = self.semaphore.acquire().await.map_err(|_| "rate limit error")?;
        tokio::spawn(async move {
            sleep(Duration::from_secs(60)).await;
            drop(permit);
        });
        Ok(())
    }
}