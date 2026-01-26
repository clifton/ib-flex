//! Comprehensive Daily Activity Analysis Example
//!
//! Complete analysis of IB FLEX Activity statements - handles both single-day
//! statements and multi-day backfill files automatically.
//!
//! ## Features
//!
//! 1. **Parse Activity FLEX statements** (single or multi-statement backfills)
//! 2. **Position analysis** - Long/short exposure, unrealized P&L
//! 3. **Trade analysis** - By asset class, symbol, date; top winners/losers
//! 4. **Derivative analysis** - Options by underlying with P&L
//! 5. **Cash flow analysis** - Dividends, interest, fees, withholding tax
//! 6. **Corporate actions** - Splits, dividends, spinoffs
//! 7. **Database integration** - PostgreSQL/sqlx example pattern
//!
//! ## Usage
//!
//! ```bash
//! # Single-day statement
//! cargo run --example comprehensive_daily_analysis -- statement.xml
//!
//! # Multi-day backfill
//! cargo run --example comprehensive_daily_analysis -- backfill.xml
//!
//! # Uses default test fixture if no file specified
//! cargo run --example comprehensive_daily_analysis
//! ```

use ib_flex::{parse_activity_flex_all, AssetCategory, BuySell};
use rust_decimal::Decimal;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let xml_path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "tests/fixtures/activity_daily_portfolio.xml"
    };

    println!("=== IB-FLEX Comprehensive Activity Analysis ===\n");
    println!("Loading FLEX statement: {}\n", xml_path);

    // Read and parse all statements
    let xml = std::fs::read_to_string(xml_path)?;
    let statements = parse_activity_flex_all(&xml)?;

    if statements.is_empty() {
        println!("No statements found in file");
        return Ok(());
    }

    // Detect single vs multi-statement
    let is_backfill = statements.len() > 1;
    println!(
        "Parsed {} statement{}\n",
        statements.len(),
        if is_backfill { "s" } else { "" }
    );

    // Get date range
    let first_date = statements.first().unwrap().from_date;
    let last_date = statements.last().unwrap().to_date;
    let account_id = &statements[0].account_id;

    println!("======================================================================");
    println!(
        "{}",
        if is_backfill {
            "BACKFILL SUMMARY"
        } else {
            "STATEMENT SUMMARY"
        }
    );
    println!("======================================================================");
    println!("Account:        {}", account_id);
    if is_backfill {
        println!("Date Range:     {} to {}", first_date, last_date);
        println!("Total Days:     {}", statements.len());
    } else {
        println!("Date:           {}", first_date);
        println!("Generated:      {}", statements[0].when_generated);
    }
    println!();

    // Aggregate all trades across all statements
    let mut all_trades = Vec::new();
    let mut all_positions = Vec::new();
    let mut all_cash_txns = Vec::new();
    let mut all_corp_actions = Vec::new();

    for stmt in &statements {
        all_trades.extend(stmt.trades.items.iter());
        all_positions.extend(stmt.positions.items.iter());
        all_cash_txns.extend(stmt.cash_transactions.items.iter());
        all_corp_actions.extend(stmt.corporate_actions.items.iter());
    }

    // ==================== POSITIONS (from latest statement) ====================
    let latest_statement = statements.last().unwrap();
    let positions = &latest_statement.positions.items;

    if !positions.is_empty() {
        println!("======================================================================");
        println!("POSITIONS (as of {})", latest_statement.to_date);
        println!("======================================================================");
        println!("Total Positions: {}\n", positions.len());

        // Calculate long/short exposure
        let long_value: Decimal = positions
            .iter()
            .filter(|p| p.quantity > Decimal::ZERO)
            .map(|p| p.position_value)
            .sum();

        let short_value: Decimal = positions
            .iter()
            .filter(|p| p.quantity < Decimal::ZERO)
            .map(|p| p.position_value)
            .sum();

        let long_count = positions
            .iter()
            .filter(|p| p.quantity > Decimal::ZERO)
            .count();
        let short_count = positions
            .iter()
            .filter(|p| p.quantity < Decimal::ZERO)
            .count();

        let total_unrealized_pnl: Decimal =
            positions.iter().filter_map(|p| p.fifo_pnl_unrealized).sum();

        println!("Exposure Summary:");
        println!("  Long:  {} positions, ${:>16.2}", long_count, long_value);
        println!("  Short: {} positions, ${:>16.2}", short_count, short_value);
        println!(
            "  Net:                    ${:>16.2}",
            long_value + short_value
        );
        println!();
        println!("  Total Unrealized P&L:   ${:>16.2}", total_unrealized_pnl);
        println!();

        // Top positions by absolute value
        let mut sorted_positions: Vec<_> = positions.iter().collect();
        sorted_positions.sort_by(|a, b| b.position_value.abs().cmp(&a.position_value.abs()));

        println!("Top 20 Positions by Value:");
        for pos in sorted_positions.iter().take(20) {
            let pnl = pos.fifo_pnl_unrealized.unwrap_or_default();
            let pnl_pct = if let Some(cost_basis) = pos.cost_basis_money {
                if cost_basis != Decimal::ZERO {
                    (pnl / cost_basis) * Decimal::from(100)
                } else {
                    Decimal::ZERO
                }
            } else {
                Decimal::ZERO
            };

            println!(
                "  {:20} {:>10} @ ${:>10.2}  Value: ${:>14.2}  P&L: ${:>10.2} ({:>6.2}%)",
                pos.symbol, pos.quantity, pos.mark_price, pos.position_value, pnl, pnl_pct
            );
        }
        println!();

        // Positions by asset class
        let mut by_asset: HashMap<AssetCategory, (usize, Decimal)> = HashMap::new();
        for pos in positions.iter() {
            let entry = by_asset
                .entry(pos.asset_category)
                .or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += pos.position_value;
        }

        let total_value: Decimal = positions.iter().map(|p| p.position_value).sum();

        println!("Positions by Asset Class:");
        let mut asset_breakdown: Vec<_> = by_asset.iter().collect();
        asset_breakdown.sort_by(|a, b| b.1 .1.abs().cmp(&a.1 .1.abs()));

        for (asset, (count, value)) in asset_breakdown {
            let pct = if total_value != Decimal::ZERO {
                (value / total_value) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };
            println!(
                "  {:12} {:>4} positions  ${:>16.2}  ({:>5.1}%)",
                format!("{:?}", asset),
                count,
                value,
                pct
            );
        }
        println!();
    }

    // ==================== TRADE ANALYSIS ====================
    println!("======================================================================");
    println!("TRADE ANALYSIS");
    println!("======================================================================");
    println!("Total Trades:   {}\n", all_trades.len());

    if !all_trades.is_empty() {
        let total_commission: Decimal = all_trades.iter().filter_map(|t| t.commission).sum();
        let total_realized_pnl: Decimal =
            all_trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();
        let total_proceeds: Decimal = all_trades.iter().filter_map(|t| t.proceeds).sum();

        println!("Financial Summary:");
        println!("  Total Proceeds:      ${:>16.2}", total_proceeds);
        println!("  Total Commissions:   ${:>16.2}", total_commission);
        println!("  Total Realized P&L:  ${:>16.2}", total_realized_pnl);
        println!();

        // By asset class
        let mut by_asset: HashMap<AssetCategory, TradeStats> = HashMap::new();
        for trade in &all_trades {
            let stats = by_asset.entry(trade.asset_category).or_default();
            stats.count += 1;
            if let Some(pnl) = trade.fifo_pnl_realized {
                stats.realized_pnl += pnl;
            }
            if let Some(comm) = trade.commission {
                stats.commission += comm;
            }
            if let Some(proceeds) = trade.proceeds {
                stats.proceeds += proceeds;
            }
        }

        println!("By Asset Class:");
        let mut asset_stats: Vec<_> = by_asset.iter().collect();
        asset_stats.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (asset, stats) in asset_stats {
            let avg_comm = if stats.count > 0 {
                stats.commission / Decimal::from(stats.count)
            } else {
                Decimal::ZERO
            };
            println!(
                "  {:12} {:>6} trades  P&L: ${:>14.2}  Proceeds: ${:>14.2}  Comm: ${:>12.2}  Avg: ${:>6.2}",
                format!("{:?}", asset),
                stats.count,
                stats.realized_pnl,
                stats.proceeds,
                stats.commission,
                avg_comm
            );
        }
        println!();

        // By symbol
        let mut by_symbol: HashMap<String, SymbolStats> = HashMap::new();
        for trade in &all_trades {
            let stats = by_symbol.entry(trade.symbol.clone()).or_default();
            stats.count += 1;
            if let Some(pnl) = trade.fifo_pnl_realized {
                stats.realized_pnl += pnl;
            }
            if let Some(comm) = trade.commission {
                stats.commission += comm;
            }
        }

        println!("Top 20 Symbols by Trade Count:");
        let mut symbol_stats: Vec<_> = by_symbol.iter().collect();
        symbol_stats.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (symbol, stats) in symbol_stats.iter().take(20) {
            println!(
                "  {:20} {:>6} trades  P&L: ${:>12.2}  Comm: ${:>10.2}",
                symbol, stats.count, stats.realized_pnl, stats.commission
            );
        }
        println!();

        // Daily volume
        let mut by_date: HashMap<String, usize> = HashMap::new();
        for trade in &all_trades {
            if let Some(date) = trade.trade_date {
                *by_date.entry(date.to_string()).or_insert(0) += 1;
            }
        }

        println!("Top 10 Trading Days:");
        let mut date_stats: Vec<_> = by_date.iter().collect();
        date_stats.sort_by(|a, b| b.1.cmp(a.1));

        for (date, count) in date_stats.iter().take(10) {
            println!("  {}  {:>6} trades", date, count);
        }
        println!();

        // Buy vs Sell
        let buys = all_trades
            .iter()
            .filter(|t| matches!(t.buy_sell, Some(BuySell::Buy)))
            .count();
        let sells = all_trades
            .iter()
            .filter(|t| matches!(t.buy_sell, Some(BuySell::Sell)))
            .count();

        println!("Trade Direction:");
        println!(
            "  Buys:  {:>6} ({:>5.1}%)",
            buys,
            (buys as f64 / all_trades.len() as f64) * 100.0
        );
        println!(
            "  Sells: {:>6} ({:>5.1}%)",
            sells,
            (sells as f64 / all_trades.len() as f64) * 100.0
        );
        println!();

        // Top trades by P&L
        let mut trades_with_pnl: Vec<_> = all_trades
            .iter()
            .filter(|t| t.fifo_pnl_realized.is_some())
            .collect();
        trades_with_pnl.sort_by(|a, b| {
            b.fifo_pnl_realized
                .unwrap_or_default()
                .cmp(&a.fifo_pnl_realized.unwrap_or_default())
        });

        if !trades_with_pnl.is_empty() {
            println!("Top 10 Best Trades by P&L:");
            for trade in trades_with_pnl.iter().take(10) {
                let side = match trade.buy_sell {
                    Some(BuySell::Buy) => "BUY ",
                    Some(BuySell::Sell) => "SELL",
                    _ => "????",
                };
                println!(
                    "  {} {:20} {:>10} @ ${:>10.2}  P&L: ${:>12.2}",
                    side,
                    trade.symbol,
                    trade.quantity.unwrap_or_default(),
                    trade.price.unwrap_or_default(),
                    trade.fifo_pnl_realized.unwrap_or_default()
                );
            }
            println!();

            println!("Top 10 Worst Trades by P&L:");
            for trade in trades_with_pnl.iter().rev().take(10) {
                let side = match trade.buy_sell {
                    Some(BuySell::Buy) => "BUY ",
                    Some(BuySell::Sell) => "SELL",
                    _ => "????",
                };
                println!(
                    "  {} {:20} {:>10} @ ${:>10.2}  P&L: ${:>12.2}",
                    side,
                    trade.symbol,
                    trade.quantity.unwrap_or_default(),
                    trade.price.unwrap_or_default(),
                    trade.fifo_pnl_realized.unwrap_or_default()
                );
            }
            println!();
        }
    }

    // ==================== DERIVATIVE ANALYSIS ====================
    let option_trades: Vec<_> = all_trades
        .iter()
        .filter(|t| matches!(t.asset_category, AssetCategory::Option))
        .collect();

    if !option_trades.is_empty() {
        println!("======================================================================");
        println!("DERIVATIVE ANALYSIS (Options)");
        println!("======================================================================");
        println!("Total Option Trades: {}\n", option_trades.len());

        // Group by underlying
        let mut by_underlying: HashMap<String, Vec<&&ib_flex::Trade>> = HashMap::new();
        for trade in &option_trades {
            let underlying = trade
                .underlying_symbol
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_string());
            by_underlying.entry(underlying).or_default().push(trade);
        }

        println!("Top 20 Underlyings:");
        let mut underlying_stats: Vec<_> = by_underlying.iter().collect();
        underlying_stats.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (underlying, trades) in underlying_stats.iter().take(20) {
            let total_contracts: Decimal = trades.iter().filter_map(|t| t.quantity).sum();
            let total_pnl: Decimal = trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();
            println!(
                "  {:10} - {:>4} trades, {:>8} contracts, P&L: ${:>12.2}",
                underlying,
                trades.len(),
                total_contracts,
                total_pnl
            );
        }
        println!();
    }

    // ==================== CASH FLOW ANALYSIS ====================
    println!("======================================================================");
    println!("CASH FLOW ANALYSIS");
    println!("======================================================================");
    println!("Total Transactions: {}\n", all_cash_txns.len());

    if !all_cash_txns.is_empty() {
        let mut by_type: HashMap<String, (usize, Decimal)> = HashMap::new();
        for txn in &all_cash_txns {
            let txn_type = txn
                .transaction_type
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
            let entry = by_type.entry(txn_type).or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += txn.amount;
        }

        println!("By Transaction Type:");
        let mut types: Vec<_> = by_type.iter().collect();
        types.sort_by(|a, b| b.1 .1.abs().cmp(&a.1 .1.abs()));

        for (txn_type, (count, amount)) in types {
            println!("  {:40} {:>6} txn  ${:>14.2}", txn_type, count, amount);
        }
        println!();

        // Category summaries
        let dividends: Decimal = all_cash_txns
            .iter()
            .filter(|t| {
                t.transaction_type
                    .as_ref()
                    .map(|s| s.contains("Dividend"))
                    .unwrap_or(false)
            })
            .map(|t| t.amount)
            .sum();

        let withholding: Decimal = all_cash_txns
            .iter()
            .filter(|t| {
                t.transaction_type
                    .as_ref()
                    .map(|s| s.contains("Withholding"))
                    .unwrap_or(false)
            })
            .map(|t| t.amount)
            .sum();

        let interest: Decimal = all_cash_txns
            .iter()
            .filter(|t| {
                t.transaction_type
                    .as_ref()
                    .map(|s| s.contains("Interest"))
                    .unwrap_or(false)
            })
            .map(|t| t.amount)
            .sum();

        let fees: Decimal = all_cash_txns
            .iter()
            .filter(|t| {
                t.transaction_type
                    .as_ref()
                    .map(|s| s.contains("Fee") || s.contains("Commission"))
                    .unwrap_or(false)
            })
            .map(|t| t.amount)
            .sum();

        println!("Category Summaries:");
        if dividends != Decimal::ZERO {
            println!("  Dividends (gross):    ${:>14.2}", dividends);
        }
        if withholding != Decimal::ZERO {
            println!("  Withholding Tax:      ${:>14.2}", withholding);
            if dividends != Decimal::ZERO {
                println!("  Dividends (net):      ${:>14.2}", dividends + withholding);
            }
        }
        if interest != Decimal::ZERO {
            println!("  Interest:             ${:>14.2}", interest);
        }
        if fees != Decimal::ZERO {
            println!("  Fees:                 ${:>14.2}", fees);
        }
        println!();
    }

    // ==================== CORPORATE ACTIONS ====================
    if !all_corp_actions.is_empty() {
        println!("======================================================================");
        println!("CORPORATE ACTIONS");
        println!("======================================================================");
        println!("Total Actions: {}\n", all_corp_actions.len());

        let mut by_type: HashMap<String, usize> = HashMap::new();
        for action in &all_corp_actions {
            let action_type = action
                .action_type
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
            *by_type.entry(action_type).or_insert(0) += 1;
        }

        println!("By Action Type:");
        let mut types: Vec<_> = by_type.iter().collect();
        types.sort_by(|a, b| b.1.cmp(a.1));

        for (action_type, count) in types {
            println!("  {:40} {:>6} actions", action_type, count);
        }
        println!();

        if all_corp_actions.len() <= 20 {
            println!("All Corporate Actions:");
            for action in &all_corp_actions {
                let action_type_str = action.action_type.as_deref().unwrap_or("N/A");
                println!(
                    "  {} {:12} - {:30} Qty: {:>10}",
                    action.report_date,
                    action.symbol,
                    action_type_str,
                    action.quantity.unwrap_or_default(),
                );
            }
        } else {
            println!("Sample Corporate Actions (first 20):");
            for action in all_corp_actions.iter().take(20) {
                let action_type_str = action.action_type.as_deref().unwrap_or("N/A");
                println!(
                    "  {} {:12} - {:30} Qty: {:>10}",
                    action.report_date,
                    action.symbol,
                    action_type_str,
                    action.quantity.unwrap_or_default(),
                );
            }
        }
        println!();
    }

    // ==================== DATABASE INTEGRATION ====================
    println!("======================================================================");
    println!("DATABASE INTEGRATION PATTERN");
    println!("======================================================================");
    println!("Example PostgreSQL/sqlx integration pattern:\n");
    println!("```rust");
    println!("use sqlx::PgPool;");
    println!("use ib_flex::{{parse_activity_flex_all, Trade}};");
    println!();
    println!("async fn import_backfill(pool: &PgPool, xml: &str) -> Result<(), Box<dyn std::error::Error>> {{");
    println!("    let statements = parse_activity_flex_all(xml)?;");
    println!("    ");
    println!("    for stmt in statements {{");
    println!("        for trade in &stmt.trades.items {{");
    println!("            // Idempotent insert - skip duplicates");
    println!("            sqlx::query!(");
    println!("                r#\"");
    println!("                INSERT INTO trades (");
    println!("                    transaction_id, account_id, trade_date,");
    println!("                    symbol, asset_category, buy_sell,");
    println!("                    quantity, price, proceeds, commission,");
    println!("                    fifo_pnl_realized, currency");
    println!("                )");
    println!("                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)");
    println!("                ON CONFLICT (transaction_id) DO NOTHING");
    println!("                \"#,");
    println!("                trade.transaction_id,");
    println!("                trade.account_id,");
    println!("                trade.trade_date,");
    println!("                trade.symbol,");
    println!("                format!(\"{{:?}}\", trade.asset_category),");
    println!("                trade.buy_sell.as_ref().map(|b| format!(\"{{:?}}\", b)),");
    println!("                trade.quantity,");
    println!("                trade.price,");
    println!("                trade.proceeds,");
    println!("                trade.commission,");
    println!("                trade.fifo_pnl_realized,");
    println!("                trade.currency");
    println!("            )");
    println!("            .execute(pool)");
    println!("            .await?;");
    println!("        }}");
    println!("    }}");
    println!("    Ok(())");
    println!("}}");
    println!("```");
    println!();
    println!("Key Points:");
    println!("  • Use transaction_id as primary key for idempotency");
    println!("  • ON CONFLICT DO NOTHING prevents duplicate insertions");
    println!("  • All Decimal types map to NUMERIC(20, 8)");
    println!("  • Enums stored as VARCHAR or PostgreSQL ENUM types");
    println!("  • Process statements in order to maintain chronology");
    println!();

    // ==================== SUMMARY ====================
    println!("======================================================================");
    println!("SUMMARY");
    println!("======================================================================");
    println!("Date Range:         {} to {}", first_date, last_date);
    println!("Total Statements:   {}", statements.len());
    println!("Total Trades:       {}", all_trades.len());
    println!("Total Cash Txns:    {}", all_cash_txns.len());
    println!("Total Corp Actions: {}", all_corp_actions.len());

    if !all_trades.is_empty() {
        let total_pnl: Decimal = all_trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();
        let total_commission: Decimal = all_trades.iter().filter_map(|t| t.commission).sum();
        println!("Total Realized P&L: ${:>14.2}", total_pnl);
        println!("Total Commissions:  ${:>14.2}", total_commission);
        println!(
            "Net P&L:            ${:>14.2}",
            total_pnl + total_commission
        );
    }
    println!();

    Ok(())
}

#[derive(Default)]
struct TradeStats {
    count: usize,
    realized_pnl: Decimal,
    commission: Decimal,
    proceeds: Decimal,
}

#[derive(Default)]
struct SymbolStats {
    count: usize,
    realized_pnl: Decimal,
    commission: Decimal,
}
