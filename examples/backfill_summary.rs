//! Example: Parse a backfill XML file and display summary statistics
//!
//! This example parses a local FLEX XML file containing multiple daily statements
//! and produces summary statistics including NAV over time, trading activity,
//! and performance metrics.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example backfill_summary -- path/to/your/backfill.xml
//! ```

use chrono::{Datelike, NaiveDate};
use quick_xml::events::Event;
use quick_xml::Reader;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get file path from args
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .map(|s| s.as_str())
        .ok_or("Usage: cargo run --example backfill_summary -- <path/to/flex.xml>")?;

    println!("=== IB FLEX Backfill Summary ===\n");
    println!("Loading: {}", file_path);

    // Read the XML
    let xml = fs::read_to_string(file_path)?;
    println!("File size: {} bytes\n", xml.len());

    // Parse using quick-xml reader for flexibility with mixed element types
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);

    let mut statement_count = 0;
    let mut nav_history: Vec<(NaiveDate, Decimal)> = Vec::new();
    let mut total_trades = 0;
    let mut total_commission = Decimal::ZERO;
    let mut total_realized_pnl = Decimal::ZERO;
    let mut trades_by_symbol: HashMap<String, (i32, Decimal, Decimal)> = HashMap::new();
    let mut cash_by_type: HashMap<String, (i32, Decimal)> = HashMap::new();
    let mut all_positions: Vec<(NaiveDate, Position)> = Vec::new();
    let mut latest_date: Option<NaiveDate> = None;
    let mut earliest_date: Option<NaiveDate> = None;
    let mut account_id = String::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                match name_str {
                    "FlexStatement" => {
                        statement_count += 1;
                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let val = std::str::from_utf8(&attr.value).unwrap_or("").to_string();
                            match key {
                                "accountId" if account_id.is_empty() => account_id = val,
                                "fromDate" => {
                                    if let Some(date) = parse_flex_date(&val) {
                                        if earliest_date.is_none() || date < earliest_date.unwrap()
                                        {
                                            earliest_date = Some(date);
                                        }
                                    }
                                }
                                "toDate" => {
                                    if let Some(date) = parse_flex_date(&val) {
                                        if latest_date.is_none() || date > latest_date.unwrap() {
                                            latest_date = Some(date);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    "EquitySummaryByReportDateInBase" => {
                        let mut date: Option<NaiveDate> = None;
                        let mut total: Option<Decimal> = None;

                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let val = std::str::from_utf8(&attr.value).unwrap_or("");
                            match key {
                                "reportDate" => date = parse_flex_date(val),
                                "total" => total = Decimal::from_str(val).ok(),
                                _ => {}
                            }
                        }

                        if let (Some(d), Some(t)) = (date, total) {
                            nav_history.push((d, t));
                        }
                    }
                    "Trade" => {
                        // Only count EXECUTION level trades
                        let mut is_execution = false;
                        let mut symbol = String::new();
                        let mut qty = Decimal::ZERO;
                        let mut commission = Decimal::ZERO;
                        let mut pnl = Decimal::ZERO;

                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let val = std::str::from_utf8(&attr.value).unwrap_or("");
                            match key {
                                "levelOfDetail" => is_execution = val == "EXECUTION",
                                "symbol" => symbol = val.to_string(),
                                "quantity" => qty = Decimal::from_str(val).unwrap_or_default(),
                                "ibCommission" => {
                                    commission = Decimal::from_str(val).unwrap_or_default()
                                }
                                "fifoPnlRealized" => {
                                    pnl = Decimal::from_str(val).unwrap_or_default()
                                }
                                _ => {}
                            }
                        }

                        if is_execution && !symbol.is_empty() {
                            total_trades += 1;
                            total_commission += commission;
                            total_realized_pnl += pnl;

                            let entry = trades_by_symbol.entry(symbol).or_insert((
                                0,
                                Decimal::ZERO,
                                Decimal::ZERO,
                            ));
                            entry.0 += 1;
                            entry.1 += qty.abs();
                            entry.2 += pnl;
                        }
                    }
                    "CashTransaction" => {
                        let mut txn_type = String::new();
                        let mut amount = Decimal::ZERO;

                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let val = std::str::from_utf8(&attr.value).unwrap_or("");
                            match key {
                                "type" => txn_type = val.to_string(),
                                "amount" => amount = Decimal::from_str(val).unwrap_or_default(),
                                _ => {}
                            }
                        }

                        if !txn_type.is_empty() {
                            let entry = cash_by_type.entry(txn_type).or_insert((0, Decimal::ZERO));
                            entry.0 += 1;
                            entry.1 += amount;
                        }
                    }
                    "OpenPosition" => {
                        let mut pos = Position::default();
                        let mut report_date: Option<NaiveDate> = None;

                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            let val = std::str::from_utf8(&attr.value).unwrap_or("");
                            match key {
                                "symbol" => pos.symbol = val.to_string(),
                                "position" => {
                                    pos.quantity = Decimal::from_str(val).unwrap_or_default()
                                }
                                "markPrice" => {
                                    pos.mark_price = Decimal::from_str(val).unwrap_or_default()
                                }
                                "positionValue" => {
                                    pos.value = Decimal::from_str(val).unwrap_or_default()
                                }
                                "fifoPnlUnrealized" => {
                                    pos.unrealized_pnl = Decimal::from_str(val).unwrap_or_default()
                                }
                                "reportDate" => report_date = parse_flex_date(val),
                                "assetCategory" => pos.asset_category = val.to_string(),
                                _ => {}
                            }
                        }

                        // Collect all positions with their dates
                        if let Some(rd) = report_date {
                            if !pos.symbol.is_empty() {
                                all_positions.push((rd, pos));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing XML: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    // Print summary
    println!("Account: {}", account_id);
    println!("Statement count: {}", statement_count);

    println!("\n{}", "=".repeat(60));
    println!("DATE RANGE");
    println!("{}", "=".repeat(60));
    if let (Some(start), Some(end)) = (earliest_date, latest_date) {
        println!("From: {}", start);
        println!("To:   {}", end);
        println!("Days: {}", (end - start).num_days() + 1);
    }

    // NAV analysis
    println!("\n{}", "=".repeat(60));
    println!("NAV OVER TIME");
    println!("{}", "=".repeat(60));

    nav_history.sort_by_key(|(date, _)| *date);
    nav_history.dedup_by_key(|(date, _)| *date);

    if !nav_history.is_empty() {
        let first_nav = nav_history.first().unwrap();
        let last_nav = nav_history.last().unwrap();

        println!("Starting NAV ({}):\t${:.2}", first_nav.0, first_nav.1);
        println!("Ending NAV ({}):\t${:.2}", last_nav.0, last_nav.1);

        let change = last_nav.1 - first_nav.1;
        let pct_change = if first_nav.1 != Decimal::ZERO {
            (change / first_nav.1) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        println!("Change:\t\t\t\t${:.2} ({:.2}%)", change, pct_change);

        // Find max and min NAV
        let max_nav = nav_history.iter().max_by_key(|(_, v)| *v).unwrap();
        let min_nav = nav_history.iter().min_by_key(|(_, v)| *v).unwrap();
        println!("\nHigh ({}):\t\t${:.2}", max_nav.0, max_nav.1);
        println!("Low ({}):\t\t${:.2}", min_nav.0, min_nav.1);

        // Drawdown from high
        let drawdown = max_nav.1 - last_nav.1;
        let drawdown_pct = if max_nav.1 != Decimal::ZERO {
            (drawdown / max_nav.1) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        println!(
            "Current drawdown from high:\t${:.2} ({:.2}%)",
            drawdown, drawdown_pct
        );

        // Show recent NAV values
        println!("\nRecent NAV (last 10 days):");
        for (date, nav) in nav_history.iter().rev().take(10).rev() {
            println!("  {}: ${:.2}", date, nav);
        }
    }

    // Trading summary
    println!("\n{}", "=".repeat(60));
    println!("TRADING ACTIVITY SUMMARY");
    println!("{}", "=".repeat(60));

    println!("Total trades: {}", total_trades);
    println!("Total commissions: ${:.2}", total_commission);
    println!("Total realized P&L: ${:.2}", total_realized_pnl);
    println!(
        "Net (realized - commissions): ${:.2}",
        total_realized_pnl + total_commission
    );

    // Top traded symbols
    if !trades_by_symbol.is_empty() {
        println!("\nTop 10 traded symbols by trade count:");
        let mut symbol_list: Vec<_> = trades_by_symbol.iter().collect();
        symbol_list.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

        println!(
            "  {:20} {:>8} {:>12} {:>12}",
            "Symbol", "Trades", "Volume", "P&L"
        );
        println!("  {}", "-".repeat(54));
        for (symbol, (count, volume, pnl)) in symbol_list.iter().take(10) {
            println!(
                "  {:20} {:>8} {:>12.0} {:>12.2}",
                symbol, count, volume, pnl
            );
        }

        // Top winners and losers
        symbol_list.sort_by(|a, b| {
            b.1 .2
                .partial_cmp(&a.1 .2)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let winners: Vec<_> = symbol_list
            .iter()
            .filter(|(_, (_, _, pnl))| *pnl > Decimal::ZERO)
            .take(5)
            .collect();
        if !winners.is_empty() {
            println!("\nTop 5 winning symbols:");
            for (symbol, (count, _, pnl)) in winners {
                println!("  {:20} {:>8} trades, P&L: ${:.2}", symbol, count, pnl);
            }
        }

        let losers: Vec<_> = symbol_list
            .iter()
            .filter(|(_, (_, _, pnl))| *pnl < Decimal::ZERO)
            .rev()
            .take(5)
            .collect();
        if !losers.is_empty() {
            println!("\nTop 5 losing symbols:");
            for (symbol, (count, _, pnl)) in losers {
                println!("  {:20} {:>8} trades, P&L: ${:.2}", symbol, count, pnl);
            }
        }
    }

    // Cash transactions summary
    if !cash_by_type.is_empty() {
        println!("\n{}", "=".repeat(60));
        println!("CASH TRANSACTIONS");
        println!("{}", "=".repeat(60));

        let mut cash_list: Vec<_> = cash_by_type.iter().collect();
        cash_list.sort_by(|a, b| {
            b.1 .1
                .abs()
                .partial_cmp(&a.1 .1.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        println!("{:40} {:>8} {:>14}", "Type", "Count", "Amount");
        println!("{}", "-".repeat(64));
        for (txn_type, (count, amount)) in cash_list {
            println!("{:40} {:>8} ${:>13.2}", txn_type, count, amount);
        }
    }

    // Position summary - filter to latest date
    let positions: Vec<Position> = if let Some(ld) = latest_date {
        all_positions
            .into_iter()
            .filter(|(date, _)| *date == ld)
            .map(|(_, pos)| pos)
            .collect()
    } else {
        Vec::new()
    };

    if !positions.is_empty() {
        println!("\n{}", "=".repeat(60));
        println!("CURRENT POSITIONS (from latest statement)");
        println!("{}", "=".repeat(60));

        if let Some(ld) = latest_date {
            println!("As of: {}", ld);
        }
        println!("Position count: {}", positions.len());

        let mut positions = positions;
        positions.sort_by(|a, b| {
            b.value
                .abs()
                .partial_cmp(&a.value.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        println!(
            "\n{:20} {:>10} {:>12} {:>12} {:>12}",
            "Symbol", "Qty", "Price", "Value", "Unreal P&L"
        );
        println!("{}", "-".repeat(70));

        for pos in positions.iter().take(15) {
            println!(
                "{:20} {:>10.0} {:>12.2} {:>12.2} {:>12.2}",
                pos.symbol, pos.quantity, pos.mark_price, pos.value, pos.unrealized_pnl
            );
        }

        if positions.len() > 15 {
            println!("... and {} more positions", positions.len() - 15);
        }

        let total_value: Decimal = positions.iter().map(|p| p.value).sum();
        let total_unrealized: Decimal = positions.iter().map(|p| p.unrealized_pnl).sum();

        println!("\nTotal position value: ${:.2}", total_value);
        println!("Total unrealized P&L: ${:.2}", total_unrealized);
    }

    // Monthly returns
    if nav_history.len() > 30 {
        println!("\n{}", "=".repeat(60));
        println!("MONTHLY RETURNS");
        println!("{}", "=".repeat(60));

        let mut monthly_returns: HashMap<(i32, u32), (Decimal, Decimal)> = HashMap::new();

        for (date, nav) in &nav_history {
            let key = (date.year(), date.month());
            let entry = monthly_returns.entry(key).or_insert((*nav, *nav));
            entry.1 = *nav;
        }

        let mut monthly_list: Vec<_> = monthly_returns.iter().collect();
        monthly_list.sort_by_key(|((year, month), _)| (*year, *month));

        println!(
            "{:10} {:>14} {:>14} {:>10}",
            "Month", "Start NAV", "End NAV", "Return"
        );
        println!("{}", "-".repeat(52));

        for ((year, month), (start, end)) in monthly_list {
            let ret = if *start != Decimal::ZERO {
                ((*end - *start) / *start) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };
            println!(
                "{:04}-{:02}    ${:>12.2} ${:>12.2} {:>9.2}%",
                year, month, start, end, ret
            );
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("Done!");

    Ok(())
}

fn parse_flex_date(s: &str) -> Option<NaiveDate> {
    // IB uses yyyyMMdd format
    if s.len() == 8 {
        let year = s[0..4].parse().ok()?;
        let month = s[4..6].parse().ok()?;
        let day = s[6..8].parse().ok()?;
        NaiveDate::from_ymd_opt(year, month, day)
    } else {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
    }
}

#[derive(Default, Clone)]
struct Position {
    symbol: String,
    quantity: Decimal,
    mark_price: Decimal,
    value: Decimal,
    unrealized_pnl: Decimal,
    asset_category: String,
}
