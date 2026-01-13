//! Example: Historical data backfill using FLEX queries
//!
//! This example demonstrates fetching historical FLEX data over multiple days
//! for backfilling a local database or data store.
//!
//! ## IB FLEX Date Range Options
//!
//! IB FLEX queries have predefined period options set in Account Management:
//! - LastBusinessDay, Last2BusinessDays, Last3BusinessDays, etc.
//! - Last7CalendarDays, Last30CalendarDays, MonthToDate, YearToDate
//! - Custom date range (fixed dates set in query definition)
//!
//! For comprehensive backfilling, you'll typically need multiple queries:
//! - A "LastBusinessDay" query for daily updates
//! - A "Last30Days" or "MonthToDate" query for catching up
//! - A "YearToDate" query for full year backfill
//!
//! ## Setup
//!
//! Create a `.env` file in the project root with:
//! ```text
//! FUND_FLEX_TOKEN=your_flex_token_here
//! FUND_FLEX_DAILY_QUERY_ID=your_daily_query_id
//! FUND_FLEX_MTD_QUERY_ID=your_month_to_date_query_id  # optional
//! FUND_FLEX_YTD_QUERY_ID=your_year_to_date_query_id   # optional
//! ```
//!
//! ## Usage
//!
//! ```bash
//! # Fetch daily data
//! cargo run --example historical_backfill --features api-client
//!
//! # Fetch month-to-date data
//! cargo run --example historical_backfill --features api-client -- --period mtd
//!
//! # Fetch year-to-date data
//! cargo run --example historical_backfill --features api-client -- --period ytd
//! ```

#[cfg(feature = "api-client")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ib_flex::api::FlexApiClient;
    use std::fs;
    use std::path::PathBuf;
    use std::time::Duration;

    // Load .env file
    load_env();

    // Parse command line args
    let args: Vec<String> = std::env::args().collect();
    let period = args
        .iter()
        .position(|a| a == "--period")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("daily");

    // Get credentials
    let token = std::env::var("FUND_FLEX_TOKEN").map_err(|_| {
        "FUND_FLEX_TOKEN not set. Add it to .env file or set as environment variable."
    })?;

    let query_id = match period {
        "mtd" => std::env::var("FUND_FLEX_MTD_QUERY_ID")
            .map_err(|_| "FUND_FLEX_MTD_QUERY_ID not set for month-to-date query.")?,
        "ytd" => std::env::var("FUND_FLEX_YTD_QUERY_ID")
            .map_err(|_| "FUND_FLEX_YTD_QUERY_ID not set for year-to-date query.")?,
        _ => std::env::var("FUND_FLEX_DAILY_QUERY_ID")
            .map_err(|_| "FUND_FLEX_DAILY_QUERY_ID not set for daily query.")?,
    };

    println!("=== Historical FLEX Backfill ===\n");
    println!("Period: {}", period);
    println!("Query ID: {}", query_id);

    // Create output directory
    let output_dir = PathBuf::from("tmp/backfill");
    fs::create_dir_all(&output_dir)?;
    println!("Output directory: {}\n", output_dir.display());

    // Fetch the statement
    let client = FlexApiClient::new(token);
    println!("Fetching FLEX statement...");

    let reference_code = client.send_request(&query_id).await?;
    println!("Reference code: {}", reference_code);

    let xml = client
        .get_statement_with_retry(&reference_code, 15, Duration::from_secs(2))
        .await?;
    println!("Received {} bytes\n", xml.len());

    // Parse the statement
    let statement = ib_flex::parse_activity_flex(&xml)?;

    // Save raw XML
    let xml_filename = format!(
        "{}_{}_to_{}.xml",
        statement.account_id, statement.from_date, statement.to_date
    );
    let xml_path = output_dir.join(&xml_filename);
    fs::write(&xml_path, &xml)?;
    println!("Saved raw XML to: {}", xml_path.display());

    // Extract and display summary
    print_statement_summary(&statement);

    // Extract daily snapshots if this is a multi-day statement
    if statement.from_date != statement.to_date {
        println!("\n{}", "=".repeat(60));
        println!("EXTRACTING DAILY SNAPSHOTS");
        println!("{}", "=".repeat(60));

        let daily_data = extract_daily_snapshots(&statement);

        for (date, snapshot) in &daily_data {
            println!("\n--- {} ---", date);
            println!("  Positions: {}", snapshot.position_count);
            println!("  Trades: {}", snapshot.trade_count);
            println!(
                "  Total Position Value: ${:.2}",
                snapshot.total_position_value
            );
            println!("  Realized P&L: ${:.2}", snapshot.realized_pnl);
            println!("  Unrealized P&L: ${:.2}", snapshot.unrealized_pnl);
        }

        // Save daily snapshots as JSON
        let json_filename = format!(
            "{}_{}_to_{}_daily.json",
            statement.account_id, statement.from_date, statement.to_date
        );
        let json_path = output_dir.join(&json_filename);
        let json = serde_json::to_string_pretty(&daily_data)?;
        fs::write(&json_path, &json)?;
        println!("\nSaved daily snapshots to: {}", json_path.display());
    }

    // Example: Incremental backfill tracking
    println!("\n{}", "=".repeat(60));
    println!("BACKFILL STATUS");
    println!("{}", "=".repeat(60));

    let status_file = output_dir.join("backfill_status.json");
    let mut status = load_backfill_status(&status_file);

    // Update status with this fetch
    status.last_fetch = Some(chrono::Utc::now().to_rfc3339());
    status.account_id = Some(statement.account_id.clone());
    status
        .fetched_ranges
        .push((statement.from_date, statement.to_date));

    // Calculate coverage
    let all_dates = get_all_dates_in_ranges(&status.fetched_ranges);
    status.total_days_fetched = all_dates.len();

    println!("Account: {}", statement.account_id);
    println!(
        "This fetch: {} to {}",
        statement.from_date, statement.to_date
    );
    println!("Total days in dataset: {}", status.total_days_fetched);
    println!(
        "Date range coverage: {:?} to {:?}",
        all_dates.iter().min(),
        all_dates.iter().max()
    );

    // Save updated status
    let status_json = serde_json::to_string_pretty(&status)?;
    fs::write(&status_file, &status_json)?;
    println!("\nUpdated backfill status: {}", status_file.display());

    println!("\n{}", "=".repeat(60));
    println!("NEXT STEPS FOR COMPLETE BACKFILL");
    println!("{}", "=".repeat(60));
    println!("1. Run with --period mtd for month-to-date data");
    println!("2. Run with --period ytd for year-to-date data");
    println!("3. Set up a daily cron job to fetch LastBusinessDay data");
    println!("4. Process the JSON snapshots into your database");

    Ok(())
}

#[cfg(feature = "api-client")]
fn load_env() {
    if let Ok(contents) = std::fs::read_to_string(".env") {
        for line in contents.lines() {
            let mut line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("export ") {
                line = rest.trim();
            }
            if let Some((key, value)) = line.split_once('=') {
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
}

#[cfg(feature = "api-client")]
fn print_statement_summary(statement: &ib_flex::ActivityFlexStatement) {
    use rust_decimal::Decimal;

    println!("{}", "=".repeat(60));
    println!("STATEMENT SUMMARY");
    println!("{}", "=".repeat(60));
    println!("Account: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!("Generated: {}", statement.when_generated);

    let days = (statement.to_date - statement.from_date).num_days() + 1;
    println!("Days covered: {}", days);

    println!("\nData counts:");
    println!("  Positions: {}", statement.positions.items.len());
    println!("  Trades: {}", statement.trades.items.len());
    println!(
        "  Cash Transactions: {}",
        statement.cash_transactions.items.len()
    );
    println!(
        "  Corporate Actions: {}",
        statement.corporate_actions.items.len()
    );

    // Calculate totals
    let total_position_value: Decimal = statement
        .positions
        .items
        .iter()
        .map(|p| p.position_value)
        .sum();

    let total_unrealized: Decimal = statement
        .positions
        .items
        .iter()
        .filter_map(|p| p.fifo_pnl_unrealized)
        .sum();

    let total_realized: Decimal = statement
        .trades
        .items
        .iter()
        .filter_map(|t| t.fifo_pnl_realized)
        .sum();

    let total_commissions: Decimal = statement.trades.items.iter().map(|t| t.commission).sum();

    println!("\nPortfolio Summary:");
    println!("  Total Position Value: ${:.2}", total_position_value);
    println!("  Unrealized P&L: ${:.2}", total_unrealized);
    println!("  Realized P&L: ${:.2}", total_realized);
    println!("  Total Commissions: ${:.2}", total_commissions);
}

/// Daily snapshot data for storage
#[cfg(feature = "api-client")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DailySnapshot {
    date: chrono::NaiveDate,
    position_count: usize,
    trade_count: usize,
    total_position_value: rust_decimal::Decimal,
    realized_pnl: rust_decimal::Decimal,
    unrealized_pnl: rust_decimal::Decimal,
    commissions: rust_decimal::Decimal,
    dividends: rust_decimal::Decimal,
}

#[cfg(feature = "api-client")]
fn extract_daily_snapshots(
    statement: &ib_flex::ActivityFlexStatement,
) -> std::collections::HashMap<chrono::NaiveDate, DailySnapshot> {
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use std::collections::HashMap;

    let mut snapshots: HashMap<NaiveDate, DailySnapshot> = HashMap::new();

    // Get all unique trade dates
    let mut dates: Vec<NaiveDate> = statement
        .trades
        .items
        .iter()
        .map(|t| t.trade_date)
        .collect();

    // Add cash transaction dates
    for txn in &statement.cash_transactions.items {
        if let Some(date) = txn.date {
            dates.push(date);
        }
    }

    // Add position report date (usually just the end date)
    for pos in &statement.positions.items {
        dates.push(pos.report_date);
    }

    dates.sort();
    dates.dedup();

    // Build snapshot for each date
    for date in dates {
        let day_trades: Vec<_> = statement
            .trades
            .items
            .iter()
            .filter(|t| t.trade_date == date)
            .collect();

        let day_cash: Vec<_> = statement
            .cash_transactions
            .items
            .iter()
            .filter(|c| c.date == Some(date))
            .collect();

        // Positions are typically end-of-period, so we only have them for the last date
        let positions_for_date = if date == statement.to_date {
            &statement.positions.items[..]
        } else {
            &[]
        };

        let total_position_value: Decimal =
            positions_for_date.iter().map(|p| p.position_value).sum();

        let unrealized_pnl: Decimal = positions_for_date
            .iter()
            .filter_map(|p| p.fifo_pnl_unrealized)
            .sum();

        let realized_pnl: Decimal = day_trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();

        let commissions: Decimal = day_trades.iter().map(|t| t.commission).sum();

        let dividends: Decimal = day_cash
            .iter()
            .filter(|c| c.transaction_type.contains("Dividend"))
            .map(|c| c.amount)
            .sum();

        snapshots.insert(
            date,
            DailySnapshot {
                date,
                position_count: positions_for_date.len(),
                trade_count: day_trades.len(),
                total_position_value,
                realized_pnl,
                unrealized_pnl,
                commissions,
                dividends,
            },
        );
    }

    snapshots
}

/// Backfill status tracking
#[cfg(feature = "api-client")]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct BackfillStatus {
    account_id: Option<String>,
    last_fetch: Option<String>,
    fetched_ranges: Vec<(chrono::NaiveDate, chrono::NaiveDate)>,
    total_days_fetched: usize,
}

#[cfg(feature = "api-client")]
fn load_backfill_status(path: &std::path::Path) -> BackfillStatus {
    if path.exists() {
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(status) = serde_json::from_str(&contents) {
                return status;
            }
        }
    }
    BackfillStatus::default()
}

#[cfg(feature = "api-client")]
fn get_all_dates_in_ranges(
    ranges: &[(chrono::NaiveDate, chrono::NaiveDate)],
) -> Vec<chrono::NaiveDate> {
    use chrono::Duration;
    use std::collections::HashSet;

    let mut dates: HashSet<chrono::NaiveDate> = HashSet::new();

    for (start, end) in ranges {
        let mut current = *start;
        while current <= *end {
            dates.insert(current);
            current += Duration::days(1);
        }
    }

    let mut result: Vec<_> = dates.into_iter().collect();
    result.sort();
    result
}

#[cfg(not(feature = "api-client"))]
fn main() {
    eprintln!("This example requires the 'api-client' feature.");
    eprintln!("Run with: cargo run --example historical_backfill --features api-client");
    std::process::exit(1);
}
