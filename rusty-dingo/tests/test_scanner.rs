#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}


#[tokio::test]
async fn test_scan_cidr_valid_range() {
    use scanner_project_rs::scanner::scan_cidr;
    use ipnetwork::IpNetwork;
    use std::net::IpAddr;

    let cidr = "192.168.1.0/24";
    let results = scan_cidr(cidr).await.unwrap();

    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_scan_cidr_invalid_string() {
    use scanner_project_rs::scanner::scan_cidr;

    let invalid_cidr = "invalid_cidr";
    let result = scan_cidr(invalid_cidr).await;

    assert!(result.is_err(), "Result should be an error for an invalid CIDR string");
}