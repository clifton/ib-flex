//! Trade Confirmation FLEX parser

use crate::error::{ParseError, Result};
use crate::types::TradeConfirmationStatement;
use quick_xml::de::from_str;

/// Parse a Trade Confirmation FLEX XML statement
///
/// Trade Confirmation FLEX statements contain real-time trade execution data.
/// They are similar to Activity FLEX but focused only on trade executions
/// and are updated immediately after each trade.
///
/// # Arguments
///
/// * `xml` - XML string from IB Trade Confirmation FLEX query
///
/// # Returns
///
/// * `Ok(TradeConfirmationStatement)` - Successfully parsed statement
/// * `Err(ParseError)` - Parse error with context
///
/// # Errors
///
/// Returns `ParseError` if XML is malformed, required fields are missing,
/// or date/decimal formats are invalid.
///
/// # Example
///
/// ```rust,no_run
/// use ib_flex::parse_trade_confirmation;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let xml = std::fs::read_to_string("trade_confirmation.xml")?;
/// let statement = parse_trade_confirmation(&xml)?;
///
/// println!("Account: {}", statement.account_id);
/// println!("Trades: {}", statement.trades.items.len());
/// # Ok(())
/// # }
/// ```
pub fn parse_trade_confirmation(xml: &str) -> Result<TradeConfirmationStatement> {
    // Parse the XML using serde and quick-xml
    from_str(xml).map_err(|e| ParseError::XmlError {
        message: format!("Failed to parse Trade Confirmation FLEX XML: {}", e),
        location: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_trade_confirmation() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TradeConfirmationStatement accountId="U1234567">
    <Trades>
        <Trade
            accountId="U1234567"
            symbol="AAPL"
            conid="265598"
            assetCategory="STK"
            tradeDate="2025-01-15"
            dateTime="2025-01-15;093015"
            settleDateTarget="2025-01-17"
            quantity="100"
            price="150.50"
            amount="-15050.00"
            proceeds="-15050.00"
            ibCommission="-1.00"
            ibCommissionCurrency="USD"
            netCash="-15051.00"
            currency="USD"
            fxRateToBase="1"
            multiplier="1"
        />
    </Trades>
</TradeConfirmationStatement>"#;

        let result = parse_trade_confirmation(xml);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let statement = result.unwrap();
        assert_eq!(statement.account_id, "U1234567");
        assert_eq!(statement.trades.items.len(), 1);
        assert_eq!(statement.trades.items[0].symbol, "AAPL");
    }

    #[test]
    fn test_parse_empty_trade_confirmation() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TradeConfirmationStatement accountId="U1234567">
    <Trades />
</TradeConfirmationStatement>"#;

        let result = parse_trade_confirmation(xml);
        assert!(result.is_ok());

        let statement = result.unwrap();
        assert_eq!(statement.account_id, "U1234567");
        assert_eq!(statement.trades.items.len(), 0);
    }

    #[test]
    fn test_parse_malformed_trade_confirmation() {
        let xml = r#"<Invalid>XML</Invalid>"#;
        let result = parse_trade_confirmation(xml);
        assert!(result.is_err());
    }
}
