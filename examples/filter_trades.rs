//! Filter trades by various criteria

use ib_flex::{parse_activity_flex, AssetCategory, BuySell};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read XML from embedded fixture
    let xml = include_str!("../tests/fixtures/activity_minimal.xml");

    // Parse the statement
    let statement = parse_activity_flex(xml)?;

    println!("=== Trade Filtering Examples ===\n");

    // Filter by asset category
    println!("Stock trades:");
    let stock_trades: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| t.asset_category == AssetCategory::Stock)
        .collect();
    println!("  Found {} stock trades", stock_trades.len());

    // Filter by buy/sell
    println!("\nBuy trades:");
    let buy_trades: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| t.buy_sell == Some(BuySell::Buy))
        .collect();
    println!("  Found {} buy trades", buy_trades.len());

    // Filter by symbol
    println!("\nAAPL trades:");
    let aapl_trades: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| t.symbol == "AAPL")
        .collect();
    println!("  Found {} AAPL trades", aapl_trades.len());

    // Filter by minimum quantity
    println!("\nTrades with quantity >= 50:");
    let large_trades: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| {
            t.quantity
                .map(|q| q.abs() >= rust_decimal::Decimal::new(50, 0))
                .unwrap_or(false)
        })
        .collect();
    println!("  Found {} large trades", large_trades.len());

    // Calculate P&L for trades with realized P&L
    println!("\nTrades with realized P&L:");
    let trades_with_pnl: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| t.fifo_pnl_realized.is_some())
        .collect();
    println!("  Found {} trades", trades_with_pnl.len());

    if !trades_with_pnl.is_empty() {
        let total_pnl: rust_decimal::Decimal = trades_with_pnl
            .iter()
            .filter_map(|t| t.fifo_pnl_realized)
            .sum();
        println!("  Total realized P&L: ${}", total_pnl);
    }

    // Filter by date
    println!("\nTrades on {}:", statement.from_date);
    let trades_on_date: Vec<_> = statement
        .trades
        .items
        .iter()
        .filter(|t| t.trade_date == statement.from_date)
        .collect();
    println!("  Found {} trades", trades_on_date.len());

    Ok(())
}
