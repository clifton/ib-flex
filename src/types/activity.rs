//! Activity FLEX statement types

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{AssetCategory, BuySell, OpenClose, OrderType, PutCall};
use crate::parsers::xml_utils::{deserialize_optional_date, deserialize_optional_decimal};

/// Top-level FLEX query response
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "FlexQueryResponse")]
pub struct FlexQueryResponse {
    /// Query name
    #[serde(rename = "@queryName", default)]
    pub query_name: Option<String>,

    /// Query type
    #[serde(rename = "@type", default)]
    pub query_type: Option<String>,

    /// FlexStatements wrapper
    #[serde(rename = "FlexStatements")]
    pub statements: FlexStatementsWrapper,
}

/// Wrapper for FlexStatements
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FlexStatementsWrapper {
    /// Count
    #[serde(rename = "@count", default)]
    pub count: Option<String>,

    /// Flex statement(s)
    #[serde(rename = "FlexStatement")]
    pub statements: Vec<ActivityFlexStatement>,
}

/// Top-level Activity FLEX statement
///
/// Contains all data from an Activity FLEX query including trades,
/// positions, cash transactions, and other portfolio data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "FlexStatement")]
pub struct ActivityFlexStatement {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Statement date range - start date
    #[serde(rename = "@fromDate")]
    pub from_date: NaiveDate,

    /// Statement date range - end date
    #[serde(rename = "@toDate")]
    pub to_date: NaiveDate,

    /// When the report was generated
    #[serde(rename = "@whenGenerated")]
    pub when_generated: String, // Parse separately due to IB format

    /// All trades in the period
    #[serde(rename = "Trades", default)]
    pub trades: TradesWrapper,

    /// Open positions at end of period
    #[serde(rename = "OpenPositions", default)]
    pub positions: PositionsWrapper,

    /// Cash transactions (deposits, withdrawals, dividends, interest)
    #[serde(rename = "CashTransactions", default)]
    pub cash_transactions: CashTransactionsWrapper,

    /// Corporate actions (splits, mergers, spinoffs)
    #[serde(rename = "CorporateActions", default)]
    pub corporate_actions: CorporateActionsWrapper,

    /// Securities information (reference data)
    #[serde(rename = "SecuritiesInfo", default)]
    pub securities_info: SecuritiesInfoWrapper,

    /// Currency conversion rates
    #[serde(rename = "ConversionRates", default)]
    pub conversion_rates: ConversionRatesWrapper,
}

/// Wrapper for trades section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TradesWrapper {
    /// List of trades
    #[serde(rename = "Trade", default)]
    pub items: Vec<Trade>,
}

/// Wrapper for positions section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct PositionsWrapper {
    /// List of positions
    #[serde(rename = "OpenPosition", default)]
    pub items: Vec<Position>,
}

/// Wrapper for cash transactions section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct CashTransactionsWrapper {
    /// List of cash transactions
    #[serde(rename = "CashTransaction", default)]
    pub items: Vec<CashTransaction>,
}

/// Wrapper for corporate actions section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct CorporateActionsWrapper {
    /// List of corporate actions
    #[serde(rename = "CorporateAction", default)]
    pub items: Vec<CorporateAction>,
}

/// A single trade execution
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trade {
    // IB identifiers
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID (unique identifier for idempotency)
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// IB order ID (may be shared across multiple executions)
    #[serde(rename = "@orderID", default)]
    pub ib_order_id: Option<String>,

    /// Execution ID
    #[serde(rename = "@execID", default)]
    pub exec_id: Option<String>,

    /// Trade ID
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    // Security
    /// IB contract ID (unique per security)
    #[serde(rename = "@conid")]
    pub conid: String,

    /// Ticker symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Security description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Asset category (stock, option, future, etc.)
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    /// Contract multiplier (for futures/options)
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    // Options/Futures
    /// Underlying security's contract ID (for derivatives)
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Strike price (for options)
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry date (for options/futures)
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put or Call (for options)
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<PutCall>,

    // Trade details
    /// Trade date
    #[serde(rename = "@tradeDate")]
    pub trade_date: NaiveDate,

    /// Trade time (date + time) - parsed from dateTime field
    #[serde(rename = "@dateTime", default)]
    pub trade_time: Option<String>, // Will parse manually

    /// Settlement date
    #[serde(rename = "@settleDateTarget")]
    pub settle_date: NaiveDate,

    /// Buy or Sell
    #[serde(rename = "@buySell", default)]
    pub buy_sell: Option<BuySell>,

    /// Open or Close indicator (for options/futures)
    #[serde(rename = "@openCloseIndicator", default)]
    pub open_close: Option<OpenClose>,

    /// Order type (market, limit, stop, etc.)
    #[serde(rename = "@orderType", default)]
    pub order_type: Option<OrderType>,

    // Quantities and prices
    /// Quantity (number of shares/contracts)
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Trade price per share/contract
    #[serde(
        rename = "@price",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub price: Option<Decimal>,

    /// Trade amount
    #[serde(
        rename = "@amount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub amount: Option<Decimal>,

    /// Trade proceeds (negative for buys, positive for sells)
    #[serde(rename = "@proceeds")]
    pub proceeds: Decimal,

    /// Commission paid
    #[serde(rename = "@ibCommission")]
    pub commission: Decimal,

    /// Commission currency
    #[serde(rename = "@ibCommissionCurrency", default)]
    pub commission_currency: Option<String>,

    /// Taxes paid
    #[serde(
        rename = "@taxes",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub taxes: Option<Decimal>,

    /// Net cash (proceeds + commission + taxes)
    #[serde(rename = "@netCash")]
    pub net_cash: Decimal,

    /// Cost
    #[serde(
        rename = "@cost",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost: Option<Decimal>,

    // P&L
    /// FIFO realized P&L (for closing trades)
    #[serde(
        rename = "@fifoPnlRealized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fifo_pnl_realized: Option<Decimal>,

    /// Mark-to-market P&L
    #[serde(
        rename = "@mtmPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtm_pnl: Option<Decimal>,

    /// FX P&L (for multi-currency)
    #[serde(
        rename = "@fxPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_pnl: Option<Decimal>,

    // Currency
    /// Trade currency
    #[serde(rename = "@currency")]
    pub currency: String,

    /// FX rate to base currency
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    // Additional fields
    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,
}

/// An open position snapshot
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Position {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB contract ID
    #[serde(rename = "@conid")]
    pub conid: String,

    /// Ticker symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Security description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    /// Contract multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    /// Strike (for options)
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry (for options/futures)
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put or Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<PutCall>,

    /// Position quantity (negative for short)
    #[serde(rename = "@position")]
    pub quantity: Decimal,

    /// Mark price (current market price)
    #[serde(rename = "@markPrice")]
    pub mark_price: Decimal,

    /// Position value (quantity * mark_price * multiplier)
    #[serde(rename = "@positionValue")]
    pub position_value: Decimal,

    /// Open price
    #[serde(rename = "@openPrice", default)]
    pub open_price: Option<Decimal>,

    /// Cost basis price per share/contract
    #[serde(rename = "@costBasisPrice", default)]
    pub cost_basis_price: Option<Decimal>,

    /// Total cost basis
    #[serde(rename = "@costBasisMoney", default)]
    pub cost_basis_money: Option<Decimal>,

    /// FIFO unrealized P&L
    #[serde(rename = "@fifoPnlUnrealized", default)]
    pub fifo_pnl_unrealized: Option<Decimal>,

    /// Percent of NAV
    #[serde(rename = "@percentOfNAV", default)]
    pub percent_of_nav: Option<Decimal>,

    /// Side (Long/Short)
    #[serde(rename = "@side", default)]
    pub side: Option<String>,

    /// Currency
    #[serde(rename = "@currency")]
    pub currency: String,

    /// FX rate to base currency
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Date of this position snapshot
    #[serde(rename = "@reportDate")]
    pub report_date: NaiveDate,
}

/// A cash transaction (deposit, withdrawal, dividend, interest, fee)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CashTransaction {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Transaction type (Deposits, Dividends, WithholdingTax, BrokerInterest, etc.)
    #[serde(rename = "@type")]
    pub transaction_type: String,

    /// Transaction date
    #[serde(rename = "@date", default)]
    pub date: Option<NaiveDate>,

    /// Transaction datetime
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Report date
    #[serde(rename = "@reportDate", default)]
    pub report_date: Option<NaiveDate>,

    /// Amount (positive for credits, negative for debits)
    #[serde(rename = "@amount")]
    pub amount: Decimal,

    /// Currency
    #[serde(rename = "@currency")]
    pub currency: String,

    /// FX rate to base currency
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Description of transaction
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Related security's contract ID (for dividends)
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Related security's symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,
}

/// A corporate action (split, merger, spinoff, etc.)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CorporateAction {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Action ID
    #[serde(rename = "@actionID", default)]
    pub action_id: Option<String>,

    /// Action type (Split, Merger, Spinoff, etc.)
    #[serde(rename = "@type")]
    pub action_type: String,

    /// Action date
    #[serde(rename = "@date", default)]
    pub action_date: Option<NaiveDate>,

    /// Action datetime
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Report date
    #[serde(rename = "@reportDate")]
    pub report_date: NaiveDate,

    /// IB contract ID
    #[serde(rename = "@conid")]
    pub conid: String,

    /// Ticker symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description of corporate action
    #[serde(rename = "@description")]
    pub description: String,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// FX rate to base
    #[serde(rename = "@fxRateToBase", default)]
    pub fx_rate_to_base: Option<Decimal>,

    /// Quantity affected
    #[serde(rename = "@quantity", default)]
    pub quantity: Option<Decimal>,

    /// Amount
    #[serde(rename = "@amount", default)]
    pub amount: Option<Decimal>,

    /// Proceeds (if any)
    #[serde(rename = "@proceeds", default)]
    pub proceeds: Option<Decimal>,

    /// Value (if any)
    #[serde(rename = "@value", default)]
    pub value: Option<Decimal>,

    /// FIFO P&L realized
    #[serde(
        rename = "@fifoPnlRealized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fifo_pnl_realized: Option<Decimal>,
}

/// Security information (reference data)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SecurityInfo {
    /// Asset category
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    /// Ticker symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Security description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// IB contract ID
    #[serde(rename = "@conid")]
    pub conid: String,

    /// Security ID
    #[serde(rename = "@securityID", default)]
    pub security_id: Option<String>,

    /// Security ID type
    #[serde(rename = "@securityIDType", default)]
    pub security_id_type: Option<String>,

    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// FIGI
    #[serde(rename = "@figi", default)]
    pub figi: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    /// Strike (for options)
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry (for options/futures)
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put or Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<PutCall>,

    /// Principal adjustment factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}

/// Foreign exchange conversion rate
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConversionRate {
    /// Report date
    #[serde(rename = "@reportDate")]
    pub report_date: NaiveDate,

    /// From currency (source)
    #[serde(rename = "@fromCurrency")]
    pub from_currency: String,

    /// To currency (target)
    #[serde(rename = "@toCurrency")]
    pub to_currency: String,

    /// Exchange rate
    #[serde(rename = "@rate")]
    pub rate: Decimal,
}

/// Wrapper for securities info section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SecuritiesInfoWrapper {
    /// List of securities
    #[serde(rename = "SecurityInfo", default)]
    pub items: Vec<SecurityInfo>,
}

/// Wrapper for conversion rates section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ConversionRatesWrapper {
    /// List of conversion rates
    #[serde(rename = "ConversionRate", default)]
    pub items: Vec<ConversionRate>,
}
