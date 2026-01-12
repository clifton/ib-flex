//! # ib-flex
//!
//! Pure Rust parser for Interactive Brokers FLEX XML statements.
//!
//! ## Features
//!
//! - 🚀 **Zero-copy parsing** with quick-xml and serde
//! - 💰 **Financial precision** with rust_decimal for all monetary values
//! - 📅 **Correct datetime handling** with chrono
//! - ✅ **Type-safe** enums for asset categories, order types, etc.
//! - 🔧 **No external dependencies** beyond XML/serde
//! - 📦 **Supports both Activity and Trade Confirmation FLEX**
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ib_flex::parse_activity_flex;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let xml = std::fs::read_to_string("flex_statement.xml")?;
//! let statement = parse_activity_flex(&xml)?;
//!
//! println!("Account: {}", statement.account_id);
//! println!("Total trades: {}", statement.trades.len());
//!
//! // Calculate total commissions
//! let total_commission: rust_decimal::Decimal =
//!     statement.trades.iter().map(|t| t.commission).sum();
//! println!("Total commissions: ${}", total_commission);
//! # Ok(())
//! # }
//! ```
//!
//! ## Supported FLEX Sections
//!
//! ### Activity FLEX
//! - ✅ Trades
//! - ✅ Open Positions
//! - ✅ Cash Transactions
//! - ✅ Corporate Actions
//! - ✅ Securities Info
//! - ✅ FX Conversion Rates
//!
//! ### Trade Confirmation FLEX
//! - ✅ Trade executions with all details
//! - ✅ Commission breakdown
//!
//! ## Known Limitations
//!
//! 1. **Date formats**: Only ISO-8601 (yyyy-MM-dd) and yyyyMMdd supported
//! 2. **Schema versions**: Tested with FLEX schema version 3

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod parsers;
pub mod types;
pub mod version;

// Re-export commonly used types
pub use error::{ParseError, Result};
pub use types::{
    AssetCategory, BuySell, OpenClose, OrderType, PutCall,
    ActivityFlexStatement, Trade, Position, CashTransaction, CorporateAction,
    TradeConfirmationStatement,
};

/// Parse an Activity FLEX XML statement
///
/// Parses Interactive Brokers Activity FLEX XML into a structured
/// Rust type with all trades, positions, cash flows, and other data.
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query (Activity type)
///
/// # Returns
///
/// * `Ok(ActivityFlexStatement)` - Successfully parsed statement
/// * `Err(ParseError)` - Parse error with detailed context
///
/// # Errors
///
/// Returns `ParseError` if:
/// - XML is malformed or invalid
/// - Required fields are missing
/// - Date/decimal formats are invalid
/// - FLEX schema version is unsupported
///
/// # Example
///
/// ```rust,no_run
/// use ib_flex::parse_activity_flex;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let xml = std::fs::read_to_string("statement.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// println!("Trades: {}", statement.trades.len());
/// # Ok(())
/// # }
/// ```
pub fn parse_activity_flex(xml: &str) -> Result<ActivityFlexStatement> {
    parsers::parse_activity_flex(xml)
}

/// Parse a Trade Confirmation FLEX XML statement
///
/// Parses Interactive Brokers Trade Confirmation FLEX XML into a structured
/// Rust type with real-time trade execution data.
///
/// # Arguments
///
/// * `xml` - XML string from IB FLEX query (Trade Confirmation type)
///
/// # Returns
///
/// * `Ok(TradeConfirmationStatement)` - Successfully parsed statement
/// * `Err(ParseError)` - Parse error with detailed context
///
/// # Errors
///
/// Returns `ParseError` if:
/// - XML is malformed or invalid
/// - Required fields are missing
/// - Date/decimal formats are invalid
///
/// # Example
///
/// ```rust,no_run
/// use ib_flex::parse_trade_confirmation;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let xml = std::fs::read_to_string("trade_conf.xml")?;
/// let statement = parse_trade_confirmation(&xml)?;
///
/// println!("Trade executions: {}", statement.trades.len());
/// # Ok(())
/// # }
/// ```
pub fn parse_trade_confirmation(xml: &str) -> Result<TradeConfirmationStatement> {
    parsers::parse_trade_confirmation(xml)
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
///
/// # Example
///
/// ```rust,no_run
/// use ib_flex::{detect_statement_type, StatementType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let xml = std::fs::read_to_string("statement.xml")?;
///
/// match detect_statement_type(&xml)? {
///     StatementType::Activity => println!("Activity FLEX"),
///     StatementType::TradeConfirmation => println!("Trade Confirmation"),
/// }
/// # Ok(())
/// # }
/// ```
pub fn detect_statement_type(xml: &str) -> Result<StatementType> {
    version::detect_statement_type(xml)
}

/// FLEX statement type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementType {
    /// Activity FLEX statement (daily EOD)
    Activity,
    /// Trade Confirmation FLEX statement (real-time)
    TradeConfirmation,
}
