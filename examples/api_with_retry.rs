//! API client with automatic retry example
//!
//! This example demonstrates using the automatic retry feature to handle
//! cases where the statement is not ready immediately.
//!
//! ## Usage
//!
//! ```bash
//! export IB_FLEX_TOKEN="your_token"
//! export IB_FLEX_QUERY_ID="your_query_id"
//! cargo run --example api_with_retry --features api-client
//! ```

#[cfg(feature = "api-client")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ib_flex::api::FlexApiClient;
    use std::time::Duration;

    // Get credentials
    let token = std::env::var("IB_FLEX_TOKEN")?;
    let query_id = std::env::var("IB_FLEX_QUERY_ID")?;

    // Create client
    let client = FlexApiClient::new(token);

    // Send request
    println!("Sending request...");
    let reference_code = client.send_request(&query_id)?;
    println!("Reference code: {}", reference_code);

    // Get statement with automatic retry
    // Will retry up to 10 times with 2-second delays if not ready
    println!("Fetching statement (with automatic retry)...");
    let xml = client.get_statement_with_retry(&reference_code, 10, Duration::from_secs(2))?;

    println!("Received statement ({} bytes)", xml.len());

    // Parse and display
    let statement = ib_flex::parse_activity_flex(&xml)?;
    println!("\nStatement Details:");
    println!("  Account: {}", statement.account_id);
    println!("  Period: {} to {}", statement.from_date, statement.to_date);
    println!("  Trades: {}", statement.trades.items.len());
    println!("  Positions: {}", statement.positions.items.len());
    println!(
        "  Cash Transactions: {}",
        statement.cash_transactions.items.len()
    );

    Ok(())
}

#[cfg(not(feature = "api-client"))]
fn main() {
    eprintln!("This example requires the 'api-client' feature.");
    eprintln!("Run with: cargo run --example api_with_retry --features api-client");
    std::process::exit(1);
}
