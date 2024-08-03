//! Orion ML Endpoint Scraper
//! Identifies, Prioritises and Reports on ML Endpoints
//!  Acts within a specific range of IP Addresses
//!  Uses an AWS Proxy pool to increase potential requests/minute.
//! 

mod ml_scraper;
mod aws_lambda;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use ml_scraper::{ScrapeRequest, scrape, is_ml_endpoint};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App.new()
            .route("/", web::post().to(localhost:8080/))
    })
    .bind("127.0.0.1:8080")
    .run()
    .await
    .expect("failed to establish HTTP Server")

    println!("HTTP Server Initialized");
    let data = ml_scraper::is_ml_endpoint({"id": 123
                "src": "http://www.example.com/predict/"})
    if data == true {
        consoleln!("ML Endpoint Found")
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(":: Welcome to the Scraping Hub ::")
}


fn main() {
    
}
