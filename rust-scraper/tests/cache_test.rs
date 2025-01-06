use rust_scraper::cache::ProfileCache;
use rust_scraper::models::Profile;

#[test]
fn test_cache_operations() {
    let cache = ProfileCache::new();
    let test_profile = Profile {
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        job_title: "Developer".to_string(),
        summary: "Test summary".to_string(),
        experience: vec![],
        skills: vec![],
    };

    cache.set("test_url".to_string(), test_profile);
    let result = cache.get("test_url");
    assert!(result.is_some());
}
