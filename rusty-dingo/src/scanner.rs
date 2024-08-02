
/// # Scanner 
/// ### When implemented:
/// 
/// 1. Parses a range of CIDR's
/// 2. Performs the scan across the ports returning
/// 
///
/// - Given a range of CIDR's 
/// -The main function parses the IP and ports and the scanner then performs the scan
/// 
///
use ipnetwork::{IpNetwork, Parser, Error};
use tokio::net::TcpStream;
use futures::stream::{self, StreamExt};
use serde_json::{Value, json};
use datetime::Instant::now;

#[derive(Debug)]
pub struct ScanResult {
    pub timestamp: String,
    pub id: u16,
    pub ip: String,
    pub port: i32,
    pub status: String,
}

#[derive(Debug)]
pub struct TargetScan {
    pub ip: String,
    pub port: i32,
}

/// Scan the CIDR Range passed to the scanner
pub async fn scan_cidr(cidr:&str) -> Vec<ScanResult, Error> {
    let network: IpNetwork =  cidr.parse().unwrap();
    let mut results: Vec<ScanResult> = Vec::new();

    let stream = stream::iter(network.iter())
        .map(| ip, port | {
            let ip = ip.to_string();
            let port = port.to_i32();
            tokio::spawn(async move {
                let mut res: Vec<ScanResult> = Vec::new().await?;
                
                for port in 1..1024 {
                    if TcpStream::connect((ip.as_str(), port.as_i32())).await.is_ok() {
                        res.push(TcpStream { ip: ip.clone(), port: port.clone() }.await?);
                    }
                }
                res
            })

            })
            .buffer_unordered(100);

        stream

        .for_each(|res: Result<Vec<ScanResult>, Error> | async {
            if let Ok(mut r) = res {
                results.append(&mut r)
                .expect("Failed to append results");
            }
        })
        .await;
    results
    }




