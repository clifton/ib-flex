//! Tax Analysis Example
//!
//! This example demonstrates how to analyze trading activity for tax reporting,
//! including:
//! - Wash sale detection and tracking (using IB's WashSale records)
//! - Short-term vs long-term capital gains classification
//! - Positions still under wash sale restriction
//! - Tax lot tracking
//!
//! **IB Wash Sale Tracking:**
//! IB automatically tracks wash sales via separate `<WashSale>` records in FLEX.
//! These records show the adjusted P&L when selling shares whose cost basis was
//! modified by wash sale rules. The `fifoPnlRealized` field reflects the gain/loss
//! AFTER the basis adjustment.

use chrono::{Duration, NaiveDate};
use ib_flex::parse_activity_flex_all;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs;

/// Summary of wash sale activity for a symbol
#[derive(Debug, Clone, Default)]
struct WashSaleSummary {
    symbol: String,
    record_count: usize,
    /// P&L recognized from selling shares with adjusted basis
    adjusted_pnl_recognized: Decimal,
    /// Dates of wash sale activity
    dates: Vec<NaiveDate>,
}

/// Represents a position potentially under wash sale restriction
#[derive(Debug, Clone)]
struct RestrictedPosition {
    symbol: String,
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
}

/// Summary of IB's wash sale tracking
#[derive(Debug, Default)]
struct WashSaleTracking {
    /// Total WashSale records from IB
    total_records: usize,
    /// Total P&L recognized through wash sale basis adjustments
    total_adjusted_pnl: Decimal,
    /// By symbol breakdown
    by_symbol: HashMap<String, WashSaleSummary>,
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

    // 1. Analyze regular trades for capital gains
    let mut capital_gains = CapitalGainsSummary::default();
    let mut trades_by_symbol: HashMap<String, Vec<_>> = HashMap::new();
    let mut loss_trades_by_symbol: HashMap<String, Vec<(NaiveDate, Decimal)>> = HashMap::new();

    // 2. Analyze IB's WashSale records (these show adjusted P&L from wash sale basis)
    let mut wash_tracking = WashSaleTracking::default();

    // First pass: Collect wash sale records from IB
    for statement in &statements {
        for ws in &statement.trades.wash_sales {
            let trade_date = match ws.trade_date {
                Some(d) => d,
                None => continue,
            };
            if trade_date < tax_year_start || trade_date > tax_year_end {
                continue;
            }

            wash_tracking.total_records += 1;
            if let Some(pnl) = ws.fifo_pnl_realized {
                wash_tracking.total_adjusted_pnl += pnl;

                let entry = wash_tracking
                    .by_symbol
                    .entry(ws.symbol.clone())
                    .or_insert_with(|| WashSaleSummary {
                        symbol: ws.symbol.clone(),
                        ..Default::default()
                    });
                entry.record_count += 1;
                entry.adjusted_pnl_recognized += pnl;
                if !entry.dates.contains(&trade_date) {
                    entry.dates.push(trade_date);
                }
            }
        }
    }

    // Second pass: Collect regular trades for capital gains
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
                        // Track loss trades by symbol for potential wash sale detection
                        loss_trades_by_symbol
                            .entry(trade.symbol.clone())
                            .or_default()
                            .push((trade_date, pnl));
                    }

                    // Track by symbol for analysis
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

    // 4. IB Wash Sale Tracking Report
    println!("=======================================================");
    println!("           IB WASH SALE BASIS ADJUSTMENTS");
    println!("=======================================================\n");

    if wash_tracking.total_records == 0 {
        println!("No wash sale records found in the trading activity.\n");
    } else {
        println!(
            "IB tracked {} wash sale adjustment record(s) in {}.\n",
            wash_tracking.total_records, tax_year
        );
        println!("These records represent P&L from selling shares that had their");
        println!("cost basis adjusted due to prior wash sale rules.\n");

        println!(
            "Total Adjusted P&L Recognized: ${:.2}",
            wash_tracking.total_adjusted_pnl
        );
        println!();

        // Sort by adjusted P&L and show top symbols
        let mut sorted_symbols: Vec<_> = wash_tracking.by_symbol.values().collect();
        sorted_symbols.sort_by(|a, b| b.adjusted_pnl_recognized.cmp(&a.adjusted_pnl_recognized));

        println!("{:<30} {:>8} {:>15}", "SYMBOL", "RECORDS", "ADJUSTED P&L");
        println!("{}", "-".repeat(55));

        for summary in sorted_symbols.iter().take(15) {
            println!(
                "{:<30} {:>8} ${:>14.2}",
                summary.symbol, summary.record_count, summary.adjusted_pnl_recognized
            );
        }
        if sorted_symbols.len() > 15 {
            println!("... and {} more symbols", sorted_symbols.len() - 15);
        }
        println!();

        println!("IMPORTANT: These wash sale adjustments are ALREADY reflected in");
        println!("the capital gains above. The adjusted P&L here shows gains from");
        println!("selling shares that had increased cost basis from prior wash sales.");
        println!("This recovered previously disallowed losses.");
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

    // 6. Wash Sale Compliance Summary
    println!("=======================================================");
    println!("           WASH SALE COMPLIANCE SUMMARY");
    println!("=======================================================\n");

    println!("IB automatically tracks wash sales and adjusts your cost basis.");
    println!("You cannot 'violate' wash sale rules - they are automatic:\n");

    println!("1. WHAT TRIGGERS A WASH SALE:");
    println!("   - Sell a security at a loss");
    println!("   - Buy substantially identical security within 30 days");
    println!("     (before OR after the sale)\n");

    println!("2. WHAT HAPPENS:");
    println!("   - The loss is DISALLOWED for current tax year");
    println!("   - Disallowed loss is ADDED to cost basis of new shares");
    println!("   - When new shares are sold, adjusted basis affects P&L\n");

    println!("3. YOUR SITUATION ({}):", tax_year);
    println!(
        "   - IB detected {} wash sale adjustment(s)",
        wash_tracking.total_records
    );
    println!(
        "   - Total adjusted P&L: ${:.2}",
        wash_tracking.total_adjusted_pnl
    );
    println!(
        "   - {} symbols had wash sale activity",
        wash_tracking.by_symbol.len()
    );
    println!();

    if wash_tracking.total_records > 0 {
        println!("4. TAX IMPLICATION:");
        println!("   The WashSale records show gains from selling shares that had");
        println!("   their cost basis increased by prior wash sales. This is normal");
        println!("   and IB handles the accounting automatically on your 1099-B.\n");

        println!(
            "   The ${:.2} in wash sale adjusted P&L represents gains",
            wash_tracking.total_adjusted_pnl
        );
        println!("   that were reduced by prior disallowed losses being 'recovered'");
        println!("   through the increased cost basis.\n");
    }

    println!("5. KEY TAKEAWAY:");
    println!("   Wash sale rules don't create additional tax liability - they");
    println!("   only DEFER losses. The loss is eventually recognized when the");
    println!("   replacement shares are sold (unless another wash sale occurs).\n");

    // 7. Summary statistics
    println!("=======================================================");
    println!("                   STATISTICS");
    println!("=======================================================\n");

    let total_trades: usize = statements.iter().map(|s| s.trades.items.len()).sum();
    let total_wash_records: usize = statements.iter().map(|s| s.trades.wash_sales.len()).sum();
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
    println!("Total WashSale records: {}", total_wash_records);
    println!("Closing trades (with P&L): {}", closing_trades);
    println!("Unique symbols traded: {}", unique_symbols.len());
    println!("Open positions: {}", last.positions.items.len());
    println!();

    // 8. Dividends and Interest (also tax-relevant)
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
