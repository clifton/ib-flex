//! Activity FLEX parser

use crate::error::{ParseError, Result};
use crate::types::activity::FlexQueryResponse;
use crate::types::ActivityFlexStatement;

/// Parse an Activity FLEX XML statement
///
/// # Arguments
///
/// * `xml` - XML string from IB Activity FLEX query
///
/// # Returns
///
/// * `Ok(ActivityFlexStatement)` - Successfully parsed statement
/// * `Err(ParseError)` - Parse error with context
///
/// # Errors
///
/// Returns `ParseError` if XML is malformed, required fields are missing,
/// or date/decimal formats are invalid.
pub fn parse_activity_flex(xml: &str) -> Result<ActivityFlexStatement> {
    // Parse XML using quick-xml with serde
    let response: FlexQueryResponse =
        quick_xml::de::from_str(xml).map_err(|e| ParseError::XmlError {
            message: format!("Failed to parse FLEX XML: {}", e),
            location: None,
        })?;

    // Extract the first statement
    let statement = response
        .statements
        .statements
        .into_iter()
        .next()
        .ok_or_else(|| ParseError::MissingField {
            field: "FlexStatement".to_string(),
            context: "FlexQueryResponse".to_string(),
        })?;

    Ok(statement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_activity() {
        let xml = include_str!("../../tests/fixtures/activity_minimal.xml");
        let result = parse_activity_flex(xml);

        match &result {
            Ok(statement) => {
                assert_eq!(statement.account_id, "U1234567");
                assert!(!statement.trades.items.is_empty());
                assert_eq!(statement.trades.items[0].symbol, "AAPL");
            }
            Err(e) => {
                panic!("Parse failed: {}", e);
            }
        }
    }
}
