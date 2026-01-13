//! Activity FLEX statement types

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{AssetCategory, BuySell, OpenClose, OrderType, PutCall};
use crate::parsers::xml_utils::{deserialize_optional_date, deserialize_optional_decimal};

/// Top-level FLEX query response
///
/// This is the root XML element in IB FLEX files. It wraps one or more
/// [`ActivityFlexStatement`]s along with query metadata.
///
/// **Note**: When using [`crate::parse_activity_flex`], this wrapper is handled
/// automatically and you receive the [`ActivityFlexStatement`] directly.
///
/// # Example
/// ```
/// use ib_flex::types::FlexQueryResponse;
/// use quick_xml::de::from_str;
///
/// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
/// <FlexQueryResponse queryName="Activity" type="AF">
///   <FlexStatements count="1">
///     <FlexStatement accountId="U1234567" fromDate="2025-01-01"
///                    toDate="2025-01-31" whenGenerated="2025-01-31;150000">
///       <Trades />
///       <OpenPositions />
///       <CashTransactions />
///       <CorporateActions />
///       <SecuritiesInfo />
///       <ConversionRates />
///     </FlexStatement>
///   </FlexStatements>
/// </FlexQueryResponse>"#;
///
/// let response: FlexQueryResponse = from_str(xml).unwrap();
/// assert_eq!(response.query_name, Some("Activity".to_string()));
/// assert_eq!(response.statements.statements.len(), 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
///
/// This is the main type returned by [`crate::parse_activity_flex`].
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use rust_decimal::Decimal;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// // Access account and date range
/// println!("Account: {}", statement.account_id);
/// println!("Period: {} to {}", statement.from_date, statement.to_date);
///
/// // Iterate through all trades
/// for trade in &statement.trades.items {
///     println!("{}: {} {} @ {}",
///         trade.symbol,
///         trade.buy_sell.as_ref().map(|b| format!("{:?}", b)).unwrap_or_default(),
///         trade.quantity.unwrap_or_default(),
///         trade.price.unwrap_or_default()
///     );
/// }
///
/// // Calculate total P&L
/// let total_pnl: Decimal = statement.trades.items.iter()
///     .filter_map(|t| t.fifo_pnl_realized)
///     .sum();
/// println!("Total realized P&L: {}", total_pnl);
///
/// // Access positions
/// for pos in &statement.positions.items {
///     println!("{}: {} shares @ {}",
///         pos.symbol,
///         pos.quantity,
///         pos.mark_price
///     );
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "FlexStatement")]
pub struct ActivityFlexStatement {
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Statement date range - start date
    #[serde(
        rename = "@fromDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
    pub from_date: NaiveDate,

    /// Statement date range - end date
    #[serde(
        rename = "@toDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
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

    // Extended v0.2.0+ sections
    /// Account information
    #[serde(rename = "AccountInformation", default)]
    pub account_information: Option<super::extended::AccountInformation>,

    /// Change in NAV
    #[serde(rename = "ChangeInNAV", default)]
    pub change_in_nav: ChangeInNAVWrapper,

    /// Equity summary by report date in base currency
    #[serde(rename = "EquitySummaryInBase", default)]
    pub equity_summary: EquitySummaryWrapper,

    /// Cash report by currency
    #[serde(rename = "CashReport", default)]
    pub cash_report: CashReportWrapper,

    /// Trade confirmations
    #[serde(rename = "TradeConfirms", default)]
    pub trade_confirms: TradeConfirmsWrapper,

    /// Option exercises, assignments, and expirations
    #[serde(rename = "OptionEAE", default)]
    pub option_eae: OptionEAEWrapper,

    /// Foreign exchange transactions
    #[serde(rename = "FxTransactions", default)]
    pub fx_transactions: FxTransactionsWrapper,

    /// Change in dividend accruals
    #[serde(rename = "ChangeInDividendAccruals", default)]
    pub change_in_dividend_accruals: ChangeInDividendAccrualsWrapper,

    /// Open dividend accruals
    #[serde(rename = "OpenDividendAccruals", default)]
    pub open_dividend_accruals: OpenDividendAccrualsWrapper,

    /// Interest accruals by currency
    #[serde(rename = "InterestAccruals", default)]
    pub interest_accruals: InterestAccrualsWrapper,

    /// Security transfers
    #[serde(rename = "Transfers", default)]
    pub transfers: TransfersWrapper,
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
///
/// Represents one trade execution from the Activity FLEX statement.
/// Includes all trade details: security info, quantities, prices, fees, and P&L.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use ib_flex::{AssetCategory, BuySell};
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// for trade in &statement.trades.items {
///     // Access basic trade info
///     println!("Symbol: {}", trade.symbol);
///     println!("Asset: {:?}", trade.asset_category);
///
///     // Check trade direction
///     match trade.buy_sell {
///         Some(BuySell::Buy) => println!("Bought"),
///         Some(BuySell::Sell) => println!("Sold"),
///         _ => {}
///     }
///
///     // Calculate total cost
///     let quantity = trade.quantity.unwrap_or_default();
///     let price = trade.price.unwrap_or_default();
///     let cost = quantity * price;
///     println!("Cost: {}", cost);
///
///     // Access P&L if available
///     if let Some(pnl) = trade.fifo_pnl_realized {
///         println!("Realized P&L: {}", pnl);
///     }
///
///     // Check for options
///     if trade.asset_category == AssetCategory::Option {
///         println!("Strike: {:?}", trade.strike);
///         println!("Expiry: {:?}", trade.expiry);
///         println!("Put/Call: {:?}", trade.put_call);
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    #[serde(
        rename = "@tradeDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
    pub trade_date: NaiveDate,

    /// Trade time (date + time) - parsed from dateTime field
    #[serde(rename = "@dateTime", default)]
    pub trade_time: Option<String>, // Will parse manually

    /// Settlement date
    #[serde(
        rename = "@settleDateTarget",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
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
    #[serde(
        rename = "@netCash",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_cash: Option<Decimal>,

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
///
/// Represents a single open position at the end of the reporting period.
/// Includes quantity, current market price, cost basis, and unrealized P&L.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use rust_decimal::Decimal;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// for position in &statement.positions.items {
///     println!("{}: {} shares", position.symbol, position.quantity);
///     println!("  Current price: {}", position.mark_price);
///     println!("  Position value: {}", position.position_value);
///
///     // Calculate gain/loss percentage
///     if let Some(cost_basis) = position.cost_basis_money {
///         let current_value = position.position_value;
///         let gain_pct = ((current_value - cost_basis) / cost_basis) * Decimal::from(100);
///         println!("  Gain: {:.2}%", gain_pct);
///     }
///
///     // Show unrealized P&L
///     if let Some(pnl) = position.fifo_pnl_unrealized {
///         println!("  Unrealized P&L: {}", pnl);
///     }
///
///     // Check if short position
///     if position.quantity < Decimal::ZERO {
///         println!("  SHORT POSITION");
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    #[serde(
        rename = "@openPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub open_price: Option<Decimal>,

    /// Cost basis price per share/contract
    #[serde(
        rename = "@costBasisPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost_basis_price: Option<Decimal>,

    /// Total cost basis
    #[serde(
        rename = "@costBasisMoney",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost_basis_money: Option<Decimal>,

    /// FIFO unrealized P&L
    #[serde(
        rename = "@fifoPnlUnrealized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fifo_pnl_unrealized: Option<Decimal>,

    /// Percent of NAV
    #[serde(
        rename = "@percentOfNAV",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
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
    #[serde(
        rename = "@reportDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
    pub report_date: NaiveDate,
}

/// A cash transaction (deposit, withdrawal, dividend, interest, fee)
///
/// Represents any cash flow that affects your account balance: deposits,
/// withdrawals, dividends, interest payments, withholding taxes, and fees.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use rust_decimal::Decimal;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// // Categorize cash flows
/// let mut dividends = Decimal::ZERO;
/// let mut interest = Decimal::ZERO;
/// let mut fees = Decimal::ZERO;
///
/// for cash_txn in &statement.cash_transactions.items {
///     match cash_txn.transaction_type.as_str() {
///         "Dividends" => {
///             dividends += cash_txn.amount;
///             println!("Dividend from {}: {}",
///                 cash_txn.symbol.as_ref().unwrap_or(&"N/A".to_string()),
///                 cash_txn.amount
///             );
///         }
///         "Broker Interest Paid" | "Broker Interest Received" => {
///             interest += cash_txn.amount;
///         }
///         "Other Fees" | "Commission Adjustments" => {
///             fees += cash_txn.amount;
///         }
///         _ => {
///             println!("{}: {}", cash_txn.transaction_type, cash_txn.amount);
///         }
///     }
/// }
///
/// println!("\nTotals:");
/// println!("  Dividends: {}", dividends);
/// println!("  Interest: {}", interest);
/// println!("  Fees: {}", fees);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Transaction datetime
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
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
///
/// Represents corporate events that affect your holdings: stock splits,
/// reverse splits, mergers, spinoffs, tender offers, bond conversions, etc.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// for action in &statement.corporate_actions.items {
///     println!("{}: {}", action.symbol, action.description);
///     println!("  Type: {}", action.action_type);
///
///     // Check action type
///     match action.action_type.as_str() {
///         "FS" => println!("  Forward stock split"),
///         "RS" => println!("  Reverse stock split"),
///         "SO" => println!("  Spinoff"),
///         "TO" => println!("  Tender offer"),
///         "TC" => println!("  Treasury bill/bond maturity"),
///         "BC" => println!("  Bond conversion"),
///         _ => println!("  Other: {}", action.action_type),
///     }
///
///     // Show quantities and proceeds
///     if let Some(qty) = action.quantity {
///         println!("  Quantity: {}", qty);
///     }
///     if let Some(proceeds) = action.proceeds {
///         println!("  Proceeds: {}", proceeds);
///     }
///
///     // Show realized P&L if applicable
///     if let Some(pnl) = action.fifo_pnl_realized {
///         println!("  Realized P&L: {}", pnl);
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub action_date: Option<NaiveDate>,

    /// Action datetime
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
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
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Quantity affected
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Amount
    #[serde(
        rename = "@amount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub amount: Option<Decimal>,

    /// Proceeds (if any)
    #[serde(
        rename = "@proceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub proceeds: Option<Decimal>,

    /// Value (if any)
    #[serde(
        rename = "@value",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
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
///
/// Provides detailed reference data for securities in the statement.
/// Includes identifiers (CUSIP, ISIN, FIGI), exchange info, and derivative details.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use ib_flex::AssetCategory;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// for security in &statement.securities_info.items {
///     println!("{} ({})", security.symbol, security.conid);
///
///     // Print description
///     if let Some(desc) = &security.description {
///         println!("  Description: {}", desc);
///     }
///
///     // Print identifiers
///     if let Some(cusip) = &security.cusip {
///         println!("  CUSIP: {}", cusip);
///     }
///     if let Some(isin) = &security.isin {
///         println!("  ISIN: {}", isin);
///     }
///
///     // Show derivative info for options
///     if security.asset_category == AssetCategory::Option {
///         println!("  Underlying: {:?}", security.underlying_symbol);
///         println!("  Strike: {:?}", security.strike);
///         println!("  Expiry: {:?}", security.expiry);
///         println!("  Type: {:?}", security.put_call);
///         println!("  Multiplier: {:?}", security.multiplier);
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
///
/// Provides daily FX conversion rates for multi-currency accounts.
/// Used to convert foreign currency amounts to your base currency.
///
/// # Example
/// ```no_run
/// use ib_flex::parse_activity_flex;
/// use rust_decimal::Decimal;
///
/// let xml = std::fs::read_to_string("activity.xml")?;
/// let statement = parse_activity_flex(&xml)?;
///
/// // Find conversion rate for a specific currency pair
/// let eur_to_usd = statement.conversion_rates.items
///     .iter()
///     .find(|r| r.from_currency == "EUR" && r.to_currency == "USD");
///
/// if let Some(rate) = eur_to_usd {
///     println!("EUR/USD rate on {}: {}", rate.report_date, rate.rate);
///
///     // Convert 1000 EUR to USD
///     let eur_amount = Decimal::from(1000);
///     let usd_amount = eur_amount * rate.rate;
///     println!("1000 EUR = {} USD", usd_amount);
/// }
///
/// // List all available rates
/// for rate in &statement.conversion_rates.items {
///     println!("{}/{}: {}", rate.from_currency, rate.to_currency, rate.rate);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConversionRate {
    /// Report date
    #[serde(
        rename = "@reportDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
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

// Extended v0.2.0+ wrappers
/// Wrapper for change in NAV section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ChangeInNAVWrapper {
    /// List of NAV changes
    #[serde(rename = "ChangeInNAV", default)]
    pub items: Vec<super::extended::ChangeInNAV>,
}

/// Wrapper for equity summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct EquitySummaryWrapper {
    /// List of equity summaries
    #[serde(rename = "EquitySummaryByReportDateInBase", default)]
    pub items: Vec<super::extended::EquitySummaryByReportDateInBase>,
}

/// Wrapper for cash report section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct CashReportWrapper {
    /// List of cash reports
    #[serde(rename = "CashReportCurrency", default)]
    pub items: Vec<super::extended::CashReportCurrency>,
}

/// Wrapper for trade confirmations section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TradeConfirmsWrapper {
    /// List of trade confirmations
    #[serde(rename = "TradeConfirm", default)]
    pub items: Vec<super::extended::TradeConfirm>,
}

/// Wrapper for option EAE section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OptionEAEWrapper {
    /// List of option exercises/assignments/expirations
    #[serde(rename = "OptionEAE", default)]
    pub items: Vec<super::extended::OptionEAE>,
}

/// Wrapper for FX transactions section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FxTransactionsWrapper {
    /// List of FX transactions
    #[serde(rename = "FxTransaction", default)]
    pub items: Vec<super::extended::FxTransaction>,
}

/// Wrapper for change in dividend accruals section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ChangeInDividendAccrualsWrapper {
    /// List of dividend accrual changes
    #[serde(rename = "ChangeInDividendAccrual", default)]
    pub items: Vec<super::extended::ChangeInDividendAccrual>,
}

/// Wrapper for open dividend accruals section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OpenDividendAccrualsWrapper {
    /// List of open dividend accruals
    #[serde(rename = "OpenDividendAccrual", default)]
    pub items: Vec<super::extended::OpenDividendAccrual>,
}

/// Wrapper for interest accruals section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct InterestAccrualsWrapper {
    /// List of interest accruals
    #[serde(rename = "InterestAccrualsCurrency", default)]
    pub items: Vec<super::extended::InterestAccrualsCurrency>,
}

/// Wrapper for transfers section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TransfersWrapper {
    /// List of transfers
    #[serde(rename = "Transfer", default)]
    pub items: Vec<super::extended::Transfer>,
}
