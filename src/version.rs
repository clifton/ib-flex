//! FLEX schema version detection

use crate::error::{ParseError, Result};
use crate::StatementType;

/// FLEX schema versions supported by this library
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexSchemaVersion {
    /// FLEX schema version 3 (current)
    V3,
    /// Unknown or unspecified version (treated as V3)
    Unknown,
}

/// Detect FLEX schema version from XML
///
/// Examines the XML to identify the FLEX schema version attribute.
/// If no version is specified or cannot be detected, defaults to V3.
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query
///
/// # Returns
///
/// * `Ok(FlexSchemaVersion)` - Detected schema version (or Unknown if undetectable)
///
/// # Example
///
/// ```rust
/// use ib_flex::version::detect_version;
///
/// let xml = r#"<FlexQueryResponse queryName="test" type="AF" version="3">"#;
/// let version = detect_version(xml).unwrap();
/// ```
pub fn detect_version(xml: &str) -> Result<FlexSchemaVersion> {
    // Look for version attribute in FlexQueryResponse or FlexStatement
    if let Some(pos) = xml.find("version=\"") {
        let version_start = pos + 9; // length of "version=\""
        if let Some(version_end) = xml[version_start..].find('"') {
            let version_str = &xml[version_start..version_start + version_end];
            return match version_str {
                "3" => Ok(FlexSchemaVersion::V3),
                _ => Ok(FlexSchemaVersion::Unknown),
            };
        }
    }

    // If no version attribute found, assume V3 (most common)
    Ok(FlexSchemaVersion::V3)
}

/// Detect FLEX statement type from XML
///
/// Examines the XML structure to determine whether it's an Activity FLEX
/// or Trade Confirmation FLEX statement by looking at the root element.
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query
///
/// # Returns
///
/// * `Ok(StatementType)` - Detected statement type
/// * `Err(ParseError)` - If type cannot be determined
///
/// # Example
///
/// ```rust
/// use ib_flex::{detect_statement_type, StatementType};
///
/// let xml = r#"<FlexQueryResponse><FlexStatements><FlexStatement ... /></FlexStatements></FlexQueryResponse>"#;
/// let stmt_type = detect_statement_type(xml).unwrap();
/// assert_eq!(stmt_type, StatementType::Activity);
/// ```
pub fn detect_statement_type(xml: &str) -> Result<StatementType> {
    // Remove XML declaration and whitespace for easier parsing
    let xml_trimmed = xml
        .trim_start()
        .trim_start_matches("<?xml")
        .trim_start()
        .trim_start_matches(|c: char| c != '<');

    // Check root element
    if xml_trimmed.starts_with("<FlexQueryResponse") {
        // Activity FLEX uses FlexQueryResponse wrapper
        Ok(StatementType::Activity)
    } else if xml_trimmed.starts_with("<TradeConfirmationStatement") {
        // Trade Confirmation uses TradeConfirmationStatement
        Ok(StatementType::TradeConfirmation)
    } else if xml_trimmed.starts_with("<FlexStatement") {
        // Direct FlexStatement is Activity FLEX
        Ok(StatementType::Activity)
    } else {
        Err(ParseError::XmlError {
            message: format!(
                "Cannot detect statement type from XML root element: {}",
                &xml_trimmed.chars().take(100).collect::<String>()
            ),
            location: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_version_v3() {
        let xml = r#"<FlexQueryResponse version="3" queryName="test">"#;
        let version = detect_version(xml).unwrap();
        assert_eq!(version, FlexSchemaVersion::V3);
    }

    #[test]
    fn test_detect_version_no_attribute() {
        let xml = r#"<FlexQueryResponse queryName="test">"#;
        let version = detect_version(xml).unwrap();
        // Should default to V3
        assert_eq!(version, FlexSchemaVersion::V3);
    }

    #[test]
    fn test_detect_version_unknown() {
        let xml = r#"<FlexQueryResponse version="4" queryName="test">"#;
        let version = detect_version(xml).unwrap();
        assert_eq!(version, FlexSchemaVersion::Unknown);
    }

    #[test]
    fn test_detect_statement_type_activity() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<FlexQueryResponse queryName="test" type="AF">
    <FlexStatements />
</FlexQueryResponse>"#;
        let stmt_type = detect_statement_type(xml).unwrap();
        assert_eq!(stmt_type, StatementType::Activity);
    }

    #[test]
    fn test_detect_statement_type_trade_confirmation() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TradeConfirmationStatement accountId="U123">
    <Trades />
</TradeConfirmationStatement>"#;
        let stmt_type = detect_statement_type(xml).unwrap();
        assert_eq!(stmt_type, StatementType::TradeConfirmation);
    }

    #[test]
    fn test_detect_statement_type_direct_flex_statement() {
        let xml = r#"<FlexStatement accountId="U123" fromDate="2025-01-01" toDate="2025-01-31">"#;
        let stmt_type = detect_statement_type(xml).unwrap();
        assert_eq!(stmt_type, StatementType::Activity);
    }

    #[test]
    fn test_detect_statement_type_invalid() {
        let xml = r#"<Invalid>XML</Invalid>"#;
        let result = detect_statement_type(xml);
        assert!(result.is_err());
    }
}
