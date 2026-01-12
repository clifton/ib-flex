//! Activity FLEX parser

use crate::error::{ParseError, Result};
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
pub fn parse_activity_flex(_xml: &str) -> Result<ActivityFlexStatement> {
    // TODO: Implement XML parsing with quick-xml and serde
    Err(ParseError::XmlError {
        message: "Activity FLEX parser not yet implemented".to_string(),
        location: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_activity_flex_not_implemented() {
        let xml = r#"<FlexQueryResponse></FlexQueryResponse>"#;
        let result = parse_activity_flex(xml);
        assert!(result.is_err());
    }
}
