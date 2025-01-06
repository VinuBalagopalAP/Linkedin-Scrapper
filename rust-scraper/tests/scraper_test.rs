// tests/scraper_test.rs
use rust_scraper::scraper::scrape_profile;

#[tokio::test]
async fn test_profile_scraping() {
    // Create test data
    let test_url = "https://linkedin.com/in/test-profile";

    // Test scraping
    let result = scrape_profile(test_url).await;

    if let Ok(profile) = result {
        // Verify profile data
        assert_eq!(profile.first_name, "John");
        assert_eq!(profile.last_name, "Doe");
    } else {
        panic!("Failed to scrape profile: {:?}", result);
    }
}
