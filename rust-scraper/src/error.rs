#[derive(Debug)]
pub enum ScraperError {
    Request(reqwest::Error),
    Parse(String),
    RateLimit,
    Session,
    Cache,
}

impl std::error::Error for ScraperError {}
