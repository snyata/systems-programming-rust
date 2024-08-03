pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

#[actix_rt::test]
async fn http_server_initializes_successfully() {
    use actix_web::{test, App, web, HttpResponse};
    use ml_scraper::is_ml_endpoint;

    let srv = test::start(|| {
        App::new()
            .route("/", web::post().to(|| HttpResponse::Ok()))
    });

    let response = srv.post("/").send().await.unwrap();
    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn http_server_fails_to_bind() {
    use actix_web::{App, HttpServer};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    drop(listener); // Release the port so the next bind attempt fails

    let result = HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080");

    assert!(result.is_err());
}

#[actix_rt::test]
async fn identifies_predict_as_valid_ml_endpoint() {
    use actix_web::{test, web, App};
    use ml_scraper::is_ml_endpoint;
    use serde_json::json;

    let mut app = test::init_service(App::new().route("/is_ml_endpoint", web::post().to(is_ml_endpoint))).await;
    let req = test::TestRequest::post()
        .uri("/is_ml_endpoint")
        .set_json(&json!("/predict"))
        .to_request();
    let resp: bool = test::call_and_read_body_json(&mut app, req).await;

    assert_eq!(resp, true);
}

#[actix_rt::test]
async fn handles_empty_input_gracefully() {
    use actix_web::{test, web, App};
    use ml_scraper::is_ml_endpoint;
    use serde_json::json;

    let mut app = test::init_service(App::new().route("/is_ml_endpoint", web::post().to(is_ml_endpoint))).await;
    let req = test::TestRequest::post()
        .uri("/is_ml_endpoint")
        .set_json(&json!(""))
        .to_request();
    let resp: bool = test::call_and_read_body_json(&mut app, req).await;

    assert_eq!(resp, false);
}
