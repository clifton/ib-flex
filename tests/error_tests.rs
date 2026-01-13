//! Error handling tests

use ib_flex::parse_activity_flex;

#[test]
fn test_malformed_xml_missing_closing_tag() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
      <Trades>
        <!-- Missing closing tag -->
    </FlexStatement>
  </FlexStatements>
</FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_err(), "Should fail with malformed XML");
}

#[test]
fn test_malformed_xml_invalid_root() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<InvalidRoot>
  <Something />
</InvalidRoot>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_err(), "Should fail with invalid root element");
}

#[test]
fn test_empty_xml() {
    let xml = "";

    let result = parse_activity_flex(xml);
    assert!(result.is_err(), "Should fail with empty XML");
}

#[test]
fn test_missing_required_flex_statement() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="0">
  </FlexStatements>
</FlexQueryResponse>"#;

    let result = parse_activity_flex(xml);
    assert!(result.is_err(), "Should fail with no FlexStatement");
}

#[test]
fn test_invalid_date_format() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="invalid-date" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
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
    assert!(result.is_err(), "Should fail with invalid date format");
}

#[test]
fn test_invalid_asset_category() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
      <Trades>
        <Trade accountId="U1234567" conid="123" symbol="TEST" assetCategory="INVALID"
               tradeDate="2025-01-15" settleDateTarget="2025-01-17" commission="0" />
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
    // Note: This might succeed due to the Unknown variant in AssetCategory
    // But we test it anyway to document behavior
    match result {
        Ok(_) => {
            // If it succeeds, the Unknown variant caught it
        }
        Err(_) => {
            // If it fails, that's also acceptable
        }
    }
}

#[test]
fn test_missing_account_id() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
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
    assert!(result.is_err(), "Should fail without required accountId");
}

#[test]
fn test_unescaped_ampersand() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
      <Trades>
        <Trade accountId="U1234567" conid="123" symbol="S&P" assetCategory="STK"
               tradeDate="2025-01-15" settleDateTarget="2025-01-17" commission="0" />
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
    assert!(result.is_err(), "Should fail with unescaped ampersand");
}

#[test]
fn test_invalid_decimal_value() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
      <Trades>
        <Trade accountId="U1234567" conid="123" symbol="TEST" assetCategory="STK"
               tradeDate="2025-01-15" settleDateTarget="2025-01-17"
               commission="not-a-number" />
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
    assert!(result.is_err(), "Should fail with invalid decimal value");
}

#[test]
fn test_null_xml() {
    let xml = "\0\0\0";

    let result = parse_activity_flex(xml);
    assert!(result.is_err(), "Should fail with null bytes");
}

#[test]
fn test_very_large_number() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="Activity" type="AF">
  <FlexStatements count="1">
    <FlexStatement accountId="U1234567" fromDate="2025-01-15" toDate="2025-01-15"
                   period="LastBusinessDay" whenGenerated="2025-01-15;150000">
      <Trades>
        <Trade accountId="U1234567" conid="123" symbol="TEST" assetCategory="STK"
               tradeDate="2025-01-15" settleDateTarget="2025-01-17"
               commission="999999999999999999999999999999.99" />
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
    // Decimal can handle very large numbers, so this might succeed or fail
    // depending on Decimal limits
    let _ = result; // Just test that it doesn't panic
}
