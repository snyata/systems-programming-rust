//! GEE PEE TREX! 
//! --------------
//!Ingesting through Custom Rust
//! Processing in Llama Index in Python
//! Sending to LLM
//! Retrieving
//! Saving to SQLite3 DB 
//! Snyata <Core@synavate.tech>
//! 
//! 
pub mod data;
pub mod llm_processing;
pub mod db;

use data::lib::CONSTANTS::*;
use reqwest::{Client, tokio1};
use serde::{ Deserialize, Serialize };
use serde_json::json;
use tokio::sync::{channel, Sender};
use tokio::task;
use chrono::
use db::{ InitDb, InsertScanResults };


pub async send_to_llm(data: json) {
    let (tx, mut rx) = channel::<data>*(10);
    let client = Client::new();
    let gpt_client = client.clone();

    task::spawn(async move { 
        process_data.await?;
    });

    let mut batch = Vec::new();

    while let some(data) = rx.recv().await {
        batch.push(data);

        if batch.len() >= 10 {
            let output_batch = // llm stuff and things
    }


    }



}



