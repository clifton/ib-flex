//! Integration tests for daily portfolio statistics
//!
//! These tests verify parsing of comprehensive daily portfolio data
//! including positions, trades, cash flows, and multi-currency support.

use ib_flex::parse_activity_flex;
use ib_flex::{AssetCategory, BuySell};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Test parsing of comprehensive daily portfolio with compact date format
#[test]
fn test_parse_daily_portfolio() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse daily portfolio");

    // Verify account info
    assert_eq!(statement.account_id, "U1234567");
    assert_eq!(statement.from_date.to_string(), "2025-01-15");
    assert_eq!(statement.to_date.to_string(), "2025-01-15");

    // Verify we have positions
    assert_eq!(statement.positions.items.len(), 5);
    assert_eq!(statement.trades.items.len(), 3);
    assert_eq!(statement.cash_transactions.items.len(), 3);
    assert_eq!(statement.corporate_actions.items.len(), 1);
    assert_eq!(statement.securities_info.items.len(), 2);
    assert_eq!(statement.conversion_rates.items.len(), 3);
}

/// Test position calculations and multi-asset support
#[test]
fn test_position_summary() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let positions = &statement.positions.items;

    // Count by asset category
    let stocks: Vec<_> = positions
        .iter()
        .filter(|p| p.asset_category == AssetCategory::Stock)
        .collect();
    let options: Vec<_> = positions
        .iter()
        .filter(|p| p.asset_category == AssetCategory::Option)
        .collect();
    let futures: Vec<_> = positions
        .iter()
        .filter(|p| p.asset_category == AssetCategory::Future)
        .collect();

    assert_eq!(stocks.len(), 3);
    assert_eq!(options.len(), 1);
    assert_eq!(futures.len(), 1);

    // Verify long AAPL position
    let aapl = positions.iter().find(|p| p.symbol == "AAPL").unwrap();
    assert_eq!(aapl.quantity, Decimal::from(500));
    assert_eq!(aapl.mark_price, Decimal::from_str("185.50").unwrap());
    assert_eq!(aapl.position_value, Decimal::from_str("92750.00").unwrap());
    assert_eq!(aapl.currency, "USD");
    assert_eq!(aapl.side.as_deref(), Some("Long"));

    // Verify short TSLA position
    let tsla = positions.iter().find(|p| p.symbol == "TSLA").unwrap();
    assert_eq!(tsla.quantity, Decimal::from(-200));
    assert!(tsla.position_value < Decimal::ZERO);
    assert_eq!(tsla.side.as_deref(), Some("Short"));

    // Verify Canadian stock with FX
    let ry = positions.iter().find(|p| p.symbol == "RY").unwrap();
    assert_eq!(ry.currency, "CAD");
    assert_eq!(ry.fx_rate_to_base, Some(Decimal::from_str("0.72").unwrap()));

    // Verify option position
    let opt = positions
        .iter()
        .find(|p| p.asset_category == AssetCategory::Option)
        .unwrap();
    assert_eq!(opt.multiplier, Some(Decimal::from(100)));
    assert_eq!(opt.strike, Some(Decimal::from(190)));
    assert_eq!(opt.expiry.map(|d| d.to_string()), Some("2025-02-21".into()));

    // Verify futures position
    let fut = positions
        .iter()
        .find(|p| p.asset_category == AssetCategory::Future)
        .unwrap();
    assert_eq!(fut.symbol, "ESH5");
    assert_eq!(fut.multiplier, Some(Decimal::from(50)));
    assert_eq!(fut.quantity, Decimal::from(10));
}

/// Test total market value calculation
#[test]
fn test_total_market_value() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let total_value: Decimal = statement
        .positions
        .items
        .iter()
        .map(|p| p.position_value)
        .sum();

    // Sum of all position values
    // AAPL: 92750, TSLA: -50050, RY: 145500, Option: 16250, Future: 2975125
    // Total = 3179575
    assert!(total_value > Decimal::ZERO);
}

/// Test unrealized P&L aggregation
#[test]
fn test_unrealized_pnl() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let total_unrealized: Decimal = statement
        .positions
        .items
        .iter()
        .filter_map(|p| p.fifo_pnl_unrealized)
        .sum();

    // All positions have unrealized P&L
    // AAPL: 2750, TSLA: 1950, RY: 5500, Option: 3750, Future: 25125
    assert_eq!(total_unrealized, Decimal::from_str("39075.00").unwrap());
}

/// Test trade execution parsing
#[test]
fn test_trade_parsing() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let trades = &statement.trades.items;
    assert_eq!(trades.len(), 3);

    // Verify buy trade
    let buy_trade = trades
        .iter()
        .find(|t| matches!(t.buy_sell, Some(BuySell::Buy)) && t.symbol == "GOOGL")
        .unwrap();
    assert_eq!(buy_trade.quantity, Some(Decimal::from(100)));
    assert_eq!(buy_trade.price, Some(Decimal::from_str("175.50").unwrap()));
    assert_eq!(buy_trade.proceeds, Decimal::from_str("-17550.00").unwrap());
    assert_eq!(buy_trade.commission, Decimal::from_str("-1.25").unwrap());
    assert_eq!(buy_trade.trade_date.to_string(), "2025-01-15");
    assert_eq!(buy_trade.settle_date.to_string(), "2025-01-17");

    // Verify sell trade with realized P&L
    let sell_trade = trades
        .iter()
        .find(|t| matches!(t.buy_sell, Some(BuySell::Sell)))
        .unwrap();
    assert_eq!(sell_trade.symbol, "MSFT");
    assert_eq!(sell_trade.quantity, Some(Decimal::from(-50)));
    assert!(sell_trade.proceeds > Decimal::ZERO);
    assert_eq!(
        sell_trade.fifo_pnl_realized,
        Some(Decimal::from_str("511.75").unwrap())
    );

    // Verify option trade
    let opt_trade = trades
        .iter()
        .find(|t| t.asset_category == AssetCategory::Option)
        .unwrap();
    assert_eq!(opt_trade.multiplier, Some(Decimal::from(100)));
    assert_eq!(opt_trade.strike, Some(Decimal::from(600)));
    assert_eq!(
        opt_trade.expiry.map(|d| d.to_string()),
        Some("2025-02-21".into())
    );
}

/// Test trade summary statistics
#[test]
fn test_trade_summary() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let trades = &statement.trades.items;

    // Total commissions
    let total_commissions: Decimal = trades.iter().map(|t| t.commission).sum();
    assert_eq!(total_commissions, Decimal::from_str("-8.50").unwrap());

    // Total realized P&L
    let total_realized: Decimal = trades.iter().filter_map(|t| t.fifo_pnl_realized).sum();
    assert_eq!(total_realized, Decimal::from_str("511.75").unwrap());

    // Buy count
    let buys = trades
        .iter()
        .filter(|t| matches!(t.buy_sell, Some(BuySell::Buy)))
        .count();
    assert_eq!(buys, 2);

    // Sell count
    let sells = trades
        .iter()
        .filter(|t| matches!(t.buy_sell, Some(BuySell::Sell)))
        .count();
    assert_eq!(sells, 1);
}

/// Test cash transactions parsing
#[test]
fn test_cash_transactions() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let cash = &statement.cash_transactions.items;
    assert_eq!(cash.len(), 3);

    // Dividend
    let dividend = cash
        .iter()
        .find(|c| c.transaction_type == "Dividends")
        .unwrap();
    assert_eq!(dividend.symbol.as_deref(), Some("AAPL"));
    assert_eq!(dividend.amount, Decimal::from_str("125.00").unwrap());

    // Withholding tax
    let tax = cash
        .iter()
        .find(|c| c.transaction_type == "Withholding Tax")
        .unwrap();
    assert_eq!(tax.amount, Decimal::from_str("-18.75").unwrap());

    // Interest
    let interest = cash
        .iter()
        .find(|c| c.transaction_type == "Broker Interest Paid")
        .unwrap();
    assert!(interest.amount < Decimal::ZERO);
}

/// Test cash flow summary
#[test]
fn test_cash_flow_summary() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let cash = &statement.cash_transactions.items;

    // Total dividends
    let dividends: Decimal = cash
        .iter()
        .filter(|c| c.transaction_type.contains("Dividend"))
        .map(|c| c.amount)
        .sum();
    assert_eq!(dividends, Decimal::from_str("125.00").unwrap());

    // Total taxes
    let taxes: Decimal = cash
        .iter()
        .filter(|c| c.transaction_type.contains("Tax"))
        .map(|c| c.amount)
        .sum();
    assert_eq!(taxes, Decimal::from_str("-18.75").unwrap());
}

/// Test corporate actions
#[test]
fn test_corporate_actions() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let actions = &statement.corporate_actions.items;
    assert_eq!(actions.len(), 1);

    let split = &actions[0];
    assert_eq!(split.symbol, "XYZ");
    assert_eq!(split.action_type, "FS"); // Forward Split
    assert_eq!(split.quantity, Some(Decimal::from(100)));
}

/// Test conversion rates for multi-currency
#[test]
fn test_conversion_rates() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let rates = &statement.conversion_rates.items;
    assert_eq!(rates.len(), 3);

    // Find CAD rate
    let cad = rates.iter().find(|r| r.from_currency == "CAD").unwrap();
    assert_eq!(cad.to_currency, "USD");
    assert_eq!(cad.rate, Decimal::from_str("0.72").unwrap());
    assert_eq!(cad.report_date.to_string(), "2025-01-15");

    // Find EUR rate
    let eur = rates.iter().find(|r| r.from_currency == "EUR").unwrap();
    assert_eq!(eur.rate, Decimal::from_str("1.085").unwrap());
}

/// Test currency exposure calculation
#[test]
fn test_currency_exposure() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    use std::collections::HashMap;
    let mut by_currency: HashMap<&str, Decimal> = HashMap::new();

    for pos in &statement.positions.items {
        *by_currency.entry(&pos.currency).or_insert(Decimal::ZERO) += pos.position_value;
    }

    assert!(by_currency.contains_key("USD"));
    assert!(by_currency.contains_key("CAD"));

    let usd_total = by_currency.get("USD").unwrap();
    let cad_total = by_currency.get("CAD").unwrap();

    assert!(*usd_total != Decimal::ZERO); // Has exposure
    assert!(*cad_total > Decimal::ZERO); // Long CAD
}

/// Test securities info parsing
#[test]
fn test_securities_info() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let securities = &statement.securities_info.items;
    assert_eq!(securities.len(), 2);

    // Stock security
    let aapl = securities.iter().find(|s| s.symbol == "AAPL").unwrap();
    assert_eq!(aapl.asset_category, AssetCategory::Stock);
    assert_eq!(aapl.cusip.as_deref(), Some("037833100"));
    assert_eq!(aapl.isin.as_deref(), Some("US0378331005"));
    assert_eq!(aapl.figi.as_deref(), Some("BBG000B9XRY4"));

    // Option security
    let opt = securities
        .iter()
        .find(|s| s.asset_category == AssetCategory::Option)
        .unwrap();
    assert_eq!(opt.underlying_symbol.as_deref(), Some("AAPL"));
    assert_eq!(opt.multiplier, Some(Decimal::from(100)));
    assert_eq!(opt.strike, Some(Decimal::from(190)));
}

/// Test extended types - NAV change
#[test]
fn test_nav_change() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let nav = &statement.change_in_nav.items;
    assert_eq!(nav.len(), 1);

    let change = &nav[0];
    assert_eq!(change.from_date.to_string(), "2025-01-15");
    assert_eq!(
        change.starting_value,
        Decimal::from_str("420000.00").unwrap()
    );
    assert_eq!(change.ending_value, Decimal::from_str("425526.00").unwrap());

    // Day's return
    let day_return = change.ending_value - change.starting_value;
    assert_eq!(day_return, Decimal::from_str("5526.00").unwrap());
}

/// Test extended types - equity summary
#[test]
fn test_equity_summary() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let summary = &statement.equity_summary.items;
    assert_eq!(summary.len(), 1);

    let eq = &summary[0];
    assert_eq!(eq.report_date.to_string(), "2025-01-15");
    assert_eq!(eq.cash, Some(Decimal::from_str("150000.50").unwrap()));
    assert_eq!(eq.stock, Some(Decimal::from_str("250000.00").unwrap()));
    assert_eq!(eq.options, Some(Decimal::from_str("25000.00").unwrap()));
    assert_eq!(eq.total, Some(Decimal::from_str("425526.00").unwrap()));
}

/// Test extended types - interest accruals
#[test]
fn test_interest_accruals() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let accruals = &statement.interest_accruals.items;
    assert_eq!(accruals.len(), 2);

    let usd = accruals.iter().find(|a| a.currency == "USD").unwrap();
    assert_eq!(usd.starting_balance, Decimal::from_str("20.25").unwrap());
    assert_eq!(usd.interest_accrued, Decimal::from_str("5.25").unwrap());
    assert_eq!(usd.ending_balance, Decimal::from_str("25.50").unwrap());
}

/// Test dividend accruals
#[test]
fn test_dividend_accruals() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    // Open dividend accruals
    let open = &statement.open_dividend_accruals.items;
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].symbol, "MSFT");
    assert_eq!(open[0].gross_rate, Decimal::from_str("0.75").unwrap());

    // Change in dividend accruals
    let change = &statement.change_in_dividend_accruals.items;
    assert_eq!(change.len(), 1);
    assert_eq!(change[0].symbol, "AAPL");
}

/// Test cash report
#[test]
fn test_cash_report() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let report = &statement.cash_report.items;
    assert_eq!(report.len(), 2);

    let usd = report.iter().find(|r| r.currency == "USD").unwrap();
    assert_eq!(usd.starting_cash, Decimal::from_str("145000.00").unwrap());
    assert_eq!(usd.commissions, Some(Decimal::from_str("-8.50").unwrap()));
    assert_eq!(usd.dividends, Some(Decimal::from_str("125.00").unwrap()));
    assert_eq!(usd.ending_cash, Decimal::from_str("150000.50").unwrap());
}

/// Test FX transactions
#[test]
fn test_fx_transactions() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let fx = &statement.fx_transactions.items;
    assert_eq!(fx.len(), 1);

    let txn = &fx[0];
    assert_eq!(txn.from_currency, "CAD");
    assert_eq!(txn.to_currency, "USD");
    assert_eq!(txn.quantity, Decimal::from(10000));
    assert_eq!(txn.proceeds, Decimal::from_str("-7200.00").unwrap());
}

/// Test account information
#[test]
fn test_account_information() {
    let xml = include_str!("fixtures/activity_daily_portfolio.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    let info = statement.account_information.as_ref().unwrap();
    assert_eq!(info.account_id, "U1234567");
    assert_eq!(info.acct_alias.as_deref(), Some("TestFund"));
    assert_eq!(info.currency.as_deref(), Some("USD"));
    assert_eq!(info.name.as_deref(), Some("Test Fund LP"));
}
