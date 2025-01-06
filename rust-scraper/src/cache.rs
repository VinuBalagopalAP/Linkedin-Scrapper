// src/cache.rs
use crate::models::Profile;
use moka::sync::Cache;
use std::time::Duration;

pub struct ProfileCache {
    cache: Cache<String, Profile>,
}

impl ProfileCache {
    pub fn new() -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600))
                .build(),
        }
    }

    pub fn get(&self, url: &str) -> Option<Profile> {
        self.cache.get(url)
    }

    pub fn set(&self, url: String, profile: Profile) {
        self.cache.insert(url, profile);
    }
}
