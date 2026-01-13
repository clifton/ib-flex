//! Calculate total commissions from an Activity FLEX statement

use ib_flex::parse_activity_flex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read XML from embedded fixture
    let xml = include_str!("../tests/fixtures/activity_minimal.xml");

    // Parse the statement
    let statement = parse_activity_flex(xml)?;

    println!("=== Commission Analysis ===");
    println!("Account: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!();

    // Calculate total commissions
    let total_commission: rust_decimal::Decimal =
        statement.trades.items.iter().map(|t| t.commission).sum();

    println!("Total trades: {}", statement.trades.items.len());
    println!("Total commissions: ${}", total_commission);
    println!();

    // Break down by symbol
    println!("=== By Symbol ===");
    let mut symbols = std::collections::HashMap::new();

    for trade in &statement.trades.items {
        let entry = symbols
            .entry(trade.symbol.clone())
            .or_insert((0i32, rust_decimal::Decimal::ZERO));
        entry.0 += 1;
        entry.1 += trade.commission;
    }

    for (symbol, (count, commission)) in symbols {
        println!("{}: {} trades, ${} commission", symbol, count, commission);
    }

    // Calculate average commission per trade
    if !statement.trades.items.is_empty() {
        let avg =
            total_commission / rust_decimal::Decimal::new(statement.trades.items.len() as i64, 0);
        println!();
        println!("Average commission per trade: ${}", avg);
    }

    Ok(())
}
