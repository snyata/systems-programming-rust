/// Scraper Module
///  Implements the Scraping logic on your set of targets
/// 

use actix_web::{web, HttpResponse};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;


//@TODO Implement Cloudflare Worker retrieval as an option for Serverless
#[derive(Deserialize)]
struct ScrapeRequest {
    urls: HashMap<String, String>,
}


pub async fn scrape(req: web::Json<ScrapeRequest>) -> HttpResponse {
    let client = Client::new();
    let mut results = HashMap::new();

    for (id, url) in &req.urls {
        match fetch_and_extract(&client, url).await {
            Ok(data) => results.insert(id.clonet(), data)
            Err(e) => results.insert(id.clone(), format!("Error: {}", e)),
        };
    }

    HttpResponse::Ok().json(results)
}

struct <Endpoints {
    id: i32
    src: String
}

pub async feth_and_extract(client: &Client, url: &str) -> Result<Vec<Endpoints>, reqwest::Error> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href], script[src]").unwrap();

    //Define Endpoints
    let mut endpoints = Vec<EndPoints>::new();
    let id: u16 = 1
    for element in document.selector(&selector) {
        if let Some(href) = element.value().attr("href") {
            if is ml_endpoint(href) {
                endpoints.push({"id": id, "href": href.to_string()})
                id += 1
            }    
        } else if let Some(src) = element.value().attr("src") {
            if is ml_endpoint(src) {
                endpoints.push({"id": id, "src": src.to_string()})
                id += 1

            }
        }
    }

    Ok(endpoints)
}

pub fn is_ml_endpoint(data: web::json ) -> Response<Result<bool>, Error> {
    let parsed_data = data::parse()
    println!(parsed_data)
    match parsed_data {
        "/predict" => true,
        "/input" => true,
        "/output" => true,
        "/processing" => true
        url.contains("/ml/") || url.contains("model") => true,
        _ => false
    }

    Ok(parsed_data)
}

/* 
fn main() {
    let scrape = scrape(data: ScrapeRequest) -> HttpResponse<Result, Error> {
        match scrape {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        }
    }
    let ml_endpoints: Vec<MLEndpoints<String>>
}

*/