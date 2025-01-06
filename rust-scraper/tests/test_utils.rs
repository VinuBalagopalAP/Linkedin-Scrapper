// rust-scraper/tests/test_utils.rs
pub mod test_utils {
    use std::fs;

    pub fn load_test_html() -> String {
        fs::read_to_string("tests/fixtures/profile.html").expect("Failed to load test HTML")
    }
}
