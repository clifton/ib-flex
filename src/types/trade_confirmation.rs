//! Trade Confirmation FLEX statement types

use serde::{Deserialize, Serialize};

/// Trade Confirmation FLEX statement
///
/// Contains real-time trade execution data from a Trade Confirmation FLEX query.
/// This is refreshed immediately after each trade execution.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeConfirmationStatement {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Trade executions
    #[serde(rename = "Trades", default)]
    pub trades: super::activity::TradesWrapper,
}
