//! Debug backfill parsing

use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "tmp/backfill-to-2026-01-13.xml".to_string());
    let xml = fs::read_to_string(&path)?;

    println!("File size: {} bytes", xml.len());
    println!("Testing parse_activity_flex_all...");

    // Try parsing all statements
    match ib_flex::parse_activity_flex_all(&xml) {
        Ok(statements) => {
            println!("SUCCESS! Found {} statements", statements.len());

            let mut total_trades = 0;
            let mut total_pnl = rust_decimal::Decimal::ZERO;

            for (i, stmt) in statements.iter().enumerate() {
                let trades = stmt.trades.items.len();
                total_trades += trades;

                // Sum realized P&L
                for trade in &stmt.trades.items {
                    if let Some(pnl) = trade.fifo_pnl_realized {
                        total_pnl += pnl;
                    }
                }

                if trades > 0 {
                    println!(
                        "  Statement {}: {} to {} - {} trades",
                        i + 1,
                        stmt.from_date,
                        stmt.to_date,
                        trades
                    );
                }
            }

            println!("\nTotals across all {} statements:", statements.len());
            println!("  Total trades: {}", total_trades);
            println!("  Total realized P&L: ${:.2}", total_pnl);
        }
        Err(e) => {
            println!("FAILED: {:?}", e);
        }
    }

    Ok(())
}
