//! Activity FLEX statement types

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{AssetCategory, BuySell, OpenClose, OrderType, PutCall};

/// Top-level Activity FLEX statement
///
/// Contains all data from an Activity FLEX query including trades,
/// positions, cash transactions, and other portfolio data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ActivityFlexStatement {
    /// IB account number
    pub account_id: String,

    /// Statement date range - start date
    pub from_date: NaiveDate,

    /// Statement date range - end date
    pub to_date: NaiveDate,

    /// When the report was generated
    pub when_generated: NaiveDateTime,

    /// All trades in the period
    #[serde(default)]
    pub trades: Vec<Trade>,

    /// Open positions at end of period
    #[serde(default)]
    pub positions: Vec<Position>,

    /// Cash transactions (deposits, withdrawals, dividends, interest)
    #[serde(default)]
    pub cash_transactions: Vec<CashTransaction>,

    /// Corporate actions (splits, mergers, spinoffs)
    #[serde(default)]
    pub corporate_actions: Vec<CorporateAction>,
}

/// A single trade execution
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trade {
    // IB identifiers
    /// IB account number
    pub account_id: String,

    /// IB transaction ID (unique identifier for idempotency)
    pub transaction_id: i64,

    /// IB order ID (may be shared across multiple executions)
    pub ib_order_id: Option<i64>,

    /// Execution ID
    pub exec_id: String,

    // Security
    /// IB contract ID (unique per security)
    pub conid: i64,

    /// Ticker symbol
    pub symbol: String,

    /// Security description
    pub description: Option<String>,

    /// Asset category (stock, option, future, etc.)
    pub asset_category: AssetCategory,

    /// Contract multiplier (for futures/options)
    pub multiplier: Option<Decimal>,

    // Options/Futures
    /// Underlying security's contract ID (for derivatives)
    pub underlying_conid: Option<i64>,

    /// Strike price (for options)
    pub strike: Option<Decimal>,

    /// Expiry date (for options/futures)
    pub expiry: Option<NaiveDate>,

    /// Put or Call (for options)
    pub put_call: Option<PutCall>,

    // Trade details
    /// Trade date
    pub trade_date: NaiveDate,

    /// Trade time (date + time)
    pub trade_time: NaiveDateTime,

    /// Settlement date
    pub settle_date: NaiveDate,

    /// Buy or Sell
    pub buy_sell: BuySell,

    /// Open or Close indicator (for options/futures)
    pub open_close: Option<OpenClose>,

    /// Order type (market, limit, stop, etc.)
    pub order_type: Option<OrderType>,

    // Quantities and prices
    /// Quantity (number of shares/contracts)
    pub quantity: Decimal,

    /// Trade price per share/contract
    pub price: Decimal,

    /// Trade proceeds (negative for buys, positive for sells)
    pub proceeds: Decimal,

    /// Commission paid
    pub commission: Decimal,

    /// Taxes paid
    pub taxes: Decimal,

    /// Net cash (proceeds + commission + taxes)
    pub net_cash: Decimal,

    // P&L
    /// FIFO realized P&L (for closing trades)
    pub fifo_pnl_realized: Option<Decimal>,

    /// Mark-to-market P&L
    pub mtm_pnl: Option<Decimal>,

    /// FX P&L (for multi-currency)
    pub fx_pnl: Option<Decimal>,

    // Currency
    /// Trade currency
    pub currency: String,

    /// FX rate to base currency
    pub fx_rate_to_base: Option<Decimal>,

    // Exchange
    /// Exchange where trade was executed
    pub exchange: Option<String>,

    /// Trade value in local currency
    pub trade_money: Option<Decimal>,
}

/// An open position snapshot
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Position {
    /// IB account number
    pub account_id: String,

    /// IB contract ID
    pub conid: i64,

    /// Ticker symbol
    pub symbol: String,

    /// Asset category
    pub asset_category: AssetCategory,

    /// Position quantity (negative for short)
    pub quantity: Decimal,

    /// Mark price (current market price)
    pub mark_price: Decimal,

    /// Position value (quantity * mark_price * multiplier)
    pub position_value: Decimal,

    /// Cost basis price per share/contract
    pub cost_basis_price: Option<Decimal>,

    /// Total cost basis
    pub cost_basis_money: Option<Decimal>,

    /// FIFO unrealized P&L
    pub fifo_pnl_unrealized: Option<Decimal>,

    /// Currency
    pub currency: String,

    /// FX rate to base currency
    pub fx_rate_to_base: Option<Decimal>,

    /// Date of this position snapshot
    pub report_date: NaiveDate,
}

/// A cash transaction (deposit, withdrawal, dividend, interest, fee)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CashTransaction {
    /// IB account number
    pub account_id: String,

    /// IB transaction ID
    pub transaction_id: Option<i64>,

    /// Transaction type (Deposits, Dividends, WithholdingTax, BrokerInterest, etc.)
    pub transaction_type: String,

    /// Transaction date
    pub date_time: NaiveDate,

    /// Settlement date
    pub settle_date: Option<NaiveDate>,

    /// Amount (positive for credits, negative for debits)
    pub amount: Decimal,

    /// Currency
    pub currency: String,

    /// FX rate to base currency
    pub fx_rate_to_base: Option<Decimal>,

    /// Description of transaction
    pub description: Option<String>,

    /// Related security's contract ID (for dividends)
    pub conid: Option<i64>,

    /// Related security's symbol
    pub symbol: Option<String>,
}

/// A corporate action (split, merger, spinoff, etc.)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CorporateAction {
    /// IB account number
    pub account_id: String,

    /// IB transaction ID
    pub transaction_id: i64,

    /// Action type (Split, Merger, Spinoff, etc.)
    pub action_type: String,

    /// Action date
    pub action_date: NaiveDate,

    /// Report date
    pub report_date: NaiveDate,

    /// IB contract ID
    pub conid: i64,

    /// Ticker symbol
    pub symbol: String,

    /// Description of corporate action
    pub description: String,

    /// Quantity affected
    pub quantity: Option<Decimal>,

    /// Proceeds (if any)
    pub proceeds: Option<Decimal>,

    /// Value (if any)
    pub value: Option<Decimal>,
}
