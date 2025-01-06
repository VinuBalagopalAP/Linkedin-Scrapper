// tests/integration_test.rs
use rust_scraper::{cache::ProfileCache, models::Profile, rate_limiter::RateLimiter};

#[tokio::test]
async fn test_full_flow() {
    // Test setup
    let cache = ProfileCache::new();
    let limiter = RateLimiter::new(10);

    assert!(limiter.acquire().await.is_ok());

    let test_profile = Profile {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        job_title: "Engineer".to_string(),
        summary: "Test".to_string(),
        experience: vec![],
        skills: vec![],
    };

    let test_url = "https://linkedin.com/test".to_string();
    cache.set(test_url.clone(), test_profile);

    let cached_profile = cache.get(&test_url);
    assert!(cached_profile.is_some());
}
