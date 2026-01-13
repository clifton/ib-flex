//! Extended FLEX statement types for v0.2.0+
//!
//! This module contains additional types beyond the core v0.1.0 types.
//! These include account information, NAV changes, performance summaries,
//! fee details, and more comprehensive trading data.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{AssetCategory, OptionAction, TransferType};
use crate::parsers::xml_utils::{
    deserialize_flex_date, deserialize_optional_date, deserialize_optional_decimal,
};

/// Account information and metadata
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccountInformation {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account type
    #[serde(rename = "@accountType", default)]
    pub account_type: Option<String>,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// Account name
    #[serde(rename = "@name", default)]
    pub name: Option<String>,

    /// Master account
    #[serde(rename = "@masterName", default)]
    pub master_name: Option<String>,
}

/// Change in portfolio NAV (Net Asset Value)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeInNAV {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// From date
    #[serde(rename = "@fromDate", deserialize_with = "deserialize_flex_date")]
    pub from_date: NaiveDate,

    /// To date
    #[serde(rename = "@toDate", deserialize_with = "deserialize_flex_date")]
    pub to_date: NaiveDate,

    /// Starting value
    #[serde(rename = "@startingValue")]
    pub starting_value: Decimal,

    /// Transfers (deposits/withdrawals)
    #[serde(
        rename = "@transfers",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub transfers: Option<Decimal>,

    /// Mark-to-market P&L
    #[serde(
        rename = "@mtmPlusRealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtm_plus_realized_pnl: Option<Decimal>,

    /// Realized P&L
    #[serde(
        rename = "@realizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_pnl: Option<Decimal>,

    /// Unrealized P&L
    #[serde(
        rename = "@unrealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_pnl: Option<Decimal>,

    /// Ending value
    #[serde(rename = "@endingValue")]
    pub ending_value: Decimal,
}

/// Equity summary by report date in base currency
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EquitySummaryByReportDateInBase {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Report date
    #[serde(rename = "@reportDate", deserialize_with = "deserialize_flex_date")]
    pub report_date: NaiveDate,

    /// Cash
    #[serde(
        rename = "@cash",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cash: Option<Decimal>,

    /// Stock value
    #[serde(
        rename = "@stock",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub stock: Option<Decimal>,

    /// Options value
    #[serde(
        rename = "@options",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub options: Option<Decimal>,

    /// Bonds value
    #[serde(
        rename = "@bonds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub bonds: Option<Decimal>,

    /// Total
    #[serde(rename = "@total")]
    pub total: Decimal,
}

/// Cash report by currency
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CashReportCurrency {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Currency
    #[serde(rename = "@currency")]
    pub currency: String,

    /// From date
    #[serde(rename = "@fromDate", deserialize_with = "deserialize_flex_date")]
    pub from_date: NaiveDate,

    /// To date
    #[serde(rename = "@toDate", deserialize_with = "deserialize_flex_date")]
    pub to_date: NaiveDate,

    /// Starting cash
    #[serde(rename = "@startingCash")]
    pub starting_cash: Decimal,

    /// Commissions paid
    #[serde(
        rename = "@commissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions: Option<Decimal>,

    /// Deposits
    #[serde(
        rename = "@deposits",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub deposits: Option<Decimal>,

    /// Withdrawals
    #[serde(
        rename = "@withdrawals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub withdrawals: Option<Decimal>,

    /// Dividends
    #[serde(
        rename = "@dividends",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub dividends: Option<Decimal>,

    /// Ending cash
    #[serde(rename = "@endingCash")]
    pub ending_cash: Decimal,
}

/// Trade confirmation
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeConfirm {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Execution ID
    #[serde(rename = "@execID")]
    pub exec_id: String,

    /// Order ID
    #[serde(rename = "@orderID", default)]
    pub order_id: Option<String>,

    /// Trade date
    #[serde(rename = "@tradeDate", deserialize_with = "deserialize_flex_date")]
    pub trade_date: NaiveDate,

    /// Trade time
    #[serde(rename = "@tradeTime", default)]
    pub trade_time: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Asset category
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Price
    #[serde(rename = "@price")]
    pub price: Decimal,
}

/// Option exercise/assignment/expiration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OptionEAE {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Action type
    #[serde(rename = "@type")]
    pub action_type: OptionAction,

    /// Date
    #[serde(rename = "@date", deserialize_with = "deserialize_flex_date")]
    pub date: NaiveDate,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Strike
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,
}

/// Foreign exchange transaction
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FxTransaction {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Date/time
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// From currency
    #[serde(rename = "@fromCurrency")]
    pub from_currency: String,

    /// To currency
    #[serde(rename = "@toCurrency")]
    pub to_currency: String,

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Proceeds
    #[serde(rename = "@proceeds")]
    pub proceeds: Decimal,

    /// FX rate
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,
}

/// Change in dividend accruals
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeInDividendAccrual {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Ex date
    #[serde(rename = "@exDate", deserialize_with = "deserialize_flex_date")]
    pub ex_date: NaiveDate,

    /// Pay date
    #[serde(
        rename = "@payDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub pay_date: Option<NaiveDate>,

    /// Gross rate
    #[serde(rename = "@grossRate")]
    pub gross_rate: Decimal,

    /// Net amount
    #[serde(rename = "@netAmount")]
    pub net_amount: Decimal,
}

/// Open dividend accruals
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenDividendAccrual {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Ex date
    #[serde(rename = "@exDate", deserialize_with = "deserialize_flex_date")]
    pub ex_date: NaiveDate,

    /// Pay date
    #[serde(
        rename = "@payDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub pay_date: Option<NaiveDate>,

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Rate
    #[serde(rename = "@grossRate")]
    pub gross_rate: Decimal,
}

/// Interest accruals by currency
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InterestAccrualsCurrency {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Currency
    #[serde(rename = "@currency")]
    pub currency: String,

    /// From date
    #[serde(rename = "@fromDate", deserialize_with = "deserialize_flex_date")]
    pub from_date: NaiveDate,

    /// To date
    #[serde(rename = "@toDate", deserialize_with = "deserialize_flex_date")]
    pub to_date: NaiveDate,

    /// Starting accrual balance
    #[serde(rename = "@startingAccrualBalance")]
    pub starting_balance: Decimal,

    /// Interest accrued
    #[serde(rename = "@interestAccrued")]
    pub interest_accrued: Decimal,

    /// Ending accrual balance
    #[serde(rename = "@endingAccrualBalance")]
    pub ending_balance: Decimal,
}

/// Security transfer
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Transfer {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Transfer type
    #[serde(rename = "@type")]
    pub transfer_type: TransferType,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Direction
    #[serde(rename = "@direction", default)]
    pub direction: Option<String>,

    /// Date
    #[serde(rename = "@date", deserialize_with = "deserialize_flex_date")]
    pub date: NaiveDate,
}
