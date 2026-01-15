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

    /// Change in NAV - single element (not wrapped like other sections)
    #[serde(rename = "ChangeInNAV", default)]
    pub change_in_nav: Option<super::extended::ChangeInNAV>,

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

    // v0.3.0+ sections - Performance and advanced features
    /// MTM performance summary by underlying
    #[serde(rename = "MTMPerformanceSummaryInBase", default)]
    pub mtm_performance_summary: MTMPerformanceSummaryWrapper,

    /// FIFO performance summary by underlying
    #[serde(rename = "FIFOPerformanceSummaryInBase", default)]
    pub fifo_performance_summary: FIFOPerformanceSummaryWrapper,

    /// MTD/YTD performance summary
    #[serde(rename = "MTDYTDPerformanceSummary", default)]
    pub mtd_ytd_performance_summary: MTDYTDPerformanceSummaryWrapper,

    /// Statement of funds (cash flow tracking)
    #[serde(rename = "StmtFunds", default)]
    pub statement_of_funds: StatementOfFundsWrapper,

    /// Change in position value (reconciliation)
    #[serde(rename = "ChangeInPositionValues", default)]
    pub change_in_position_values: ChangeInPositionValueWrapper,

    /// Unbundled commission details
    #[serde(rename = "UnbundledCommissionDetails", default)]
    pub unbundled_commission_details: UnbundledCommissionDetailWrapper,

    /// Client fees (advisory fees)
    #[serde(rename = "ClientFees", default)]
    pub client_fees: ClientFeesWrapper,

    /// Client fees detail
    #[serde(rename = "ClientFeesDetails", default)]
    pub client_fees_detail: ClientFeesDetailWrapper,

    /// Securities lending activities
    #[serde(rename = "SLBActivities", default)]
    pub slb_activities: SLBActivitiesWrapper,

    /// Securities lending fees
    #[serde(rename = "SLBFees", default)]
    pub slb_fees: SLBFeesWrapper,

    /// Hard to borrow details
    #[serde(rename = "HardToBorrowDetails", default)]
    pub hard_to_borrow_details: HardToBorrowDetailsWrapper,

    /// FX position lots
    #[serde(rename = "FxLots", default)]
    pub fx_lots: FxLotsWrapper,

    /// Unsettled transfers
    #[serde(rename = "UnsettledTransfers", default)]
    pub unsettled_transfers: UnsettledTransfersWrapper,

    /// Trade transfers (inter-broker)
    #[serde(rename = "TradeTransfers", default)]
    pub trade_transfers: TradeTransfersWrapper,

    /// Prior period positions
    #[serde(rename = "PriorPeriodPositions", default)]
    pub prior_period_positions: PriorPeriodPositionsWrapper,

    /// Tier interest details
    #[serde(rename = "TierInterestDetails", default)]
    pub tier_interest_details: TierInterestDetailsWrapper,

    /// Debit card activities
    #[serde(rename = "DebitCardActivities", default)]
    pub debit_card_activities: DebitCardActivitiesWrapper,

    /// Sales tax
    #[serde(rename = "SalesTaxes", default)]
    pub sales_tax: SalesTaxWrapper,

    // Note: SymbolSummary and AssetSummary elements appear INSIDE <Trades>,
    // not as separate sections. They're handled by TradesWrapper.
    // Orders also appear inside <Trades> as Order elements.
    // See TradesWrapper for how these are handled.

    // --- Catch-all fields for sections not yet fully implemented ---
    // These prevent parse errors when XML contains these sections
    #[serde(rename = "DepositsOnHold", default, skip_serializing)]
    deposits_on_hold: IgnoredSection,
    #[serde(rename = "FxPositions", default, skip_serializing)]
    fx_positions: IgnoredSection,
    #[serde(rename = "NetStockPositions", default, skip_serializing)]
    net_stock_positions: IgnoredSection,
    #[serde(rename = "ComplexPositions", default, skip_serializing)]
    complex_positions: IgnoredSection,
    #[serde(rename = "CFDCharges", default, skip_serializing)]
    cfd_charges: IgnoredSection,
    #[serde(rename = "CommissionCredits", default, skip_serializing)]
    commission_credits: IgnoredSection,
    #[serde(rename = "FdicInsuredDepositsByBank", default, skip_serializing)]
    fdic_insured_deposits: IgnoredSection,
    #[serde(rename = "HKIPOOpenSubscriptions", default, skip_serializing)]
    hk_ipo_open_subscriptions: IgnoredSection,
    #[serde(rename = "HKIPOSubscriptionActivity", default, skip_serializing)]
    hk_ipo_subscription_activity: IgnoredSection,
    #[serde(rename = "IBGNoteTransactions", default, skip_serializing)]
    ibg_note_transactions: IgnoredSection,
    #[serde(rename = "IncentiveCouponAccrualDetails", default, skip_serializing)]
    incentive_coupon_accruals: IgnoredSection,
    #[serde(rename = "MutualFundDividendDetails", default, skip_serializing)]
    mutual_fund_dividends: IgnoredSection,
    #[serde(rename = "NetStockPositionSummary", default, skip_serializing)]
    net_stock_position_summary: IgnoredSection,
    #[serde(rename = "PendingExcercises", default, skip_serializing)]
    pending_exercises: IgnoredSection,
    #[serde(rename = "RoutingCommissions", default, skip_serializing)]
    routing_commissions: IgnoredSection,
    #[serde(rename = "SLBCollaterals", default, skip_serializing)]
    slb_collaterals: IgnoredSection,
    #[serde(rename = "SLBOpenContracts", default, skip_serializing)]
    slb_open_contracts: IgnoredSection,
    #[serde(rename = "SoftDollars", default, skip_serializing)]
    soft_dollars: IgnoredSection,
    #[serde(rename = "StockGrantActivities", default, skip_serializing)]
    stock_grant_activities: IgnoredSection,
    #[serde(rename = "TransactionTaxes", default, skip_serializing)]
    transaction_taxes: IgnoredSection,
    #[serde(rename = "UnbookedTrades", default, skip_serializing)]
    unbooked_trades: IgnoredSection,
    // Note: Catch-all flatten disabled as it causes issues with multi-statement files
    // All unknown sections should be explicitly listed above with IgnoredSection
}

/// Helper type for sections we want to ignore during parsing
#[derive(Debug, Clone, PartialEq, Default)]
struct IgnoredSection;

impl<'de> serde::Deserialize<'de> for IgnoredSection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Ignore whatever content is in this section
        serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(IgnoredSection)
    }
}

/// Element types that can appear in the `<Trades>` section.
///
/// IB FLEX interleaves different element types by symbol, so we parse them all
/// into an enum and then filter by type for user access.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
enum TradesItem {
    Trade(Trade),
    Order(Trade),
    SymbolSummary(Trade),
    AssetSummary(Trade),
    WashSale(Trade),
    Lot(Trade),
}

/// Wrapper for trades section
///
/// The IB FLEX `<Trades>` section can contain multiple element types based on
/// the `levelOfDetail` attribute:
/// - `<Trade>` with levelOfDetail="EXECUTION" - individual trade executions
/// - `<Order>` with levelOfDetail="ORDER" - order summaries
/// - `<SymbolSummary>`, `<AssetSummary>`, `<WashSale>`, `<Lot>` - various summary records
///
/// These elements can be interleaved (grouped by symbol), not by type.
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct TradesWrapper {
    /// Trade executions (main trading data)
    pub items: Vec<Trade>,

    /// Wash sale records
    pub wash_sales: Vec<Trade>,
}

impl<'de> serde::Deserialize<'de> for TradesWrapper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "$value", default)]
            items: Vec<TradesItem>,
        }

        let raw = Raw::deserialize(deserializer)?;

        let mut trades = Vec::new();
        let mut wash_sales = Vec::new();

        for item in raw.items {
            match item {
                TradesItem::Trade(t) => trades.push(t),
                TradesItem::WashSale(t) => wash_sales.push(t),
                // Ignore Order, SymbolSummary, AssetSummary, Lot for items
                _ => {}
            }
        }

        Ok(TradesWrapper {
            items: trades,
            wash_sales,
        })
    }
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
/// Fields are organized into CORE (essential for tax/portfolio analytics)
/// and EXTENDED (metadata, execution details) sections.
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
    // ==================== CORE FIELDS ====================
    // Essential for tax reporting and portfolio analytics

    // --- Account ---
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID (unique identifier for idempotency)
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    // --- Security Identification ---
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

    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// FIGI
    #[serde(rename = "@figi", default)]
    pub figi: Option<String>,

    /// Security ID
    #[serde(rename = "@securityID", default)]
    pub security_id: Option<String>,

    /// Security ID type
    #[serde(rename = "@securityIDType", default)]
    pub security_id_type: Option<String>,

    // --- Derivatives (Options/Futures) ---
    /// Contract multiplier (for futures/options)
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

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

    /// Underlying security's contract ID (for derivatives)
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    // --- Trade Execution ---
    /// Trade date (may be empty for summary records)
    #[serde(
        rename = "@tradeDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub trade_date: Option<NaiveDate>,

    /// Settlement date (may be empty for summary records)
    #[serde(
        rename = "@settleDateTarget",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub settle_date: Option<NaiveDate>,

    /// Buy or Sell
    #[serde(rename = "@buySell", default)]
    pub buy_sell: Option<BuySell>,

    /// Open or Close indicator (for options/futures)
    #[serde(rename = "@openCloseIndicator", default)]
    pub open_close: Option<OpenClose>,

    /// Transaction type (ExchTrade, BookTrade, etc.)
    #[serde(rename = "@transactionType", default)]
    pub transaction_type: Option<String>,

    // --- Quantities and Prices ---
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

    /// Trade proceeds (negative for buys, positive for sells)
    #[serde(
        rename = "@proceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub proceeds: Option<Decimal>,

    /// Cost basis
    #[serde(
        rename = "@cost",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost: Option<Decimal>,

    // --- Fees and Taxes ---
    /// Commission paid
    #[serde(
        rename = "@ibCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commission: Option<Decimal>,

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

    // --- P&L ---
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

    // --- Currency ---
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

    // --- Tax Lot Tracking (Critical for tax reporting) ---
    /// Original trade date (for lot tracking and holding period)
    #[serde(
        rename = "@origTradeDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub orig_trade_date: Option<NaiveDate>,

    /// Original trade price (cost basis of the lot)
    #[serde(
        rename = "@origTradePrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub orig_trade_price: Option<Decimal>,

    /// Original trade ID (links closing trade to opening trade)
    #[serde(rename = "@origTradeID", default)]
    pub orig_trade_id: Option<String>,

    /// Holding period date/time (for long-term vs short-term determination)
    #[serde(rename = "@holdingPeriodDateTime", default)]
    pub holding_period_date_time: Option<String>,

    /// When position was opened
    #[serde(rename = "@openDateTime", default)]
    pub open_date_time: Option<String>,

    /// When position was reopened (for wash sale tracking)
    #[serde(rename = "@whenReopened", default)]
    pub when_reopened: Option<String>,

    /// Trade notes/codes (may contain wash sale indicator "W")
    #[serde(rename = "@notes", default)]
    pub notes: Option<String>,

    // ==================== EXTENDED FIELDS ====================
    // Metadata, execution details, and less commonly used fields

    // --- Order/Execution IDs ---
    /// IB order ID (may be shared across multiple executions)
    #[serde(rename = "@orderID", default)]
    pub ib_order_id: Option<String>,

    /// Execution ID
    #[serde(rename = "@execID", default)]
    pub exec_id: Option<String>,

    /// Trade ID
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    /// Original transaction ID
    #[serde(rename = "@origTransactionID", default)]
    pub orig_transaction_id: Option<String>,

    /// Original order ID
    #[serde(rename = "@origOrderID", default)]
    pub orig_order_id: Option<String>,

    // --- Timestamps ---
    /// Trade time (date + time)
    #[serde(rename = "@dateTime", default)]
    pub trade_time: Option<String>,

    /// When P&L was realized
    #[serde(rename = "@whenRealized", default)]
    pub when_realized: Option<String>,

    /// Order time
    #[serde(rename = "@orderTime", default)]
    pub order_time: Option<String>,

    // --- Order Details ---
    /// Order type (market, limit, stop, etc.)
    #[serde(rename = "@orderType", default)]
    pub order_type: Option<OrderType>,

    /// Brokerage order ID
    #[serde(rename = "@brokerageOrderID", default)]
    pub brokerage_order_id: Option<String>,

    /// Order reference
    #[serde(rename = "@orderReference", default)]
    pub order_reference: Option<String>,

    /// Exchange order ID
    #[serde(rename = "@exchOrderId", default)]
    pub exch_order_id: Option<String>,

    /// External execution ID
    #[serde(rename = "@extExecID", default)]
    pub ext_exec_id: Option<String>,

    /// IB execution ID
    #[serde(rename = "@ibExecID", default)]
    pub ib_exec_id: Option<String>,

    // --- Issuer/Security Metadata ---
    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Issuer country code
    #[serde(rename = "@issuerCountryCode", default)]
    pub issuer_country_code: Option<String>,

    /// Sub-category
    #[serde(rename = "@subCategory", default)]
    pub sub_category: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    // --- Underlying Extended ---
    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    // --- Execution Metadata ---
    /// Trader ID
    #[serde(rename = "@traderID", default)]
    pub trader_id: Option<String>,

    /// Is API order
    #[serde(rename = "@isAPIOrder", default)]
    pub is_api_order: Option<String>,

    /// Volatility order link
    #[serde(rename = "@volatilityOrderLink", default)]
    pub volatility_order_link: Option<String>,

    /// Clearing firm ID
    #[serde(rename = "@clearingFirmID", default)]
    pub clearing_firm_id: Option<String>,

    /// Level of detail (EXECUTION, ORDER, CLOSED_LOT, etc.)
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,

    // --- Price/Quantity Changes ---
    /// Trade amount
    #[serde(
        rename = "@amount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub amount: Option<Decimal>,

    /// Trade money (quantity * price)
    #[serde(
        rename = "@tradeMoney",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub trade_money: Option<Decimal>,

    /// Close price
    #[serde(
        rename = "@closePrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub close_price: Option<Decimal>,

    /// Change in price
    #[serde(
        rename = "@changeInPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub change_in_price: Option<Decimal>,

    /// Change in quantity
    #[serde(
        rename = "@changeInQuantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub change_in_quantity: Option<Decimal>,

    /// Commission currency
    #[serde(rename = "@ibCommissionCurrency", default)]
    pub commission_currency: Option<String>,

    // --- Related Trade Tracking ---
    /// Related trade ID
    #[serde(rename = "@relatedTradeID", default)]
    pub related_trade_id: Option<String>,

    /// Related transaction ID
    #[serde(rename = "@relatedTransactionID", default)]
    pub related_transaction_id: Option<String>,

    // --- Bond Fields ---
    /// Accrued interest
    #[serde(
        rename = "@accruedInt",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub accrued_int: Option<Decimal>,

    /// Principal adjust factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    // --- Commodity/Physical Delivery ---
    /// Serial number (for physical delivery)
    #[serde(rename = "@serialNumber", default)]
    pub serial_number: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Commodity type
    #[serde(rename = "@commodityType", default)]
    pub commodity_type: Option<String>,

    /// Fineness (for precious metals)
    #[serde(
        rename = "@fineness",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fineness: Option<Decimal>,

    /// Weight
    #[serde(rename = "@weight", default)]
    pub weight: Option<String>,

    // --- Other Metadata ---
    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Exchange where trade executed
    #[serde(rename = "@exchange", default)]
    pub exchange: Option<String>,

    /// Model (for model portfolios)
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// RTN
    #[serde(rename = "@rtn", default)]
    pub rtn: Option<String>,

    /// Position action ID
    #[serde(rename = "@positionActionID", default)]
    pub position_action_id: Option<String>,

    /// Initial investment
    #[serde(
        rename = "@initialInvestment",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub initial_investment: Option<Decimal>,
}

/// An open position snapshot
///
/// Represents a single open position at the end of the reporting period.
/// Fields are organized into CORE (essential for tax/portfolio analytics)
/// and EXTENDED (metadata) sections.
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
    // ==================== CORE FIELDS ====================
    // Essential for tax reporting and portfolio analytics

    // --- Account ---
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    // --- Security Identification ---
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

    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// FIGI
    #[serde(rename = "@figi", default)]
    pub figi: Option<String>,

    /// Security ID
    #[serde(rename = "@securityID", default)]
    pub security_id: Option<String>,

    /// Security ID type
    #[serde(rename = "@securityIDType", default)]
    pub security_id_type: Option<String>,

    // --- Derivatives (Options/Futures) ---
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

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    // --- Position and Value ---
    /// Position quantity (negative for short)
    #[serde(rename = "@position")]
    pub quantity: Decimal,

    /// Mark price (current market price)
    #[serde(rename = "@markPrice")]
    pub mark_price: Decimal,

    /// Position value (quantity * mark_price * multiplier)
    #[serde(rename = "@positionValue")]
    pub position_value: Decimal,

    /// Side (Long/Short)
    #[serde(rename = "@side", default)]
    pub side: Option<String>,

    // --- Cost Basis and P&L ---
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

    // --- Currency ---
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

    // --- Dates ---
    /// Date of this position snapshot
    #[serde(
        rename = "@reportDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
    pub report_date: NaiveDate,

    // --- Tax Lot Tracking (Critical for tax reporting) ---
    /// Holding period date/time (for long-term vs short-term determination)
    #[serde(rename = "@holdingPeriodDateTime", default)]
    pub holding_period_date_time: Option<String>,

    /// When position was opened
    #[serde(rename = "@openDateTime", default)]
    pub open_date_time: Option<String>,

    /// Originating transaction ID
    #[serde(rename = "@originatingTransactionID", default)]
    pub originating_transaction_id: Option<String>,

    /// Position code (may contain tax-related codes)
    #[serde(rename = "@code", default)]
    pub code: Option<String>,

    // ==================== EXTENDED FIELDS ====================
    // Metadata and less commonly used fields

    // --- Extended IDs ---
    /// Originating order ID (links to opening trade)
    #[serde(rename = "@originatingOrderID", default)]
    pub originating_order_id: Option<String>,

    // --- Issuer/Security Metadata ---
    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Issuer country code
    #[serde(rename = "@issuerCountryCode", default)]
    pub issuer_country_code: Option<String>,

    /// Sub-category
    #[serde(rename = "@subCategory", default)]
    pub sub_category: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    // --- Underlying Extended ---
    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    // --- Bond Fields ---
    /// Accrued interest
    #[serde(
        rename = "@accruedInt",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub accrued_int: Option<Decimal>,

    /// Principal adjust factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    // --- Commodity/Physical Delivery ---
    /// Serial number (for physical delivery)
    #[serde(rename = "@serialNumber", default)]
    pub serial_number: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Commodity type
    #[serde(rename = "@commodityType", default)]
    pub commodity_type: Option<String>,

    /// Fineness (for precious metals)
    #[serde(
        rename = "@fineness",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fineness: Option<Decimal>,

    /// Weight
    #[serde(rename = "@weight", default)]
    pub weight: Option<String>,

    // --- Other Metadata ---
    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,

    /// Model (for model portfolios)
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Vesting date (for restricted stock)
    #[serde(
        rename = "@vestingDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub vesting_date: Option<NaiveDate>,
}

/// A cash transaction (deposit, withdrawal, dividend, interest, fee)
///
/// Represents any cash flow that affects your account balance: deposits,
/// withdrawals, dividends, interest payments, withholding taxes, and fees.
/// Fields are organized into CORE and EXTENDED sections.
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
    // ==================== CORE FIELDS ====================
    // Essential for tax reporting and portfolio analytics

    // --- Account ---
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    // --- Transaction Details ---
    /// Transaction type (Deposits, Dividends, WithholdingTax, BrokerInterest, etc.)
    #[serde(rename = "@type", default)]
    pub transaction_type: Option<String>,

    /// Description of transaction
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

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

    // --- Dates ---
    /// Transaction date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Settlement date
    #[serde(
        rename = "@settleDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub settle_date: Option<NaiveDate>,

    /// Ex-dividend date (tax-critical for dividends)
    #[serde(
        rename = "@exDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub ex_date: Option<NaiveDate>,

    // --- Security Identification ---
    /// Related security's contract ID (for dividends)
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Related security's symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// FIGI
    #[serde(rename = "@figi", default)]
    pub figi: Option<String>,

    /// Security ID
    #[serde(rename = "@securityID", default)]
    pub security_id: Option<String>,

    /// Security ID type
    #[serde(rename = "@securityIDType", default)]
    pub security_id_type: Option<String>,

    // --- Derivatives ---
    /// Contract multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    /// Strike price
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry date
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put or Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<PutCall>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Transaction code (tax-relevant codes)
    #[serde(rename = "@code", default)]
    pub code: Option<String>,

    // ==================== EXTENDED FIELDS ====================
    // Metadata and less commonly used fields

    // --- Timestamps ---
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

    /// Available for trading date
    #[serde(
        rename = "@availableForTradingDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub available_for_trading_date: Option<NaiveDate>,

    // --- Extended IDs ---
    /// Action ID
    #[serde(rename = "@actionID", default)]
    pub action_id: Option<String>,

    /// Trade ID (for dividend/interest related to specific trade)
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    /// Client reference
    #[serde(rename = "@clientReference", default)]
    pub client_reference: Option<String>,

    // --- Issuer/Security Metadata ---
    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Issuer country code
    #[serde(rename = "@issuerCountryCode", default)]
    pub issuer_country_code: Option<String>,

    /// Sub-category
    #[serde(rename = "@subCategory", default)]
    pub sub_category: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    // --- Underlying Extended ---
    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    // --- Bond Fields ---
    /// Principal adjust factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    // --- Commodity/Physical Delivery ---
    /// Serial number
    #[serde(rename = "@serialNumber", default)]
    pub serial_number: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Commodity type
    #[serde(rename = "@commodityType", default)]
    pub commodity_type: Option<String>,

    /// Fineness
    #[serde(
        rename = "@fineness",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fineness: Option<Decimal>,

    /// Weight
    #[serde(rename = "@weight", default)]
    pub weight: Option<String>,

    // --- Other Metadata ---
    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,
}

/// A corporate action (split, merger, spinoff, etc.)
///
/// Represents corporate events that affect your holdings: stock splits,
/// reverse splits, mergers, spinoffs, tender offers, bond conversions, etc.
/// Fields are organized into CORE and EXTENDED sections.
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
    // ==================== CORE FIELDS ====================
    // Essential for tax reporting and portfolio analytics

    // --- Account ---
    /// IB account number
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// IB transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    // --- Action Details ---
    /// Action type (Split, Merger, Spinoff, etc.)
    #[serde(rename = "@type", default)]
    pub action_type: Option<String>,

    /// Description of corporate action
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    // --- Dates (Tax-critical) ---
    /// Action date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub action_date: Option<NaiveDate>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        deserialize_with = "crate::parsers::xml_utils::deserialize_flex_date"
    )]
    pub report_date: NaiveDate,

    /// Ex-date (ex-dividend date for dividends)
    #[serde(
        rename = "@exDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub ex_date: Option<NaiveDate>,

    /// Pay date
    #[serde(
        rename = "@payDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub pay_date: Option<NaiveDate>,

    /// Record date
    #[serde(
        rename = "@recordDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub record_date: Option<NaiveDate>,

    // --- Security Identification ---
    /// IB contract ID
    #[serde(rename = "@conid")]
    pub conid: String,

    /// Ticker symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// FIGI
    #[serde(rename = "@figi", default)]
    pub figi: Option<String>,

    /// Security ID
    #[serde(rename = "@securityID", default)]
    pub security_id: Option<String>,

    /// Security ID type
    #[serde(rename = "@securityIDType", default)]
    pub security_id_type: Option<String>,

    // --- Derivatives ---
    /// Contract multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

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

    /// Put or Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<PutCall>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    // --- Quantities and Values ---
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

    /// Cost
    #[serde(
        rename = "@cost",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost: Option<Decimal>,

    // --- P&L ---
    /// FIFO P&L realized
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

    // --- Currency ---
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

    /// Code (may contain tax-relevant info)
    #[serde(rename = "@code", default)]
    pub code: Option<String>,

    // ==================== EXTENDED FIELDS ====================
    // Metadata and less commonly used fields

    // --- Extended IDs ---
    /// Action ID
    #[serde(rename = "@actionID", default)]
    pub action_id: Option<String>,

    // --- Timestamps ---
    /// Action datetime
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    // --- Issuer/Security Metadata ---
    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Issuer country code
    #[serde(rename = "@issuerCountryCode", default)]
    pub issuer_country_code: Option<String>,

    /// Sub-category
    #[serde(rename = "@subCategory", default)]
    pub sub_category: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    // --- Underlying Extended ---
    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    // --- Bond Fields ---
    /// Accrued interest
    #[serde(
        rename = "@accruedInt",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub accrued_int: Option<Decimal>,

    /// Principal adjust factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    // --- Commodity/Physical Delivery ---
    /// Serial number
    #[serde(rename = "@serialNumber", default)]
    pub serial_number: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Commodity type
    #[serde(rename = "@commodityType", default)]
    pub commodity_type: Option<String>,

    /// Fineness (for precious metals)
    #[serde(
        rename = "@fineness",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fineness: Option<Decimal>,

    /// Weight
    #[serde(rename = "@weight", default)]
    pub weight: Option<String>,

    // --- Other Metadata ---
    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,

    /// Model (for model portfolios)
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,
}

/// Security information (reference data)
///
/// Provides detailed reference data for securities in the statement.
/// Includes identifiers (CUSIP, ISIN, FIGI), exchange info, and derivative details.
/// Fields are organized into CORE and EXTENDED sections.
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
    // ==================== CORE FIELDS ====================
    // Essential for tax reporting and portfolio analytics

    // --- Security Identification ---
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

    /// SEDOL
    #[serde(rename = "@sedol", default)]
    pub sedol: Option<String>,

    // --- Derivatives (Options/Futures) ---
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

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    // --- Bond/Fixed Income ---
    /// Maturity date (for bonds)
    #[serde(
        rename = "@maturity",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub maturity: Option<NaiveDate>,

    /// Principal adjustment factor
    #[serde(
        rename = "@principalAdjustFactor",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub principal_adjust_factor: Option<Decimal>,

    // --- Currency ---
    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    // ==================== EXTENDED FIELDS ====================
    // Metadata and less commonly used fields

    // --- Exchange Info ---
    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    /// Underlying security ID
    #[serde(rename = "@underlyingSecurityID", default)]
    pub underlying_security_id: Option<String>,

    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    // --- Issuer/Security Metadata ---
    /// Issuer
    #[serde(rename = "@issuer", default)]
    pub issuer: Option<String>,

    /// Issuer country code
    #[serde(rename = "@issuerCountryCode", default)]
    pub issuer_country_code: Option<String>,

    /// Sub-category
    #[serde(rename = "@subCategory", default)]
    pub sub_category: Option<String>,

    // --- Futures ---
    /// Delivery month (for futures)
    #[serde(rename = "@deliveryMonth", default)]
    pub delivery_month: Option<String>,

    // --- Commodity/Physical Delivery ---
    /// Serial number
    #[serde(rename = "@serialNumber", default)]
    pub serial_number: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Commodity type
    #[serde(rename = "@commodityType", default)]
    pub commodity_type: Option<String>,

    /// Fineness (for precious metals)
    #[serde(
        rename = "@fineness",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fineness: Option<Decimal>,

    /// Weight
    #[serde(rename = "@weight", default)]
    pub weight: Option<String>,

    // --- Other ---
    /// Code
    #[serde(rename = "@code", default)]
    pub code: Option<String>,
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

// v0.3.0+ wrappers for performance and advanced features

/// Wrapper for MTM performance summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MTMPerformanceSummaryWrapper {
    /// List of MTM performance summaries by underlying
    #[serde(rename = "MTMPerformanceSummaryUnderlying", default)]
    pub items: Vec<super::extended::MTMPerformanceSummaryUnderlying>,
}

/// Wrapper for FIFO performance summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FIFOPerformanceSummaryWrapper {
    /// List of FIFO performance summaries by underlying
    #[serde(rename = "FIFOPerformanceSummaryUnderlying", default)]
    pub items: Vec<super::extended::FIFOPerformanceSummaryUnderlying>,
}

/// Wrapper for MTD/YTD performance summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MTDYTDPerformanceSummaryWrapper {
    /// List of MTD/YTD performance summaries
    #[serde(rename = "MTDYTDPerformanceSummaryUnderlying", default)]
    pub items: Vec<super::extended::MTDYTDPerformanceSummary>,
}

/// Wrapper for statement of funds section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct StatementOfFundsWrapper {
    /// List of statement of funds lines
    #[serde(rename = "StatementOfFundsLine", default)]
    pub items: Vec<super::extended::StatementOfFundsLine>,
}

/// Wrapper for change in position value section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ChangeInPositionValueWrapper {
    /// List of position value changes
    #[serde(rename = "ChangeInPositionValue", default)]
    pub items: Vec<super::extended::ChangeInPositionValue>,
}

/// Wrapper for unbundled commission details section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnbundledCommissionDetailWrapper {
    /// List of unbundled commission details
    #[serde(rename = "UnbundledCommissionDetail", default)]
    pub items: Vec<super::extended::UnbundledCommissionDetail>,
}

/// Wrapper for client fees section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ClientFeesWrapper {
    /// List of client fees
    #[serde(rename = "ClientFee", default)]
    pub items: Vec<super::extended::ClientFee>,
}

/// Wrapper for client fees detail section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ClientFeesDetailWrapper {
    /// List of client fee details
    #[serde(rename = "ClientFeesDetail", default)]
    pub items: Vec<super::extended::ClientFeesDetail>,
}

/// Wrapper for SLB activities section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SLBActivitiesWrapper {
    /// List of SLB activities
    #[serde(rename = "SLBActivity", default)]
    pub items: Vec<super::extended::SLBActivity>,
}

/// Wrapper for SLB fees section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SLBFeesWrapper {
    /// List of SLB fees
    #[serde(rename = "SLBFee", default)]
    pub items: Vec<super::extended::SLBFee>,
}

/// Wrapper for hard to borrow details section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct HardToBorrowDetailsWrapper {
    /// List of hard to borrow details
    #[serde(rename = "HardToBorrowDetail", default)]
    pub items: Vec<super::extended::HardToBorrowDetail>,
}

/// Wrapper for FX lots section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FxLotsWrapper {
    /// List of FX lots
    #[serde(rename = "FxLot", default)]
    pub items: Vec<super::extended::FxLot>,
}

/// Wrapper for unsettled transfers section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnsettledTransfersWrapper {
    /// List of unsettled transfers
    #[serde(rename = "UnsettledTransfer", default)]
    pub items: Vec<super::extended::UnsettledTransfer>,
}

/// Wrapper for trade transfers section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TradeTransfersWrapper {
    /// List of trade transfers
    #[serde(rename = "TradeTransfer", default)]
    pub items: Vec<super::extended::TradeTransfer>,
}

/// Wrapper for prior period positions section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct PriorPeriodPositionsWrapper {
    /// List of prior period positions
    #[serde(rename = "PriorPeriodPosition", default)]
    pub items: Vec<super::extended::PriorPeriodPosition>,
}

/// Wrapper for tier interest details section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TierInterestDetailsWrapper {
    /// List of tier interest details
    #[serde(rename = "TierInterestDetail", default)]
    pub items: Vec<super::extended::TierInterestDetail>,
}

/// Wrapper for debit card activities section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct DebitCardActivitiesWrapper {
    /// List of debit card activities
    #[serde(rename = "DebitCardActivity", default)]
    pub items: Vec<super::extended::DebitCardActivity>,
}

/// Wrapper for sales tax section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SalesTaxWrapper {
    /// List of sales tax entries
    #[serde(rename = "SalesTax", default)]
    pub items: Vec<super::extended::SalesTax>,
}

/// Wrapper for symbol summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SymbolSummaryWrapper {
    /// List of symbol summaries
    #[serde(rename = "SymbolSummary", default)]
    pub items: Vec<super::extended::SymbolSummary>,
}

/// Wrapper for asset summary section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct AssetSummaryWrapper {
    /// List of asset summaries
    #[serde(rename = "AssetSummary", default)]
    pub items: Vec<super::extended::AssetSummary>,
}

/// Wrapper for orders section
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OrdersWrapper {
    /// List of orders
    #[serde(rename = "Order", default)]
    pub items: Vec<super::extended::Order>,
}
