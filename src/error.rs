//! Error types for FLEX parsing

use thiserror::Error;

/// Result type alias for FLEX parsing operations
pub type Result<T> = std::result::Result<T, ParseError>;

/// Errors that can occur during FLEX XML parsing
#[derive(Error, Debug)]
pub enum ParseError {
    /// XML deserialization error
    #[error("XML deserialization error: {message}")]
    XmlError {
        /// Error message from XML parser
        message: String,
        /// Optional location in XML where error occurred
        location: Option<String>,
    },

    /// Invalid date format
    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    /// Invalid decimal format
    #[error("Invalid decimal format: {0}")]
    InvalidDecimal(String),

    /// Missing required field
    #[error("Missing required field: {field} in {context}")]
    MissingField {
        /// Name of the missing field
        field: String,
        /// Context where field was expected
        context: String,
    },

    /// Unknown enum variant
    #[error("Unknown enum variant: {variant} for type {enum_type}")]
    UnknownEnumVariant {
        /// The unknown variant value
        variant: String,
        /// The enum type name
        enum_type: String,
    },

    /// Unsupported FLEX schema version
    #[error("Unsupported FLEX schema version: {0}")]
    UnsupportedSchemaVersion(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
