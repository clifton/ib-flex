//! Common enums used across FLEX statements

use serde::{Deserialize, Serialize};

/// Asset category (security type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetCategory {
    /// Stock
    #[serde(rename = "STK")]
    Stock,

    /// Option
    #[serde(rename = "OPT")]
    Option,

    /// Future
    #[serde(rename = "FUT")]
    Future,

    /// Future Option
    #[serde(rename = "FOP")]
    FutureOption,

    /// Cash/Forex
    #[serde(rename = "CASH")]
    Cash,

    /// Bond
    #[serde(rename = "BOND")]
    Bond,

    /// CFD
    #[serde(rename = "CFD")]
    Cfd,

    /// Unknown or unrecognized asset category
    #[serde(other)]
    Unknown,
}

/// Buy or Sell side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BuySell {
    /// Buy
    #[serde(rename = "BUY")]
    Buy,

    /// Sell
    #[serde(rename = "SELL")]
    Sell,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Open or Close indicator (for options/futures)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum OpenClose {
    /// Opening trade
    #[serde(rename = "O")]
    Open,

    /// Closing trade
    #[serde(rename = "C")]
    Close,

    /// Close and open (same-day round trip)
    #[serde(rename = "C;O")]
    CloseOpen,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Market order
    #[serde(rename = "MKT")]
    Market,

    /// Limit order
    #[serde(rename = "LMT")]
    Limit,

    /// Stop order
    #[serde(rename = "STP")]
    Stop,

    /// Stop limit order
    #[serde(rename = "STP LMT")]
    StopLimit,

    /// Market on close
    #[serde(rename = "MOC")]
    MarketOnClose,

    /// Limit on close
    #[serde(rename = "LOC")]
    LimitOnClose,

    /// Market if touched
    #[serde(rename = "MIT")]
    MarketIfTouched,

    /// Limit if touched
    #[serde(rename = "LIT")]
    LimitIfTouched,

    /// Trailing stop
    #[serde(rename = "TRAIL")]
    TrailingStop,

    /// Unknown or unrecognized order type
    #[serde(other)]
    Unknown,
}

/// Put or Call (for options)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PutCall {
    /// Put option
    #[serde(rename = "P")]
    Put,

    /// Call option
    #[serde(rename = "C")]
    Call,

    /// Unknown
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_category_deserialize() {
        let json = r#""STK""#;
        let asset: AssetCategory = serde_json::from_str(json).unwrap();
        assert_eq!(asset, AssetCategory::Stock);
    }

    #[test]
    fn test_buy_sell_deserialize() {
        let json = r#""BUY""#;
        let side: BuySell = serde_json::from_str(json).unwrap();
        assert_eq!(side, BuySell::Buy);
    }
}
