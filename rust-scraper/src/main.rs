use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

mod models;
mod scraper;

#[derive(Deserialize)]
struct ScrapeRequest {
   url: String,
}

async fn handle_scrape(req: web::Json<ScrapeRequest>) -> HttpResponse {
   match scraper::scrape_profile(&req.url).await {
       Ok(profile) => HttpResponse::Ok().json(profile),
       Err(_) => HttpResponse::InternalServerError().finish()
   }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   println!("Server running at http://localhost:8080");
   HttpServer::new(|| {
       App::new()
           .route("/scrape", web::post().to(handle_scrape))
           .wrap(actix_web::middleware::Logger::default())
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}