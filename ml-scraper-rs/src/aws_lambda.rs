/// # AWS Lambda Proxy Pool
///  - Implements the Scraping logic on your set of targets
///  - Serverless with AWS Proxy Pool
/// 

use aws_sdk_lambda::{Client, Error};
use serde::Serialize;

#[derive(Serialize)]
struct ProxyRequest {
    urls: Vec<String>
}

pub async fn get_proxy_ips(urls: Vec<String>) -> Result<Vec<String>, Error> {
    let client = Client::new();
    let payload = serde_json::to_vec(&ProxyRequest { urls })?;
    
    let response = client
        .invoke()
        .function_name("LAMBDA_NAME")
        .payload(payload.into())
        .send()
        .await?
    
    let result: Vec<String> = serde_json::from_slice(&response
        .payload.unwrap()
        .as_ref())?;
    Ok(result)
}