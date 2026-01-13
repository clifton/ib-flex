//! Simple API client usage example
//!
//! This is a minimal example showing the basic API client workflow.
//!
//! ## Usage
//!
//! ```bash
//! export IB_FLEX_TOKEN="your_token"
//! export IB_FLEX_QUERY_ID="your_query_id"
//! cargo run --example api_simple_usage --features api-client
//! ```

#[cfg(feature = "api-client")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ib_flex::api::FlexApiClient;
    use std::time::Duration;

    // Get credentials from environment
    let token = std::env::var("IB_FLEX_TOKEN")?;
    let query_id = std::env::var("IB_FLEX_QUERY_ID")?;

    // Create client
    let client = FlexApiClient::new(token);

    // Send request and get reference code
    let reference_code = client.send_request(&query_id).await?;
    println!("Reference code: {}", reference_code);

    // Wait for statement generation
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Get statement
    let xml = client.get_statement(&reference_code).await?;

    // Parse statement
    let statement = ib_flex::parse_activity_flex(&xml)?;

    println!("Account: {}", statement.account_id);
    println!("Trades: {}", statement.trades.items.len());

    Ok(())
}

#[cfg(not(feature = "api-client"))]
fn main() {
    eprintln!("This example requires the 'api-client' feature.");
    eprintln!("Run with: cargo run --example api_simple_usage --features api-client");
    std::process::exit(1);
}
