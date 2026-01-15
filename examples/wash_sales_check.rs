//! Quick check of wash sales data from backfill

use ib_flex::parse_activity_flex_all;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "tmp/backfill-to-2026-01-13.xml".to_string());

    println!("Loading: {}", xml_path);
    let xml = fs::read_to_string(&xml_path)?;
    let statements = parse_activity_flex_all(&xml)?;

    println!("Found {} statements", statements.len());

    // Count wash sales across all statements
    let mut total_wash_sales = 0;
    let mut total_wash_pnl = Decimal::ZERO;
    let mut wash_by_symbol: HashMap<String, (usize, Decimal)> = HashMap::new();

    for statement in &statements {
        let ws_count = statement.trades.wash_sales.len();
        total_wash_sales += ws_count;

        for ws in &statement.trades.wash_sales {
            if let Some(pnl) = ws.fifo_pnl_realized {
                total_wash_pnl += pnl;
                let entry = wash_by_symbol
                    .entry(ws.symbol.clone())
                    .or_insert((0, Decimal::ZERO));
                entry.0 += 1;
                entry.1 += pnl;
            }
        }
    }

    println!("\n=== WASH SALE RECORDS SUMMARY ===");
    println!("Total WashSale records: {}", total_wash_sales);
    println!(
        "Total fifoPnlRealized in WashSale records: ${:.2}",
        total_wash_pnl
    );

    println!("\n=== BY SYMBOL ===");
    let mut sorted: Vec<_> = wash_by_symbol.into_iter().collect();
    sorted.sort_by(|a, b| b.1 .1.cmp(&a.1 .1)); // Sort by PnL descending

    for (symbol, (count, pnl)) in sorted.iter().take(15) {
        println!("{:<30} {:>4} records  ${:>12.2} PnL", symbol, count, pnl);
    }

    // Show a sample wash sale record
    println!("\n=== SAMPLE WASH SALE RECORD ===");
    for statement in &statements {
        if let Some(ws) = statement.trades.wash_sales.first() {
            println!("Symbol: {}", ws.symbol);
            println!("Trade Date: {:?}", ws.trade_date);
            println!("Quantity: {:?}", ws.quantity);
            println!("Buy/Sell: {:?}", ws.buy_sell);
            println!("FIFO P&L Realized: {:?}", ws.fifo_pnl_realized);
            println!("Open DateTime: {:?}", ws.open_date_time);
            println!("Holding Period DateTime: {:?}", ws.holding_period_date_time);
            println!("When Realized: {:?}", ws.when_realized);
            println!("When Reopened: {:?}", ws.when_reopened);
            println!("Notes: {:?}", ws.notes);
            println!("Level of Detail: {:?}", ws.level_of_detail);
            break;
        }
    }

    Ok(())
}
