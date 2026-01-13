//! Example: Fetch FLEX statement using the Web Service API
//!
//! This example demonstrates how to programmatically fetch FLEX statements
//! from Interactive Brokers using the FLEX Web Service API.
//!
//! ## Setup
//!
//! 1. Log in to IB Account Management
//! 2. Navigate to: Reports → Settings → FlexWeb Service
//! 3. Generate a FLEX Web Service token
//! 4. Create a FLEX query and note the Query ID
//!
//! ## Usage
//!
//! Set environment variables and run:
//!
//! ```bash
//! export IB_FLEX_TOKEN="your_token_here"
//! export IB_FLEX_QUERY_ID="your_query_id_here"
//! cargo run --example fetch_flex_statement --features api-client
//! ```
//!
//! ## Features Required
//!
//! This example requires the `api-client` feature:
//!
//! ```toml
//! [dependencies]
//! ib-flex = { version = "0.1", features = ["api-client"] }
//! ```

#[cfg(feature = "api-client")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ib_flex::api::FlexApiClient;
    use std::time::Duration;

    println!("=== FLEX Web Service API Example ===\n");

    // Get credentials from environment
    let token = std::env::var("IB_FLEX_TOKEN").map_err(|_| {
        "IB_FLEX_TOKEN environment variable not set.\n\
         Get your token from: IB Account Management → Reports → Settings → FlexWeb Service"
    })?;

    let query_id = std::env::var("IB_FLEX_QUERY_ID").map_err(|_| {
        "IB_FLEX_QUERY_ID environment variable not set.\n\
         Create a FLEX query in: IB Account Management → Reports → Flex Queries"
    })?;

    println!("Token: {}...", &token[..token.len().min(10)]);
    println!("Query ID: {}\n", query_id);

    // Create API client
    let client = FlexApiClient::new(token);

    // Step 1: Send request
    println!("Step 1: Sending FLEX query request...");
    let reference_code = client.send_request(&query_id).await?;
    println!("✓ Reference code received: {}\n", reference_code);

    // Step 2: Get statement with automatic retry
    println!("Step 2: Retrieving statement (will retry if not ready)...");
    let xml = client.get_statement_with_retry(&reference_code, 10, Duration::from_secs(2)).await?;
    println!("✓ Statement received ({} bytes)\n", xml.len());

    // Step 3: Parse the statement
    println!("Step 3: Parsing statement...");
    let statement = ib_flex::parse_activity_flex(&xml)?;
    println!("✓ Statement parsed successfully\n");

    // Display summary
    println!("=== Statement Summary ===\n");
    println!("Account ID: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!();

    println!("Content:");
    println!("  Trades: {}", statement.trades.items.len());
    println!("  Positions: {}", statement.positions.items.len());
    println!(
        "  Cash Transactions: {}",
        statement.cash_transactions.items.len()
    );
    println!(
        "  Corporate Actions: {}",
        statement.corporate_actions.items.len()
    );
    println!("  Securities Info: {}", statement.securities_info.items.len());
    println!(
        "  Conversion Rates: {}",
        statement.conversion_rates.items.len()
    );
    println!();

    // Show some trade details if available
    if !statement.trades.items.is_empty() {
        println!("\n=== Recent Trades (up to 5) ===\n");
        for (i, trade) in statement.trades.items.iter().take(5).enumerate() {
            println!("Trade {}:", i + 1);
            println!("  Symbol: {}", trade.symbol);
            println!("  Date: {}", trade.trade_date);
            if let Some(ref buy_sell) = trade.buy_sell {
                println!("  Side: {:?}", buy_sell);
            }
            if let Some(quantity) = trade.quantity {
                println!("  Quantity: {}", quantity);
            }
            if let Some(price) = trade.price {
                println!("  Price: {}", price);
            }
            println!("  Commission: {}", trade.commission);
            println!();
        }
    }

    // Calculate total P&L if available
    let total_realized_pnl: rust_decimal::Decimal = statement
        .trades
        .items
        .iter()
        .filter_map(|t| t.fifo_pnl_realized)
        .sum();

    if total_realized_pnl != rust_decimal::Decimal::ZERO {
        println!("=== P&L Summary ===\n");
        println!("Total Realized P&L: ${}", total_realized_pnl);
    }

    println!("\n✓ Done!");

    Ok(())
}

#[cfg(not(feature = "api-client"))]
fn main() {
    eprintln!("This example requires the 'api-client' feature.");
    eprintln!("Run with: cargo run --example fetch_flex_statement --features api-client");
    std::process::exit(1);
}
