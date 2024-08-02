//!  Une Swiss Army Scanner
//!  A compilation of useful scanning tools written in Rust
//!  Compilation of exercises accumulated over learning experience.
//! Includes Scanner, Password_cracker (simple), sqlite3 DB, Http Server, Keylogger


mod scanner;
mod password_cr;
mod db;
mod http_server;
mod keylogger;

use clap::{App, Arg};
use colored::*;
use tokio::time::{sleep, Duration};
use scanner::scan_cidr;
use db::Db;
use std::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use chrono::Utc;

use tokio::time::{sleep, Duration};
use db::Db;

use datetime::Instant::now;

/// Main Begins the scanning across the cider range,
/// starts related services.
/// 
/// Example: Run pw cracker
#[tokio::main]
async fn main() {
    // Correctly generate a timestamp using `chrono::Utc`
    let timestamp = Utc::now().to_rfc3339();

    let matches = App::new("Le Dingo Scanner")
        .version("0.1.0")
        .about("Experimenting with Cyber tools in Rust locally")
        .arg(
            Arg::with_name("crack")
                .long("crack")
                .value_name("HASH")
                .help("Run the password cracker against the provided hash")
                .takes_value(true)
        )
        .get_matches();

    if let Some(hash) = matches.value_of("crack") {
        println!("{}", "Running password cracker...".green());
        password_cracker::crack_passwords(hash);
        return;
    }

    // Start Scanner and Write to DB
    let db = Db::new("routine_scans.db").await.unwrap();
    db.init().await.unwrap();

    let cidr_ranges: Vec<&str> = vec!["192.168.1.0/24", "10.0.0.0/24"];
    println!("{}", "Starting network scan...".green());

    let file_name = format!("{}_scan_results.txt", timestamp);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_name)
        .await
        .expect("File Path Creation Failed");

    // Loop through CIDR ranges
    for cidr in cidr_ranges {
        match scan_cidr(cidr).await {
            Ok(results) => {
                println!("Results: {:?}", results); // Print results
                
                for result in results {
                    db.insert_scan_results(&result.timestamp, &result.ip, result.port, &result.status).await.unwrap();

                    // Write to file path
                    let entry = format!("{} | {} | {} | {}\n", result.timestamp, result.ip, result.port, result.status);
                    file.write_all(entry.as_bytes()).await.unwrap();
                }

                println!("Results have been written to the database and file.");
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Start HTTP Server
    let db_pool = db.get_pool();
    tokio::spawn(async move {
        http_server::run_server(db_pool).await;
    });

    // Start Keylogger
    tokio::spawn(async move {
        keylogger::start_keylogger().await;
    });

    // Sleep to keep the program running
    sleep(Duration::from_secs(60 * 10)).await;
    
    //Start HTTP Server
    tokio::spawn(http_server::run_server());

    //Start Keylogger
    tokio::spawn(keylogger::start_keylogger());

    //Sleep to ensure all tasks are completed asynchronously
    sleep(Duration::from_secs(1)).await;

    //Run pw cracker example
    //password_cr::crack_passwords();
}
