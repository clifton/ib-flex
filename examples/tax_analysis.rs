//! Tax Analysis Example
//!
//! This example demonstrates how to analyze trading activity for tax reporting,
//! including:
//! - Wash sale detection and tracking
//! - Short-term vs long-term capital gains classification
//! - Positions still under wash sale restriction
//! - Tax lot tracking

use chrono::{Datelike, Duration, NaiveDate};
use ib_flex::parse_activity_flex_all;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs;

/// Represents a wash sale event
#[derive(Debug, Clone)]
struct WashSale {
    symbol: String,
    sell_date: NaiveDate,
    loss_amount: Decimal,
    disallowed_amount: Decimal,
    repurchase_date: Option<NaiveDate>,
}

/// Represents a position potentially under wash sale restriction
#[derive(Debug, Clone)]
struct RestrictedPosition {
    symbol: String,
    conid: String,
    quantity: Decimal,
    acquisition_date: NaiveDate,
    cost_basis_adjustment: Decimal,
    restriction_ends: NaiveDate,
}

/// Summary of capital gains by category
#[derive(Debug, Default)]
struct CapitalGainsSummary {
    short_term_gains: Decimal,
    short_term_losses: Decimal,
    long_term_gains: Decimal,
    long_term_losses: Decimal,
    wash_sale_disallowed: Decimal,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the XML file path from command line or use default
    // Note: Uses the test fixture by default. For real IB files, pass the path as argument.
    let xml_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "tests/fixtures/activity_daily_portfolio.xml".to_string());

    println!("=======================================================");
    println!("           TAX ANALYSIS REPORT");
    println!("=======================================================\n");

    // Read and parse the XML file
    println!("Loading FLEX statement from: {}", xml_path);
    let xml = fs::read_to_string(&xml_path)?;
    let statements = parse_activity_flex_all(&xml)?;

    println!("Found {} statements", statements.len());
    if statements.is_empty() {
        println!("No statements found!");
        return Ok(());
    }

    // Get account and date range from all statements
    let first = &statements[0];
    let last = &statements[statements.len() - 1];
    println!("Account: {}", first.account_id);
    println!("Period: {} to {}\n", first.from_date, last.to_date);

    // Analyze tax year 2025
    let tax_year = 2025;
    let tax_year_start = NaiveDate::from_ymd_opt(tax_year, 1, 1).unwrap();
    let tax_year_end = NaiveDate::from_ymd_opt(tax_year, 12, 31).unwrap();

    println!("Analyzing tax year: {}\n", tax_year);

    // 1. Analyze trades for wash sales and capital gains
    let mut wash_sales: Vec<WashSale> = Vec::new();
    let mut capital_gains = CapitalGainsSummary::default();
    let mut trades_by_symbol: HashMap<String, Vec<_>> = HashMap::new();

    // Collect all closing trades from all statements
    for statement in &statements {
        for trade in &statement.trades.items {
            // Skip if no trade date or not in tax year
            let trade_date = match trade.trade_date {
                Some(d) => d,
                None => continue,
            };
            if trade_date < tax_year_start || trade_date > tax_year_end {
                continue;
            }

            // Check for wash sale indicator in notes
            let is_wash_sale = trade
                .notes
                .as_ref()
                .map(|n| n.contains('W') || n.to_uppercase().contains("WASH"))
                .unwrap_or(false);

            // Get realized P&L
            if let Some(pnl) = trade.fifo_pnl_realized {
                if pnl != Decimal::ZERO {
                    // Determine if long-term or short-term
                    let is_long_term = if let Some(orig_date) = trade.orig_trade_date {
                        let holding_period = trade_date - orig_date;
                        holding_period > Duration::days(365)
                    } else if let Some(hpdt) = &trade.holding_period_date_time {
                        // Parse holding period datetime if available
                        if hpdt.len() >= 10 {
                            if let Ok(hp_date) = NaiveDate::parse_from_str(&hpdt[..10], "%Y-%m-%d")
                            {
                                let holding_period = trade_date - hp_date;
                                holding_period > Duration::days(365)
                            } else {
                                false // Default to short-term if can't determine
                            }
                        } else {
                            false
                        }
                    } else {
                        false // Default to short-term if no original date
                    };

                    // Record wash sale if applicable
                    if is_wash_sale && pnl < Decimal::ZERO {
                        wash_sales.push(WashSale {
                            symbol: trade.symbol.clone(),
                            sell_date: trade_date,
                            loss_amount: pnl,
                            disallowed_amount: pnl.abs(), // Assuming full loss is disallowed
                            repurchase_date: None,        // Would need to match with purchases
                        });
                        capital_gains.wash_sale_disallowed += pnl.abs();
                    }

                    // Categorize the gain/loss
                    if is_long_term {
                        if pnl >= Decimal::ZERO {
                            capital_gains.long_term_gains += pnl;
                        } else {
                            capital_gains.long_term_losses += pnl.abs();
                        }
                    } else if pnl >= Decimal::ZERO {
                        capital_gains.short_term_gains += pnl;
                    } else {
                        capital_gains.short_term_losses += pnl.abs();
                    }

                    // Track by symbol for wash sale analysis
                    trades_by_symbol
                        .entry(trade.symbol.clone())
                        .or_default()
                        .push((trade_date, pnl, trade.quantity.unwrap_or_default()));
                }
            }
        }
    } // End of statement loop

    // 2. Identify positions potentially under wash sale restriction
    // A position is restricted if acquired within 30 days before or after a loss sale
    let mut restricted_positions: Vec<RestrictedPosition> = Vec::new();
    let wash_sale_window = Duration::days(30);
    let today = last.to_date;

    // Use positions from last statement (most recent)
    for position in &last.positions.items {
        // Parse acquisition date from open_date_time or holding_period_date_time
        let acquisition_date = if let Some(odt) = &position.open_date_time {
            if odt.len() >= 10 {
                NaiveDate::parse_from_str(&odt[..10], "%Y-%m-%d").ok()
            } else {
                None
            }
        } else if let Some(hpdt) = &position.holding_period_date_time {
            if hpdt.len() >= 10 {
                NaiveDate::parse_from_str(&hpdt[..10], "%Y-%m-%d").ok()
            } else {
                None
            }
        } else {
            None
        };

        if let Some(acq_date) = acquisition_date {
            // Check if this symbol had any loss sales within wash sale window
            if let Some(symbol_trades) = trades_by_symbol.get(&position.symbol) {
                for (trade_date, pnl, _qty) in symbol_trades {
                    if *pnl < Decimal::ZERO {
                        // This was a loss sale
                        let days_diff = if acq_date > *trade_date {
                            (acq_date - *trade_date).num_days()
                        } else {
                            (*trade_date - acq_date).num_days()
                        };

                        if days_diff <= 30 {
                            let restriction_ends = *trade_date + wash_sale_window;
                            // Only include if restriction is still active
                            if restriction_ends >= today {
                                restricted_positions.push(RestrictedPosition {
                                    symbol: position.symbol.clone(),
                                    conid: position.conid.clone(),
                                    quantity: position.quantity,
                                    acquisition_date: acq_date,
                                    cost_basis_adjustment: pnl.abs(),
                                    restriction_ends,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Print the report
    println!("=======================================================");
    println!("               CAPITAL GAINS SUMMARY");
    println!("=======================================================\n");

    println!("SHORT-TERM CAPITAL GAINS/LOSSES (held <= 1 year):");
    println!("  Gains:  ${:.2}", capital_gains.short_term_gains);
    println!("  Losses: ${:.2}", capital_gains.short_term_losses);
    println!(
        "  Net:    ${:.2}",
        capital_gains.short_term_gains - capital_gains.short_term_losses
    );
    println!();

    println!("LONG-TERM CAPITAL GAINS/LOSSES (held > 1 year):");
    println!("  Gains:  ${:.2}", capital_gains.long_term_gains);
    println!("  Losses: ${:.2}", capital_gains.long_term_losses);
    println!(
        "  Net:    ${:.2}",
        capital_gains.long_term_gains - capital_gains.long_term_losses
    );
    println!();

    let net_short = capital_gains.short_term_gains - capital_gains.short_term_losses;
    let net_long = capital_gains.long_term_gains - capital_gains.long_term_losses;
    println!("TOTAL NET CAPITAL GAIN/LOSS: ${:.2}", net_short + net_long);
    println!();

    // 4. Wash Sales Report
    println!("=======================================================");
    println!("                  WASH SALES");
    println!("=======================================================\n");

    if wash_sales.is_empty() {
        println!("No wash sales detected in the trading activity.\n");
    } else {
        println!("Found {} wash sale transaction(s):\n", wash_sales.len());
        println!(
            "{:<10} {:<12} {:>15} {:>15}",
            "SYMBOL", "SELL DATE", "LOSS", "DISALLOWED"
        );
        println!("{}", "-".repeat(55));

        for ws in &wash_sales {
            println!(
                "{:<10} {:<12} {:>15.2} {:>15.2}",
                ws.symbol, ws.sell_date, ws.loss_amount, ws.disallowed_amount
            );
        }
        println!();
        println!(
            "Total Wash Sale Loss Disallowed: ${:.2}",
            capital_gains.wash_sale_disallowed
        );
        println!();
        println!("Note: Disallowed losses are added to the cost basis of the");
        println!("replacement shares and will be recognized when those shares are sold.");
    }
    println!();

    // 5. Positions Under Wash Sale Restriction
    println!("=======================================================");
    println!("      POSITIONS UNDER WASH SALE RESTRICTION");
    println!("=======================================================\n");

    if restricted_positions.is_empty() {
        println!("No positions currently under wash sale restriction.\n");
    } else {
        println!(
            "Found {} position(s) potentially under wash sale restriction:\n",
            restricted_positions.len()
        );
        println!(
            "{:<10} {:>10} {:<12} {:<12} {:>15}",
            "SYMBOL", "QTY", "ACQUIRED", "RESTRICTION", "BASIS ADJ"
        );
        println!("{}", "-".repeat(65));

        for rp in &restricted_positions {
            println!(
                "{:<10} {:>10.2} {:<12} {:<12} {:>15.2}",
                rp.symbol,
                rp.quantity,
                rp.acquisition_date,
                rp.restriction_ends,
                rp.cost_basis_adjustment
            );
        }
        println!();
        println!("Note: These positions have an adjusted cost basis due to wash sale rules.");
        println!("The disallowed loss is added to the cost basis of these shares.");
    }
    println!();

    // 6. Summary statistics
    println!("=======================================================");
    println!("                   STATISTICS");
    println!("=======================================================\n");

    let total_trades: usize = statements.iter().map(|s| s.trades.items.len()).sum();
    let closing_trades: usize = statements
        .iter()
        .flat_map(|s| s.trades.items.iter())
        .filter(|t| t.fifo_pnl_realized.is_some() && t.fifo_pnl_realized != Some(Decimal::ZERO))
        .count();
    let unique_symbols: std::collections::HashSet<_> = statements
        .iter()
        .flat_map(|s| s.trades.items.iter())
        .map(|t| &t.symbol)
        .collect();

    println!("Total trades in period: {}", total_trades);
    println!("Closing trades (with P&L): {}", closing_trades);
    println!("Unique symbols traded: {}", unique_symbols.len());
    println!("Open positions: {}", last.positions.items.len());
    println!();

    // 7. Dividends and Interest (also tax-relevant)
    let mut total_dividends = Decimal::ZERO;
    let mut total_withholding = Decimal::ZERO;
    let mut total_interest = Decimal::ZERO;

    for statement in &statements {
        for cash_txn in &statement.cash_transactions.items {
            // Skip if not in tax year
            if let Some(date) = cash_txn.date {
                if date < tax_year_start || date > tax_year_end {
                    continue;
                }
            }

            match cash_txn.transaction_type.as_deref() {
                Some("Dividends") | Some("Payment In Lieu Of Dividends") => {
                    total_dividends += cash_txn.amount;
                }
                Some("Withholding Tax") => {
                    total_withholding += cash_txn.amount; // Usually negative
                }
                Some("Broker Interest Received") | Some("Bond Interest Received") => {
                    total_interest += cash_txn.amount;
                }
                Some("Broker Interest Paid") => {
                    total_interest += cash_txn.amount; // Usually negative
                }
                _ => {}
            }
        }
    } // End statement loop

    println!("=======================================================");
    println!("              DIVIDENDS & INTEREST");
    println!("=======================================================\n");
    println!("Total Dividends Received: ${:.2}", total_dividends);
    println!("Withholding Tax Paid:     ${:.2}", total_withholding.abs());
    println!(
        "Net Dividends:            ${:.2}",
        total_dividends + total_withholding
    );
    println!();
    println!("Interest (net):           ${:.2}", total_interest);
    println!();

    println!("=======================================================");
    println!("                  END OF REPORT");
    println!("=======================================================");

    Ok(())
}
