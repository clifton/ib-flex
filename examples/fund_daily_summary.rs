//! Example: Fetch Fund Daily FLEX statement and print summary statistics
//!
//! This example demonstrates fetching a FLEX statement using credentials
//! from a .env file and displaying portfolio summary statistics.
//!
//! ## Setup
//!
//! Create a `.env` file in the project root with:
//! ```text
//! FUND_FLEX_TOKEN=your_flex_token_here
//! FUND_FLEX_DAILY_QUERY_ID=your_query_id_here
//! ```
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example fund_daily_summary --features api-client
//! ```

#[cfg(feature = "api-client")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ib_flex::api::FlexApiClient;
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::time::Duration;

    // Load .env file
    if let Ok(contents) = std::fs::read_to_string(".env") {
        for line in contents.lines() {
            let mut line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Handle "export KEY=value" format
            if let Some(rest) = line.strip_prefix("export ") {
                line = rest.trim();
            }
            if let Some((key, value)) = line.split_once('=') {
                // Strip quotes from value
                let value = value.trim();
                let value = value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                    .unwrap_or(value);
                std::env::set_var(key.trim(), value);
            }
        }
    }

    // Get credentials from environment
    let token = std::env::var("FUND_FLEX_TOKEN").map_err(|_| {
        "FUND_FLEX_TOKEN not set. Add it to .env file or set as environment variable."
    })?;

    let query_id = std::env::var("FUND_FLEX_DAILY_QUERY_ID").map_err(|_| {
        "FUND_FLEX_DAILY_QUERY_ID not set. Add it to .env file or set as environment variable."
    })?;

    println!("=== Fund Daily FLEX Summary ===\n");
    println!("Query ID: {}", query_id);

    // Create API client and fetch statement
    let client = FlexApiClient::new(token);

    println!("Fetching FLEX statement...");
    let reference_code = client.send_request(&query_id).await?;

    let xml = client
        .get_statement_with_retry(&reference_code, 15, Duration::from_secs(2))
        .await?;

    // Parse the statement
    let statement = ib_flex::parse_activity_flex(&xml)?;

    // === Account Summary ===
    println!("\n{}", "=".repeat(60));
    println!("ACCOUNT SUMMARY");
    println!("{}", "=".repeat(60));
    println!("Account ID:     {}", statement.account_id);
    println!(
        "Period:         {} to {}",
        statement.from_date, statement.to_date
    );
    println!("Generated:      {}", statement.when_generated);

    // === Position Summary ===
    println!("\n{}", "=".repeat(60));
    println!("POSITION SUMMARY");
    println!("{}", "=".repeat(60));

    let positions = &statement.positions.items;
    println!("Total Positions: {}", positions.len());

    if !positions.is_empty() {
        let total_value: Decimal = positions.iter().map(|p| p.position_value).sum();
        let total_unrealized_pnl: Decimal =
            positions.iter().filter_map(|p| p.fifo_pnl_unrealized).sum();

        println!("Total Market Value: ${:.2}", total_value);
        println!("Total Unrealized P&L: ${:.2}", total_unrealized_pnl);

        // Group by asset category
        let mut by_category: HashMap<String, (usize, Decimal)> = HashMap::new();
        for pos in positions {
            let cat = format!("{:?}", pos.asset_category);
            let entry = by_category.entry(cat).or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += pos.position_value;
        }

        println!("\nBy Asset Category:");
        let mut categories: Vec<_> = by_category.iter().collect();
        categories.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
        for (cat, (count, value)) in categories {
            let pct = if total_value != Decimal::ZERO {
                (value / total_value) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };
            println!(
                "  {:12} {:3} positions  ${:>14.2}  ({:>5.1}%)",
                cat, count, value, pct
            );
        }

        // Top positions by value
        println!("\nTop 10 Positions:");
        let mut sorted_positions: Vec<_> = positions.iter().collect();
        sorted_positions.sort_by(|a, b| b.position_value.abs().cmp(&a.position_value.abs()));
        for pos in sorted_positions.iter().take(10) {
            let pnl_str = pos
                .fifo_pnl_unrealized
                .map(|p| format!("{:+.2}", p))
                .unwrap_or_else(|| "N/A".to_string());
            println!(
                "  {:12} {:>10} @ {:>10.2}  Value: ${:>12.2}  P&L: ${:>10}",
                pos.symbol, pos.quantity, pos.mark_price, pos.position_value, pnl_str
            );
        }
    }

    // === Trade Summary ===
    println!("\n{}", "=".repeat(60));
    println!("TRADE SUMMARY");
    println!("{}", "=".repeat(60));

    let trades = &statement.trades.items;
    println!("Total Trades: {}", trades.len());

    if !trades.is_empty() {
        let total_commission: Decimal = trades.iter().map(|t| t.commission).sum();
        let total_realized_pnl: Decimal = trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();
        let total_proceeds: Decimal = trades.iter().map(|t| t.proceeds).sum();

        println!("Total Proceeds: ${:.2}", total_proceeds);
        println!("Total Commissions: ${:.2}", total_commission);
        println!("Total Realized P&L: ${:.2}", total_realized_pnl);

        // Trades by direction
        let buys = trades
            .iter()
            .filter(|t| matches!(t.buy_sell, Some(ib_flex::BuySell::Buy)))
            .count();
        let sells = trades
            .iter()
            .filter(|t| matches!(t.buy_sell, Some(ib_flex::BuySell::Sell)))
            .count();
        println!("\nTrade Direction:");
        println!("  Buys:  {}", buys);
        println!("  Sells: {}", sells);

        // Recent trades
        if !trades.is_empty() {
            println!("\nRecent Trades (up to 5):");
            for trade in trades.iter().take(5) {
                let side = trade
                    .buy_sell
                    .as_ref()
                    .map(|b| format!("{:?}", b))
                    .unwrap_or_else(|| "?".to_string());
                let qty = trade.quantity.unwrap_or_default();
                let price = trade.price.unwrap_or_default();
                println!(
                    "  {} {:5} {:12} {:>8} @ {:>10.4}  Commission: ${:.2}",
                    trade.trade_date, side, trade.symbol, qty, price, trade.commission
                );
            }
        }
    }

    // === Cash Transactions ===
    println!("\n{}", "=".repeat(60));
    println!("CASH TRANSACTIONS");
    println!("{}", "=".repeat(60));

    let cash_txns = &statement.cash_transactions.items;
    println!("Total Transactions: {}", cash_txns.len());

    if !cash_txns.is_empty() {
        // Group by type
        let mut by_type: HashMap<&str, Decimal> = HashMap::new();
        for txn in cash_txns {
            *by_type
                .entry(&txn.transaction_type)
                .or_insert(Decimal::ZERO) += txn.amount;
        }

        println!("\nBy Type:");
        let mut types: Vec<_> = by_type.iter().collect();
        types.sort_by(|a, b| b.1.abs().cmp(&a.1.abs()));
        for (txn_type, amount) in types {
            println!("  {:30} ${:>12.2}", txn_type, amount);
        }

        // Total dividends
        let dividends: Decimal = cash_txns
            .iter()
            .filter(|t| t.transaction_type.contains("Dividend"))
            .map(|t| t.amount)
            .sum();
        if dividends != Decimal::ZERO {
            println!("\nTotal Dividends: ${:.2}", dividends);
        }

        // Withholding tax
        let withholding: Decimal = cash_txns
            .iter()
            .filter(|t| t.transaction_type.contains("Withholding"))
            .map(|t| t.amount)
            .sum();
        if withholding != Decimal::ZERO {
            println!("Total Withholding Tax: ${:.2}", withholding);
        }
    }

    // === Corporate Actions ===
    let corp_actions = &statement.corporate_actions.items;
    if !corp_actions.is_empty() {
        println!("\n{}", "=".repeat(60));
        println!("CORPORATE ACTIONS");
        println!("{}", "=".repeat(60));
        println!("Total Actions: {}", corp_actions.len());

        for action in corp_actions.iter().take(5) {
            println!(
                "  {} {} - {} ({})",
                action.report_date, action.symbol, action.action_type, action.description
            );
        }
    }

    // === Currency Exposure ===
    if !positions.is_empty() {
        println!("\n{}", "=".repeat(60));
        println!("CURRENCY EXPOSURE");
        println!("{}", "=".repeat(60));

        let mut by_currency: HashMap<&str, Decimal> = HashMap::new();
        for pos in positions {
            *by_currency.entry(&pos.currency).or_insert(Decimal::ZERO) += pos.position_value;
        }

        let total: Decimal = by_currency.values().sum();
        let mut currencies: Vec<_> = by_currency.iter().collect();
        currencies.sort_by(|a, b| b.1.abs().cmp(&a.1.abs()));
        for (currency, value) in currencies {
            let pct = if total != Decimal::ZERO {
                (value / total) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };
            println!("  {:5} ${:>14.2}  ({:>5.1}%)", currency, value, pct);
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("Done!");

    Ok(())
}

#[cfg(not(feature = "api-client"))]
fn main() {
    eprintln!("This example requires the 'api-client' feature.");
    eprintln!("Run with: cargo run --example fund_daily_summary --features api-client");
    std::process::exit(1);
}
