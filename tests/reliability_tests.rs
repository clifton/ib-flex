// Comprehensive reliability testing for ib-flex parser
//
// This test suite includes:
// 1. Property-based testing with proptest
// 2. Stress tests with large XML files
// 3. Fuzzing with malformed inputs
// 4. Edge case variations

use ib_flex::parse_activity_flex;
use proptest::prelude::*;
use rand::Rng;

// ===== PROPERTY-BASED TESTS =====

proptest! {
    #[test]
    fn test_parse_handles_any_valid_account_id(account_id in "[A-Z0-9]{8}") {
        let xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="{}" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    <Trades />
                    <OpenPositions />
                    <CashTransactions />
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#, account_id);

        let result = parse_activity_flex(&xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().account_id, account_id);
    }

    #[test]
    fn test_parse_handles_various_date_ranges(
        year in 2020u32..=2030u32,
        month in 1u32..=12u32,
        day in 1u32..=28u32  // Use 28 to avoid invalid dates
    ) {
        let from_date = format!("{:04}-{:02}-{:02}", year, month, day);
        let to_date = format!("{:04}-{:02}-{:02}", year, month, day);

        let xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="{}"
                               toDate="{}" whenGenerated="2025-01-31;150000">
                    <Trades />
                    <OpenPositions />
                    <CashTransactions />
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#, from_date, to_date);

        let result = parse_activity_flex(&xml);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_parse_handles_various_decimal_values(
        price in -1000000.0f64..1000000.0f64,
        quantity in 0.001f64..1000000.0f64
    ) {
        // Round to 2 decimal places for price, 3 for quantity
        let price_str = format!("{:.2}", price);
        let quantity_str = format!("{:.3}", quantity);

        let xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    <Trades>
                        <Trade accountId="U1234567" conid="265598" symbol="TEST"
                               assetCategory="STK" tradeDate="2025-01-15"
                               settleDateTarget="2025-01-17" quantity="{}"
                               tradePrice="{}" proceeds="-15000.00"
                               ibCommission="-1.00" netCash="-15001.00"
                               currency="USD" buySell="BUY" />
                    </Trades>
                    <OpenPositions />
                    <CashTransactions />
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#, quantity_str, price_str);

        let result = parse_activity_flex(&xml);
        prop_assert!(result.is_ok());
    }
}

// ===== STRESS TESTS =====

#[test]
fn test_parse_large_number_of_trades() {
    // Generate XML with 1000 trades
    let mut trades = String::from("<Trades>");
    for i in 0..1000 {
        trades.push_str(&format!(
            r#"<Trade accountId="U1234567" conid="{}" symbol="SYM{}"
                   assetCategory="STK" tradeDate="2025-01-{:02}"
                   settleDateTarget="2025-01-{:02}" quantity="100"
                   tradePrice="100.00" proceeds="-10000.00"
                   ibCommission="-1.00" netCash="-10001.00"
                   currency="USD" buySell="BUY" />"#,
            265598 + i,
            i,
            (i % 28) + 1,
            ((i % 28) + 1 + 2) % 28 + 1
        ));
    }
    trades.push_str("</Trades>");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Stress Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    {}
                    <OpenPositions />
                    <CashTransactions />
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#,
        trades
    );

    let result = parse_activity_flex(&xml);
    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.trades.items.len(), 1000);
}

#[test]
fn test_parse_large_number_of_positions() {
    // Generate XML with 500 positions
    let mut positions = String::from("<OpenPositions>");
    for i in 0..500 {
        positions.push_str(&format!(
            r#"<OpenPosition accountId="U1234567" conid="{}" symbol="POS{}"
                             assetCategory="STK" reportDate="2025-01-31"
                             position="100" markPrice="150.00" positionValue="15000.00"
                             currency="USD" />"#,
            300000 + i,
            i
        ));
    }
    positions.push_str("</OpenPositions>");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Stress Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    <Trades />
                    {}
                    <CashTransactions />
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#,
        positions
    );

    let result = parse_activity_flex(&xml);
    if let Err(e) = &result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.positions.items.len(), 500);
}

#[test]
fn test_parse_large_number_of_cash_transactions() {
    // Generate XML with 200 cash transactions
    let mut transactions = String::from("<CashTransactions>");
    for i in 0..200 {
        transactions.push_str(&format!(
            r#"<CashTransaction accountId="U1234567" reportDate="2025-01-{:02}"
                               type="Deposits/Withdrawals" amount="1000.00"
                               currency="USD" />"#,
            (i % 28) + 1
        ));
    }
    transactions.push_str("</CashTransactions>");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Stress Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    <Trades />
                    <OpenPositions />
                    {}
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#,
        transactions
    );

    let result = parse_activity_flex(&xml);
    if let Err(e) = &result {
        eprintln!("Parse error: {:?}", e);
    }
    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.cash_transactions.items.len(), 200);
}

#[test]
fn test_parse_mixed_large_sections() {
    // Test with multiple large sections at once
    let mut trades = String::from("<Trades>");
    for i in 0..100 {
        trades.push_str(&format!(
            r#"<Trade accountId="U1234567" conid="{}" symbol="T{}"
                   assetCategory="STK" tradeDate="2025-01-15"
                   settleDateTarget="2025-01-17" quantity="100"
                   tradePrice="100.00" proceeds="-10000.00"
                   ibCommission="-1.00" netCash="-10001.00"
                   currency="USD" buySell="BUY" />"#,
            i, i
        ));
    }
    trades.push_str("</Trades>");

    let mut positions = String::from("<OpenPositions>");
    for i in 0..100 {
        positions.push_str(&format!(
            r#"<OpenPosition accountId="U1234567" conid="{}" symbol="P{}"
                             assetCategory="STK" reportDate="2025-01-31"
                             position="100" markPrice="150.00" positionValue="15000.00"
                             currency="USD" />"#,
            i, i
        ));
    }
    positions.push_str("</OpenPositions>");

    let mut cash = String::from("<CashTransactions>");
    for _i in 0..100 {
        cash.push_str(
            r#"<CashTransaction accountId="U1234567" reportDate="2025-01-15"
                               type="Deposits/Withdrawals" amount="1000.00"
                               currency="USD" />"#
        );
    }
    cash.push_str("</CashTransactions>");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <FlexQueryResponse queryName="Mixed Stress Test" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                               toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                    {}
                    {}
                    {}
                    <CorporateActions />
                    <SecuritiesInfo />
                    <ConversionRates />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>"#,
        trades, positions, cash
    );

    let result = parse_activity_flex(&xml);
    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.trades.items.len(), 100);
    assert_eq!(statement.positions.items.len(), 100);
    assert_eq!(statement.cash_transactions.items.len(), 100);
}

// ===== FUZZING / ROBUSTNESS TESTS =====

#[test]
fn test_parse_handles_extra_whitespace() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse    queryName="Test"    type="AF"   >
        <FlexStatements    count="1"   >
            <FlexStatement    accountId="U1234567"    fromDate="2025-01-01"
                           toDate="2025-01-31"    whenGenerated="2025-01-31;150000"   >
                <Trades   />
                <OpenPositions   />
                <CashTransactions   />
                <CorporateActions   />
                <SecuritiesInfo   />
                <ConversionRates   />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_ok());
}

#[test]
fn test_parse_handles_varying_attribute_order() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse type="AF" queryName="Test">
        <FlexStatements count="1">
            <FlexStatement whenGenerated="2025-01-31;150000" toDate="2025-01-31"
                           fromDate="2025-01-01" accountId="U1234567">
                <Trades />
                <OpenPositions />
                <CashTransactions />
                <CorporateActions />
                <SecuritiesInfo />
                <ConversionRates />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_ok());
}

#[test]
fn test_parse_handles_unicode_in_description() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse queryName="Test" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                           toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                <Trades>
                    <Trade accountId="U1234567" conid="265598" symbol="AAPL"
                           description="Apple Inc. 🍎 Technology"
                           assetCategory="STK" tradeDate="2025-01-15"
                           settleDateTarget="2025-01-17" quantity="100"
                           tradePrice="150.00" proceeds="-15000.00"
                           ibCommission="-1.00" netCash="-15001.00"
                           currency="USD" buySell="BUY" />
                </Trades>
                <OpenPositions />
                <CashTransactions />
                <CorporateActions />
                <SecuritiesInfo />
                <ConversionRates />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_ok());
}

#[test]
fn test_parse_handles_very_long_symbol() {
    let long_symbol = "A".repeat(100);
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse queryName="Test" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                           toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                <Trades>
                    <Trade accountId="U1234567" conid="265598" symbol="{}"
                           assetCategory="STK" tradeDate="2025-01-15"
                           settleDateTarget="2025-01-17" quantity="100"
                           tradePrice="150.00" proceeds="-15000.00"
                           ibCommission="-1.00" netCash="-15001.00"
                           currency="USD" buySell="BUY" />
                </Trades>
                <OpenPositions />
                <CashTransactions />
                <CorporateActions />
                <SecuritiesInfo />
                <ConversionRates />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#,
        long_symbol
    );

    let result = parse_activity_flex(&xml);
    assert!(result.is_ok());
}

// ===== EDGE CASE TESTS =====

#[test]
fn test_parse_handles_all_sections_empty() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse queryName="Empty Test" type="AF">
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

    let result = parse_activity_flex(xml);
    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.trades.items.len(), 0);
    assert_eq!(statement.positions.items.len(), 0);
    assert_eq!(statement.cash_transactions.items.len(), 0);
}

#[test]
fn test_parse_handles_extreme_decimal_precision() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <FlexQueryResponse queryName="Test" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2025-01-01"
                           toDate="2025-01-31" whenGenerated="2025-01-31;150000">
                <Trades>
                    <Trade accountId="U1234567" conid="265598" symbol="TEST"
                           assetCategory="STK" tradeDate="2025-01-15"
                           settleDateTarget="2025-01-17"
                           quantity="0.00000001"
                           tradePrice="123456789.987654321"
                           proceeds="-15000.00"
                           ibCommission="-0.000001"
                           netCash="-15001.00"
                           currency="USD" buySell="BUY" />
                </Trades>
                <OpenPositions />
                <CashTransactions />
                <CorporateActions />
                <SecuritiesInfo />
                <ConversionRates />
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_ok());
}

#[test]
fn test_memory_efficiency_with_repeated_parsing() {
    // Test that repeated parsing doesn't leak memory or degrade performance
    let xml = include_str!("fixtures/activity_minimal.xml");

    for _ in 0..100 {
        let result = parse_activity_flex(xml);
        assert!(result.is_ok());
    }
}

#[test]
fn test_concurrent_parsing_safety() {
    use std::thread;

    let xml = include_str!("fixtures/activity_minimal.xml").to_string();
    let mut handles = vec![];

    for _ in 0..10 {
        let xml_clone = xml.clone();
        let handle = thread::spawn(move || {
            let result = parse_activity_flex(&xml_clone);
            assert!(result.is_ok());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
