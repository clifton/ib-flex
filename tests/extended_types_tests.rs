use ib_flex::parse_activity_flex;
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_parse_extended_activity_statement() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse extended activity XML");

    // Verify basic fields
    assert_eq!(statement.account_id, "U1234567");
    assert_eq!(statement.from_date.to_string(), "2025-01-01");
    assert_eq!(statement.to_date.to_string(), "2025-01-31");

    // Verify core trade is parsed
    assert_eq!(statement.trades.items.len(), 1);
    let trade = &statement.trades.items[0];
    // Trade symbol is a required String field
    assert_eq!(&trade.symbol, "AAPL");
}

#[test]
fn test_account_information() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert!(statement.account_information.is_some());
    let acct_info = statement.account_information.as_ref().unwrap();

    assert_eq!(acct_info.account_id, "U1234567");
    assert_eq!(acct_info.account_type.as_deref(), Some("INDIVIDUAL"));
    assert_eq!(acct_info.acct_alias.as_deref(), Some("Main"));
    assert_eq!(acct_info.currency.as_deref(), Some("USD"));
    assert_eq!(acct_info.name.as_deref(), Some("John Doe"));
    assert_eq!(acct_info.master_name.as_deref(), Some("MASTER"));
}

#[test]
fn test_change_in_nav() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    // ChangeInNAV is now a single element, not a wrapper
    let nav = statement
        .change_in_nav
        .as_ref()
        .expect("Expected ChangeInNAV");

    assert_eq!(nav.account_id, "U1234567");
    assert_eq!(nav.from_date.to_string(), "2025-01-01");
    assert_eq!(nav.to_date.to_string(), "2025-01-31");
    assert_eq!(nav.starting_value, Decimal::from_str("100000.00").unwrap());
    assert_eq!(nav.ending_value, Decimal::from_str("107500.00").unwrap());
    // New field names matching production schema
    assert_eq!(nav.mtm, Some(Decimal::from_str("2500.00").unwrap()));
    assert_eq!(nav.realized, Some(Decimal::from_str("1500.00").unwrap()));
    assert_eq!(
        nav.change_in_unrealized,
        Some(Decimal::from_str("1000.00").unwrap())
    );
    assert_eq!(
        nav.deposits_withdrawals,
        Some(Decimal::from_str("5000.00").unwrap())
    );
    assert_eq!(nav.commissions, Some(Decimal::from_str("-50.00").unwrap()));
}

#[test]
fn test_equity_summary() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.equity_summary.items.len(), 1);
    let summary = &statement.equity_summary.items[0];

    assert_eq!(summary.account_id, "U1234567");
    assert_eq!(summary.report_date.to_string(), "2025-01-31");
    assert_eq!(summary.cash, Some(Decimal::from_str("50000.00").unwrap()));
    assert_eq!(summary.stock, Some(Decimal::from_str("40000.00").unwrap()));
    assert_eq!(
        summary.options,
        Some(Decimal::from_str("15000.00").unwrap())
    );
    assert_eq!(summary.bonds, Some(Decimal::from_str("2500.00").unwrap()));
    assert_eq!(summary.total, Some(Decimal::from_str("107500.00").unwrap()));
}

#[test]
fn test_cash_report() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.cash_report.items.len(), 1);
    let report = &statement.cash_report.items[0];

    assert_eq!(report.account_id, "U1234567");
    assert_eq!(report.currency, "USD");
    assert_eq!(report.from_date.to_string(), "2025-01-01");
    assert_eq!(report.to_date.to_string(), "2025-01-31");
    assert_eq!(report.starting_cash, Decimal::from_str("45000.00").unwrap());
    assert_eq!(
        report.commissions,
        Some(Decimal::from_str("-50.00").unwrap())
    );
    assert_eq!(report.deposits, Some(Decimal::from_str("5000.00").unwrap()));
    assert_eq!(report.withdrawals, Some(Decimal::from_str("0.00").unwrap()));
    assert_eq!(report.dividends, Some(Decimal::from_str("100.00").unwrap()));
    assert_eq!(report.ending_cash, Decimal::from_str("50050.00").unwrap());
}

#[test]
fn test_trade_confirms() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.trade_confirms.items.len(), 1);
    let confirm = &statement.trade_confirms.items[0];

    assert_eq!(confirm.account_id, "U1234567");
    assert_eq!(confirm.exec_id, "00012345.001");
    assert_eq!(confirm.order_id.as_deref(), Some("12345"));
    assert_eq!(confirm.trade_date.to_string(), "2025-01-15");
    assert_eq!(confirm.trade_time.as_deref(), Some("09:30:00"));
    assert_eq!(confirm.symbol, "AAPL");
    assert_eq!(confirm.quantity, Decimal::from_str("100").unwrap());
    assert_eq!(confirm.price, Decimal::from_str("150.00").unwrap());
}

#[test]
fn test_option_eae() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.option_eae.items.len(), 1);
    let eae = &statement.option_eae.items[0];

    assert_eq!(eae.account_id, "U1234567");
    assert_eq!(eae.transaction_id.as_deref(), Some("1001"));
    assert_eq!(eae.date.to_string(), "2025-01-20");
    assert_eq!(eae.symbol, "AAPL  250120P00145000");
    assert_eq!(eae.quantity, Decimal::from_str("1").unwrap());
    assert_eq!(eae.strike, Some(Decimal::from_str("145.00").unwrap()));
    assert_eq!(eae.underlying_symbol.as_deref(), Some("AAPL"));
}

#[test]
fn test_fx_transactions() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.fx_transactions.items.len(), 1);
    let fx = &statement.fx_transactions.items[0];

    assert_eq!(fx.account_id, "U1234567");
    assert_eq!(fx.transaction_id.as_deref(), Some("2001"));
    assert_eq!(fx.from_currency, "USD");
    assert_eq!(fx.to_currency, "EUR");
    assert_eq!(fx.quantity, Decimal::from_str("1000.00").unwrap());
    assert_eq!(fx.proceeds, Decimal::from_str("-920.50").unwrap());
    assert_eq!(
        fx.fx_rate_to_base,
        Some(Decimal::from_str("1.087").unwrap())
    );
}

#[test]
fn test_change_in_dividend_accruals() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.change_in_dividend_accruals.items.len(), 1);
    let div = &statement.change_in_dividend_accruals.items[0];

    assert_eq!(div.account_id, "U1234567");
    assert_eq!(div.symbol, "MSFT");
    assert_eq!(div.ex_date.to_string(), "2025-01-15");
    assert_eq!(
        div.pay_date.as_ref().map(|d| d.to_string()),
        Some("2025-02-15".to_string())
    );
    assert_eq!(div.gross_rate, Decimal::from_str("0.75").unwrap());
    assert_eq!(div.net_amount, Decimal::from_str("75.00").unwrap());
}

#[test]
fn test_open_dividend_accruals() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.open_dividend_accruals.items.len(), 1);
    let div = &statement.open_dividend_accruals.items[0];

    assert_eq!(div.account_id, "U1234567");
    assert_eq!(div.symbol, "GOOGL");
    assert_eq!(div.ex_date.to_string(), "2025-01-25");
    assert_eq!(
        div.pay_date.as_ref().map(|d| d.to_string()),
        Some("2025-02-25".to_string())
    );
    assert_eq!(div.quantity, Decimal::from_str("50").unwrap());
    assert_eq!(div.gross_rate, Decimal::from_str("0.50").unwrap());
}

#[test]
fn test_interest_accruals() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.interest_accruals.items.len(), 1);
    let interest = &statement.interest_accruals.items[0];

    assert_eq!(interest.account_id, "U1234567");
    assert_eq!(interest.currency, "USD");
    assert_eq!(interest.from_date.to_string(), "2025-01-01");
    assert_eq!(interest.to_date.to_string(), "2025-01-31");
    assert_eq!(
        interest.starting_balance,
        Decimal::from_str("50000.00").unwrap()
    );
    assert_eq!(
        interest.interest_accrued,
        Decimal::from_str("12.50").unwrap()
    );
    assert_eq!(
        interest.ending_balance,
        Decimal::from_str("50012.50").unwrap()
    );
}

#[test]
fn test_transfers() {
    let xml = include_str!("fixtures/activity_extended.xml");
    let statement = parse_activity_flex(xml).expect("Failed to parse");

    assert_eq!(statement.transfers.items.len(), 1);
    let transfer = &statement.transfers.items[0];

    assert_eq!(transfer.account_id, "U1234567");
    assert_eq!(transfer.symbol, "TSLA");
    assert_eq!(transfer.quantity, Decimal::from_str("25").unwrap());
    assert_eq!(transfer.direction.as_deref(), Some("IN"));
    assert_eq!(transfer.date.to_string(), "2025-01-05");
}

#[test]
fn test_extended_types_default_to_empty() {
    // Test with minimal XML that doesn't include extended sections
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse queryName="Test" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                           toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                <Trades />
                <OpenPositions />
                <CashTransactions />
                <CorporateActions />
                <SecuritiesInfo />
                <ConversionRates />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#;

    let statement = parse_activity_flex(xml).expect("Failed to parse");

    // All extended sections should default to empty
    assert!(statement.account_information.is_none());
    assert!(statement.change_in_nav.is_none()); // Single element, not wrapper
    assert_eq!(statement.equity_summary.items.len(), 0);
    assert_eq!(statement.cash_report.items.len(), 0);
    assert_eq!(statement.trade_confirms.items.len(), 0);
    assert_eq!(statement.option_eae.items.len(), 0);
    assert_eq!(statement.fx_transactions.items.len(), 0);
    assert_eq!(statement.change_in_dividend_accruals.items.len(), 0);
    assert_eq!(statement.open_dividend_accruals.items.len(), 0);
    assert_eq!(statement.interest_accruals.items.len(), 0);
    assert_eq!(statement.transfers.items.len(), 0);
}
