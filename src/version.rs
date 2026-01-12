//! FLEX schema version detection

use crate::error::{ParseError, Result};
use crate::StatementType;

/// FLEX schema versions supported by this library
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexSchemaVersion {
    /// FLEX schema version 3 (current)
    V3,
}

/// Detect FLEX schema version from XML
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query
///
/// # Returns
///
/// * `Ok(FlexSchemaVersion)` - Detected schema version
/// * `Err(ParseError)` - If version cannot be determined or is unsupported
///
/// # Errors
///
/// Returns `ParseError::UnsupportedSchemaVersion` if the schema version
/// is not supported by this library.
pub fn detect_version(_xml: &str) -> Result<FlexSchemaVersion> {
    // TODO: Parse version attribute from XML
    // For now, assume v3
    Ok(FlexSchemaVersion::V3)
}

/// Detect FLEX statement type from XML
///
/// Examines the XML structure to determine whether it's an Activity FLEX
/// or Trade Confirmation FLEX statement.
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query
///
/// # Returns
///
/// * `Ok(StatementType)` - Detected statement type
/// * `Err(ParseError)` - If type cannot be determined
pub fn detect_statement_type(_xml: &str) -> Result<StatementType> {
    // TODO: Implement actual detection logic
    // For now, return error
    Err(ParseError::XmlError {
        message: "Statement type detection not yet implemented".to_string(),
        location: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_version() {
        let xml = r#"<FlexQueryResponse></FlexQueryResponse>"#;
        let version = detect_version(xml);
        assert!(version.is_ok());
    }
}
