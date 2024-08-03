# Web Scraper with Actix and AWS Lambda

### DEV NOTES 030824:
- Ensure that the integration with AWS Lambda functions as a proxy pool works seamlessly.
- Implement robust error handling and asynchronous programming for web scraping.
- Customize the scraper to extract machine learning endpoints and models.

**Welcome to the Web Scraper Project by Synavate Labs**

```plaintext
:::::::::::::::::::::::::::::::::::::::::::::::::::::::
:::::       ____   _____   _____  ____   _   _       ::
:::::/ __ \ |  __ \ |_   _|/ __ \ | \ | |            ::
:::::| |  | || |__) |  | | | |  | ||  \| |           :: 
:::::| |  | ||  _  /   | | | |  | || . ` |           :: 
:::::| |__| || | \ \  _| |_| |__| || |\  |           :: 
:::::\____/ |_|  \_\|_____|\____/ |_| \_|            :: 
::::::::::::::::::::::::::::::::::::::::::::::::::::::::

```

### Features:

🚀 Extract machine learning endpoints and models from websites
🔧 Use AWS Lambda functions as a proxy pool for IP addresses
🧪 More to come, this is a robust web scraping solution
❌ Built-in error handling and asynchronous programming

### Config
`Cargo.toml`
```toml
[package]
name = "web_scraper"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
aws-sdk-lambda = "0.0.25-alpha"
tokio = { version = "1", features = ["full"] }
scraper = "0.13"
thiserror = "1.0"
```

### Example:

`cli`
```
# Set up logging

export RUST_LOG=debug
export RUST_LOG_STYLE=auto
```

### Llamba Configuration:

`src/aws_lambda.rs`
```rust
use aws_sdk_lambda::{Client, Error};
use serde::Serialize;

#[derive(Serialize)]
struct ProxyRequest {
    urls: Vec<String>,
}

pub async fn get_proxy_ips(urls: Vec<String>) -> Result<Vec<String>, Error> {
    let client = Client::new();
    let payload = serde_json::to_vec(&ProxyRequest { urls })?;

    let response = client
        .invoke()
        .function_name("your_lambda_function_name")
        .payload(payload.into())
        .send()
        .await?;

    let result: Vec<String> = serde_json::from_slice(&response.payload.unwrap().as_ref())?;
    Ok(result)
}
```

----------------
Thanks for using the Web Scraper project! 👋

**- Synavate Labs**
***core@synavate.tech***