//! Parse an Activity FLEX statement and display summary

use ib_flex::parse_activity_flex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read XML from file or use embedded example
    let xml = include_str!("../tests/fixtures/activity_minimal.xml");

    // Parse the statement
    let statement = parse_activity_flex(xml)?;

    // Display account info
    println!("=== Activity FLEX Statement ===");
    println!("Account ID: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!();

    // Display trades summary
    println!("=== Trades ({}) ===", statement.trades.items.len());
    for trade in &statement.trades.items {
        println!(
            "{} {:?} {} @ {} {} (Commission: {})",
            trade.trade_date,
            trade.buy_sell,
            trade.quantity.unwrap_or_default(),
            trade.symbol,
            trade.price.unwrap_or_default(),
            trade.commission
        );
    }
    println!();

    // Calculate total commissions
    let total_commission: rust_decimal::Decimal =
        statement.trades.items.iter().map(|t| t.commission).sum();
    println!("Total Commissions: ${}", total_commission);
    println!();

    // Display positions
    println!(
        "=== Open Positions ({}) ===",
        statement.positions.items.len()
    );
    for position in &statement.positions.items {
        let pnl = position.fifo_pnl_unrealized.unwrap_or_default();
        println!(
            "{}: {} @ {} (Value: {}, P&L: {})",
            position.symbol, position.quantity, position.mark_price, position.position_value, pnl
        );
    }
    println!();

    // Display cash transactions
    println!(
        "=== Cash Transactions ({}) ===",
        statement.cash_transactions.items.len()
    );
    for cash in &statement.cash_transactions.items {
        println!(
            "{:?} {}: {} {}",
            cash.date, cash.transaction_type, cash.amount, cash.currency
        );
        if let Some(desc) = &cash.description {
            println!("  Description: {}", desc);
        }
    }
    println!();

    // Display corporate actions
    println!(
        "=== Corporate Actions ({}) ===",
        statement.corporate_actions.items.len()
    );
    for action in &statement.corporate_actions.items {
        println!(
            "{} {}: {} - {}",
            action.report_date, action.action_type, action.symbol, action.description
        );
    }

    Ok(())
}
