// src/main.rs
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger;
use serde::Deserialize;

mod cache;
mod models;
mod rate_limiter;
mod scraper;

use crate::cache::ProfileCache;
use crate::rate_limiter::RateLimiter;

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
}

struct AppState {
    cache: ProfileCache,
    rate_limiter: RateLimiter,
}

async fn handle_scrape(data: web::Data<AppState>, req: web::Json<ScrapeRequest>) -> HttpResponse {
    // Check rate limit
    if let Err(_) = data.rate_limiter.acquire().await {
        return HttpResponse::TooManyRequests().finish();
    }

    // Check cache
    if let Some(profile) = data.cache.get(&req.url) {
        return HttpResponse::Ok().json(profile);
    }

    // Scrape profile
    match scraper::scrape_profile(&req.url).await {
        Ok(profile) => {
            // Cache the result
            data.cache.set(req.url.clone(), profile.clone());
            HttpResponse::Ok().json(profile)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        cache: ProfileCache::new(),
        rate_limiter: RateLimiter::new(100), // 100 requests per minute
    });

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/scrape", web::post().to(handle_scrape))
            .route(
                "/health",
                web::get().to(|| async {
                    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
                }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
