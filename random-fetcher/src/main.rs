use serde::Deserialize;
use reqwest::{Result, Value, blocking::get};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Quote {
    content: String,
    author: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://api.quotable.io/random")?
    let json_response = response.json::<Value>();
    
    println!("\"{}\" - {}", json_response.to_string());
    
    Ok(())

    }
