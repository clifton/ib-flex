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

    /// Customer type
    #[serde(rename = "@customerType", default)]
    pub customer_type: Option<String>,

    /// Date account opened
    #[serde(
        rename = "@dateOpened",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date_opened: Option<NaiveDate>,

    /// Date account funded
    #[serde(
        rename = "@dateFunded",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date_funded: Option<NaiveDate>,

    /// Date closed
    #[serde(
        rename = "@dateClosed",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date_closed: Option<NaiveDate>,

    /// Primary email
    #[serde(rename = "@primaryEmail", default)]
    pub primary_email: Option<String>,

    /// Street address
    #[serde(rename = "@streetAddress", default)]
    pub street_address: Option<String>,

    /// Street address 2
    #[serde(rename = "@streetAddress2", default)]
    pub street_address2: Option<String>,

    /// City
    #[serde(rename = "@city", default)]
    pub city: Option<String>,

    /// State
    #[serde(rename = "@state", default)]
    pub state: Option<String>,

    /// Country
    #[serde(rename = "@country", default)]
    pub country: Option<String>,

    /// Postal code
    #[serde(rename = "@postalCode", default)]
    pub postal_code: Option<String>,

    /// Account capabilities
    #[serde(rename = "@accountCapabilities", default)]
    pub account_capabilities: Option<String>,

    /// Trading permissions
    #[serde(rename = "@tradingPermissions", default)]
    pub trading_permissions: Option<String>,

    /// Registered representative name
    #[serde(rename = "@registeredRepName", default)]
    pub registered_rep_name: Option<String>,

    /// Registered representative phone
    #[serde(rename = "@registeredRepPhone", default)]
    pub registered_rep_phone: Option<String>,

    /// IB entity
    #[serde(rename = "@ibEntity", default)]
    pub ib_entity: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,
}

/// Change in portfolio NAV (Net Asset Value)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeInNAV {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// From date
    #[serde(rename = "@fromDate", deserialize_with = "deserialize_flex_date")]
    pub from_date: NaiveDate,

    /// To date
    #[serde(rename = "@toDate", deserialize_with = "deserialize_flex_date")]
    pub to_date: NaiveDate,

    /// Starting NAV value
    #[serde(rename = "@startingValue")]
    pub starting_value: Decimal,

    /// Ending NAV value
    #[serde(rename = "@endingValue")]
    pub ending_value: Decimal,

    /// Mark-to-market P&L
    #[serde(
        rename = "@mtm",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtm: Option<Decimal>,

    /// Realized P&L
    #[serde(
        rename = "@realized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized: Option<Decimal>,

    /// Change in unrealized P&L
    #[serde(
        rename = "@changeInUnrealized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub change_in_unrealized: Option<Decimal>,

    /// Deposits and withdrawals
    #[serde(
        rename = "@depositsWithdrawals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub deposits_withdrawals: Option<Decimal>,

    /// Dividends received
    #[serde(
        rename = "@dividends",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub dividends: Option<Decimal>,

    /// Withholding tax
    #[serde(
        rename = "@withholdingTax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub withholding_tax: Option<Decimal>,

    /// Change in dividend accruals
    #[serde(
        rename = "@changeInDividendAccruals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub change_in_dividend_accruals: Option<Decimal>,

    /// Interest income
    #[serde(
        rename = "@interest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub interest: Option<Decimal>,

    /// Change in interest accruals
    #[serde(
        rename = "@changeInInterestAccruals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub change_in_interest_accruals: Option<Decimal>,

    /// Advisor fees
    #[serde(
        rename = "@advisorFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub advisor_fees: Option<Decimal>,

    /// Client fees
    #[serde(
        rename = "@clientFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub client_fees: Option<Decimal>,

    /// Other fees
    #[serde(
        rename = "@otherFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub other_fees: Option<Decimal>,

    /// Commissions
    #[serde(
        rename = "@commissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions: Option<Decimal>,

    /// FX translation P&L
    #[serde(
        rename = "@fxTranslation",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_translation: Option<Decimal>,

    /// Time-weighted return
    #[serde(
        rename = "@twr",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub twr: Option<Decimal>,

    /// Corporate action proceeds
    #[serde(
        rename = "@corporateActionProceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub corporate_action_proceeds: Option<Decimal>,
}

/// Equity summary by report date in base currency
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EquitySummaryByReportDateInBase {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

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

    /// Cash long
    #[serde(
        rename = "@cashLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cash_long: Option<Decimal>,

    /// Cash short
    #[serde(
        rename = "@cashShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cash_short: Option<Decimal>,

    /// Settled cash
    #[serde(
        rename = "@settledCash",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub settled_cash: Option<Decimal>,

    /// Slb cash collateral
    #[serde(
        rename = "@slbCashCollateral",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub slb_cash_collateral: Option<Decimal>,

    /// Stock value
    #[serde(
        rename = "@stock",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub stock: Option<Decimal>,

    /// Stock long
    #[serde(
        rename = "@stockLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub stock_long: Option<Decimal>,

    /// Stock short
    #[serde(
        rename = "@stockShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub stock_short: Option<Decimal>,

    /// Slb direct securities borrowed
    #[serde(
        rename = "@slbDirectSecuritiesBorrowed",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub slb_direct_securities_borrowed: Option<Decimal>,

    /// Slb direct securities lent
    #[serde(
        rename = "@slbDirectSecuritiesLent",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub slb_direct_securities_lent: Option<Decimal>,

    /// Options value
    #[serde(
        rename = "@options",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub options: Option<Decimal>,

    /// Options long
    #[serde(
        rename = "@optionsLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub options_long: Option<Decimal>,

    /// Options short
    #[serde(
        rename = "@optionsShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub options_short: Option<Decimal>,

    /// Bonds value
    #[serde(
        rename = "@bonds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub bonds: Option<Decimal>,

    /// Bonds long
    #[serde(
        rename = "@bondsLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub bonds_long: Option<Decimal>,

    /// Bonds short
    #[serde(
        rename = "@bondsShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub bonds_short: Option<Decimal>,

    /// Notes value
    #[serde(
        rename = "@notes",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub notes: Option<Decimal>,

    /// Funds value
    #[serde(
        rename = "@funds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub funds: Option<Decimal>,

    /// Futures value (unrealized P&L)
    #[serde(
        rename = "@futures",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub futures: Option<Decimal>,

    /// Futures long
    #[serde(
        rename = "@futuresLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub futures_long: Option<Decimal>,

    /// Futures short
    #[serde(
        rename = "@futuresShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub futures_short: Option<Decimal>,

    /// Commodities value
    #[serde(
        rename = "@commodities",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commodities: Option<Decimal>,

    /// Total
    #[serde(
        rename = "@total",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total: Option<Decimal>,

    /// Total long
    #[serde(
        rename = "@totalLong",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_long: Option<Decimal>,

    /// Total short
    #[serde(
        rename = "@totalShort",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_short: Option<Decimal>,

    /// Interest accruals
    #[serde(
        rename = "@interestAccruals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub interest_accruals: Option<Decimal>,

    /// Dividend accruals
    #[serde(
        rename = "@dividendAccruals",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub dividend_accruals: Option<Decimal>,

    /// Accrued interest
    #[serde(
        rename = "@accruedInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub accrued_interest: Option<Decimal>,

    /// Accrued dividend
    #[serde(
        rename = "@accruedDividend",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub accrued_dividend: Option<Decimal>,

    /// Soft dollar value
    #[serde(
        rename = "@softDollars",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub soft_dollars: Option<Decimal>,

    /// Forex CFD unrealized P&L
    #[serde(
        rename = "@forexCfdUnrealizedPl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub forex_cfd_unrealized_pl: Option<Decimal>,

    /// CFD unrealized P&L
    #[serde(
        rename = "@cfdUnrealizedPl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cfd_unrealized_pl: Option<Decimal>,

    /// Broker cash component
    #[serde(
        rename = "@brokerCashComponent",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub broker_cash_component: Option<Decimal>,

    /// Broker interest accruals component
    #[serde(
        rename = "@brokerInterestAccrualsComponent",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub broker_interest_accruals_component: Option<Decimal>,

    /// Gross position value
    #[serde(
        rename = "@grossPositionValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub gross_position_value: Option<Decimal>,

    /// Net liquidation value
    #[serde(
        rename = "@netLiquidation",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_liquidation: Option<Decimal>,

    /// Net liquidation uncertainty
    #[serde(
        rename = "@netLiquidationUncertainty",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_liquidation_uncertainty: Option<Decimal>,
}

/// Cash report by currency
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CashReportCurrency {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

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

    /// Starting cash (securities segment)
    #[serde(
        rename = "@startingCashSec",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub starting_cash_sec: Option<Decimal>,

    /// Starting cash (commodities segment)
    #[serde(
        rename = "@startingCashCom",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub starting_cash_com: Option<Decimal>,

    /// Commissions paid
    #[serde(
        rename = "@commissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions: Option<Decimal>,

    /// Commissions (securities segment)
    #[serde(
        rename = "@commissionsSec",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions_sec: Option<Decimal>,

    /// Commissions (commodities segment)
    #[serde(
        rename = "@commissionsCom",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions_com: Option<Decimal>,

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

    /// Broker interest received
    #[serde(
        rename = "@brokerInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub broker_interest: Option<Decimal>,

    /// Bond interest received
    #[serde(
        rename = "@bondInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub bond_interest: Option<Decimal>,

    /// Withholding tax
    #[serde(
        rename = "@withholdingTax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub withholding_tax: Option<Decimal>,

    /// Net trades sales
    #[serde(
        rename = "@netTradesSales",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_trades_sales: Option<Decimal>,

    /// Net trades purchases
    #[serde(
        rename = "@netTradesPurchases",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_trades_purchases: Option<Decimal>,

    /// Account transfers
    #[serde(
        rename = "@accountTransfers",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub account_transfers: Option<Decimal>,

    /// Internal transfers
    #[serde(
        rename = "@internalTransfers",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub internal_transfers: Option<Decimal>,

    /// External transfers
    #[serde(
        rename = "@externalTransfers",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub external_transfers: Option<Decimal>,

    /// Link interest
    #[serde(
        rename = "@linkingAdjustments",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub linking_adjustments: Option<Decimal>,

    /// Other fees
    #[serde(
        rename = "@otherFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub other_fees: Option<Decimal>,

    /// FX translation P&L
    #[serde(
        rename = "@fxTranslationPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_translation_pnl: Option<Decimal>,

    /// Billable sales tax
    #[serde(
        rename = "@billableSalesTax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub billable_sales_tax: Option<Decimal>,

    /// Realized forex P&L
    #[serde(
        rename = "@realizedForexPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_forex_pnl: Option<Decimal>,

    /// Debit card activity
    #[serde(
        rename = "@debitCardActivity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub debit_card_activity: Option<Decimal>,

    /// Client fees
    #[serde(
        rename = "@clientFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub client_fees: Option<Decimal>,

    /// Cash settling MTM
    #[serde(
        rename = "@cashSettlingMtm",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cash_settling_mtm: Option<Decimal>,

    /// Soft dollar fees
    #[serde(
        rename = "@softDollars",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub soft_dollars: Option<Decimal>,

    /// Ending cash
    #[serde(rename = "@endingCash")]
    pub ending_cash: Decimal,

    /// Ending cash (securities segment)
    #[serde(
        rename = "@endingCashSec",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_cash_sec: Option<Decimal>,

    /// Ending cash (commodities segment)
    #[serde(
        rename = "@endingCashCom",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_cash_com: Option<Decimal>,

    /// Ending settled cash
    #[serde(
        rename = "@endingSettledCash",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_settled_cash: Option<Decimal>,

    /// Ending settled cash (securities segment)
    #[serde(
        rename = "@endingSettledCashSec",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_settled_cash_sec: Option<Decimal>,

    /// Ending settled cash (commodities segment)
    #[serde(
        rename = "@endingSettledCashCom",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_settled_cash_com: Option<Decimal>,
}

/// Trade confirmation
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeConfirm {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Execution ID
    #[serde(rename = "@execID")]
    pub exec_id: String,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Trade ID
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    /// Order ID
    #[serde(rename = "@orderID", default)]
    pub order_id: Option<String>,

    /// Trade date
    #[serde(rename = "@tradeDate", deserialize_with = "deserialize_flex_date")]
    pub trade_date: NaiveDate,

    /// Trade time
    #[serde(rename = "@tradeTime", default)]
    pub trade_time: Option<String>,

    /// Date time
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Settlement date
    #[serde(
        rename = "@settleDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub settle_date: Option<NaiveDate>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    // Security identifiers
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

    // Options fields
    /// Strike
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put/Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<String>,

    /// Multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    // Underlying
    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    // Trade details
    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Price
    #[serde(rename = "@price")]
    pub price: Decimal,

    /// Proceeds
    #[serde(
        rename = "@proceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub proceeds: Option<Decimal>,

    /// Commission
    #[serde(
        rename = "@commission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commission: Option<Decimal>,

    /// Taxes
    #[serde(
        rename = "@tax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub tax: Option<Decimal>,

    /// Net cash
    #[serde(
        rename = "@netCash",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_cash: Option<Decimal>,

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

    /// Buy/Sell
    #[serde(rename = "@buySell", default)]
    pub buy_sell: Option<String>,

    /// Order type
    #[serde(rename = "@orderType", default)]
    pub order_type: Option<String>,

    /// Exchange
    #[serde(rename = "@exchange", default)]
    pub exchange: Option<String>,

    /// Clearing ID
    #[serde(rename = "@clearingID", default)]
    pub clearing_id: Option<String>,

    /// Away broker commission
    #[serde(
        rename = "@awayBrokerCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub away_broker_commission: Option<Decimal>,

    /// Regulatory fee
    #[serde(
        rename = "@regulatoryFee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub regulatory_fee: Option<Decimal>,

    /// Order reference
    #[serde(rename = "@orderReference", default)]
    pub order_reference: Option<String>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

/// Option exercise/assignment/expiration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OptionEAE {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Action ID
    #[serde(rename = "@actionID", default)]
    pub action_id: Option<String>,

    /// Action type
    #[serde(rename = "@type", default)]
    pub action_type: Option<OptionAction>,

    /// Date
    #[serde(rename = "@date", deserialize_with = "deserialize_flex_date")]
    pub date: NaiveDate,

    /// Date time
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    // Security identifiers
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

    /// Expiry
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put/Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<String>,

    /// Multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Trade price
    #[serde(
        rename = "@tradePrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub trade_price: Option<Decimal>,

    /// Proceeds
    #[serde(
        rename = "@proceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub proceeds: Option<Decimal>,

    /// Commission
    #[serde(
        rename = "@commission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commission: Option<Decimal>,

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

    /// FIFO P&L realized
    #[serde(
        rename = "@fifoPnlRealized",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fifo_pnl_realized: Option<Decimal>,

    /// Notes/codes
    #[serde(rename = "@notes", default)]
    pub notes: Option<String>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

/// Foreign exchange transaction
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FxTransaction {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Action ID
    #[serde(rename = "@actionID", default)]
    pub action_id: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Date/time
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Functional currency
    #[serde(rename = "@functionalCurrency", default)]
    pub functional_currency: Option<String>,

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

    /// Cost
    #[serde(
        rename = "@cost",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost: Option<Decimal>,

    /// Realized P&L
    #[serde(
        rename = "@realizedPL",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_pl: Option<Decimal>,

    /// FX rate
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<String>,
}

/// Change in dividend accruals
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeInDividendAccrual {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

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

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

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

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Tax
    #[serde(
        rename = "@tax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub tax: Option<Decimal>,

    /// Fee
    #[serde(
        rename = "@fee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fee: Option<Decimal>,

    /// Gross rate
    #[serde(rename = "@grossRate")]
    pub gross_rate: Decimal,

    /// Gross amount
    #[serde(
        rename = "@grossAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub gross_amount: Option<Decimal>,

    /// Net amount
    #[serde(rename = "@netAmount")]
    pub net_amount: Decimal,

    /// From accrual (prior period)
    #[serde(
        rename = "@fromAccrual",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub from_accrual: Option<Decimal>,

    /// To accrual (current period)
    #[serde(
        rename = "@toAccrual",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub to_accrual: Option<Decimal>,

    /// Code
    #[serde(rename = "@code", default)]
    pub code: Option<String>,
}

/// Open dividend accruals
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenDividendAccrual {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

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

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

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

    /// Tax
    #[serde(
        rename = "@tax",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub tax: Option<Decimal>,

    /// Fee
    #[serde(
        rename = "@fee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fee: Option<Decimal>,

    /// Gross rate
    #[serde(rename = "@grossRate")]
    pub gross_rate: Decimal,

    /// Gross amount
    #[serde(
        rename = "@grossAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub gross_amount: Option<Decimal>,

    /// Net amount
    #[serde(
        rename = "@netAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_amount: Option<Decimal>,

    /// Code
    #[serde(rename = "@code", default)]
    pub code: Option<String>,
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

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Transaction ID
    #[serde(rename = "@transactionID", default)]
    pub transaction_id: Option<String>,

    /// Transfer type
    #[serde(rename = "@type", default)]
    pub transfer_type: Option<TransferType>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol")]
    pub symbol: String,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    // Security identifiers
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

    /// Quantity
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    /// Price
    #[serde(
        rename = "@transferPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub transfer_price: Option<Decimal>,

    /// Position amount
    #[serde(
        rename = "@positionAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub position_amount: Option<Decimal>,

    /// Position amount in base
    #[serde(
        rename = "@positionAmountInBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub position_amount_in_base: Option<Decimal>,

    /// Cash transfer
    #[serde(
        rename = "@cashTransfer",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cash_transfer: Option<Decimal>,

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

    /// Direction
    #[serde(rename = "@direction", default)]
    pub direction: Option<String>,

    /// Date
    #[serde(rename = "@date", deserialize_with = "deserialize_flex_date")]
    pub date: NaiveDate,

    /// Payer/payee account
    #[serde(rename = "@ppiPayerPayeeAccount", default)]
    pub ppi_payer_payee_account: Option<String>,

    /// Delivering/receiving broker
    #[serde(rename = "@deliveringReceivingBroker", default)]
    pub delivering_receiving_broker: Option<String>,

    // Options fields
    /// Strike
    #[serde(
        rename = "@strike",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub strike: Option<Decimal>,

    /// Expiry
    #[serde(
        rename = "@expiry",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expiry: Option<NaiveDate>,

    /// Put/Call
    #[serde(rename = "@putCall", default)]
    pub put_call: Option<String>,

    /// Multiplier
    #[serde(
        rename = "@multiplier",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub multiplier: Option<Decimal>,
}

// =============================================================================
// Performance Summary Types
// =============================================================================

/// Mark-to-market performance summary by underlying security
///
/// Provides MTM performance metrics for each security in the portfolio,
/// including realized and unrealized P&L, commissions, and transaction MTM.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MTMPerformanceSummaryUnderlying {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    // Security identifiers
    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    /// Underlying listing exchange
    #[serde(rename = "@underlyingListingExchange", default)]
    pub underlying_listing_exchange: Option<String>,

    // P&L fields
    /// Cost adjustment
    #[serde(
        rename = "@costAdj",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost_adj: Option<Decimal>,

    /// Realized short-term profit
    #[serde(
        rename = "@realizedSTProfit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_st_profit: Option<Decimal>,

    /// Realized short-term loss
    #[serde(
        rename = "@realizedSTLoss",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_st_loss: Option<Decimal>,

    /// Realized long-term profit
    #[serde(
        rename = "@realizedLTProfit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_lt_profit: Option<Decimal>,

    /// Realized long-term loss
    #[serde(
        rename = "@realizedLTLoss",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_lt_loss: Option<Decimal>,

    /// Unrealized short-term profit
    #[serde(
        rename = "@unrealizedSTProfit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_st_profit: Option<Decimal>,

    /// Unrealized short-term loss
    #[serde(
        rename = "@unrealizedSTLoss",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_st_loss: Option<Decimal>,

    /// Unrealized long-term profit
    #[serde(
        rename = "@unrealizedLTProfit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_lt_profit: Option<Decimal>,

    /// Unrealized long-term loss
    #[serde(
        rename = "@unrealizedLTLoss",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_lt_loss: Option<Decimal>,

    /// Transaction MTM
    #[serde(
        rename = "@transactionMtm",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub transaction_mtm: Option<Decimal>,

    /// Commissions
    #[serde(
        rename = "@commissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commissions: Option<Decimal>,

    /// Other fees
    #[serde(
        rename = "@other",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub other: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

/// FIFO performance summary by underlying security
///
/// Provides FIFO-based performance metrics including realized and unrealized
/// P&L calculated using FIFO cost basis method.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FIFOPerformanceSummaryUnderlying {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    // Security identifiers
    /// CUSIP
    #[serde(rename = "@cusip", default)]
    pub cusip: Option<String>,

    /// ISIN
    #[serde(rename = "@isin", default)]
    pub isin: Option<String>,

    /// Listing exchange
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,

    /// Underlying symbol
    #[serde(rename = "@underlyingSymbol", default)]
    pub underlying_symbol: Option<String>,

    /// Underlying contract ID
    #[serde(rename = "@underlyingConid", default)]
    pub underlying_conid: Option<String>,

    // P&L fields
    /// Realized short-term P&L
    #[serde(
        rename = "@realizedShortTermPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_short_term_pnl: Option<Decimal>,

    /// Realized long-term P&L
    #[serde(
        rename = "@realizedLongTermPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_long_term_pnl: Option<Decimal>,

    /// Realized total P&L
    #[serde(
        rename = "@realizedTotalPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_total_pnl: Option<Decimal>,

    /// Unrealized short-term P&L
    #[serde(
        rename = "@unrealizedShortTermPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_short_term_pnl: Option<Decimal>,

    /// Unrealized long-term P&L
    #[serde(
        rename = "@unrealizedLongTermPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_long_term_pnl: Option<Decimal>,

    /// Unrealized total P&L
    #[serde(
        rename = "@unrealizedTotalPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_total_pnl: Option<Decimal>,

    /// Total income
    #[serde(
        rename = "@totalIncome",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_income: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

/// Month-to-date/Year-to-date performance summary
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MTDYTDPerformanceSummary {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    // MTD fields
    /// MTD realized P&L
    #[serde(
        rename = "@mtdRealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtd_realized_pnl: Option<Decimal>,

    /// MTD unrealized P&L
    #[serde(
        rename = "@mtdUnrealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtd_unrealized_pnl: Option<Decimal>,

    /// MTD commissions
    #[serde(
        rename = "@mtdCommissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtd_commissions: Option<Decimal>,

    /// MTD fees
    #[serde(
        rename = "@mtdFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtd_fees: Option<Decimal>,

    // YTD fields
    /// YTD realized P&L
    #[serde(
        rename = "@ytdRealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ytd_realized_pnl: Option<Decimal>,

    /// YTD unrealized P&L
    #[serde(
        rename = "@ytdUnrealizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ytd_unrealized_pnl: Option<Decimal>,

    /// YTD commissions
    #[serde(
        rename = "@ytdCommissions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ytd_commissions: Option<Decimal>,

    /// YTD fees
    #[serde(
        rename = "@ytdFees",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ytd_fees: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

// =============================================================================
// Statement of Funds Types
// =============================================================================

/// Statement of funds line item
///
/// Represents a single cash flow entry in the statement of funds,
/// tracking debits and credits with running balance.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StatementOfFundsLine {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// Activity code
    #[serde(rename = "@activityCode", default)]
    pub activity_code: Option<String>,

    /// Activity description
    #[serde(rename = "@activityDescription", default)]
    pub activity_description: Option<String>,

    /// Trade ID
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Debit amount
    #[serde(
        rename = "@debit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub debit: Option<Decimal>,

    /// Credit amount
    #[serde(
        rename = "@credit",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub credit: Option<Decimal>,

    /// Amount
    #[serde(
        rename = "@amount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub amount: Option<Decimal>,

    /// Balance
    #[serde(
        rename = "@balance",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub balance: Option<Decimal>,

    /// FX rate to base
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

// =============================================================================
// Position Value Change Types
// =============================================================================

/// Change in position value for reconciliation
///
/// Tracks how position values changed due to various factors like
/// transactions, MTM changes, corporate actions, and FX translation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChangeInPositionValue {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// Prior period value
    #[serde(
        rename = "@priorPeriodValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub prior_period_value: Option<Decimal>,

    /// Transactions
    #[serde(
        rename = "@transactions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub transactions: Option<Decimal>,

    /// MTM prior period positions
    #[serde(
        rename = "@mtmPriorPeriodPositions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtm_prior_period_positions: Option<Decimal>,

    /// MTM transactions
    #[serde(
        rename = "@mtmTransactions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub mtm_transactions: Option<Decimal>,

    /// Corporate actions
    #[serde(
        rename = "@corporateActions",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub corporate_actions: Option<Decimal>,

    /// FX translation
    #[serde(
        rename = "@fxTranslation",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_translation: Option<Decimal>,

    /// Other
    #[serde(
        rename = "@other",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub other: Option<Decimal>,

    /// Ending value
    #[serde(
        rename = "@endingValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ending_value: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

// =============================================================================
// Fee Detail Types
// =============================================================================

/// Unbundled commission detail
///
/// Breaks down commission charges into components: execution, clearing,
/// regulatory, third-party, and exchange fees.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnbundledCommissionDetail {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Execution ID
    #[serde(rename = "@execID", default)]
    pub exec_id: Option<String>,

    /// Order ID
    #[serde(rename = "@orderID", default)]
    pub order_id: Option<String>,

    /// Trade ID
    #[serde(rename = "@tradeID", default)]
    pub trade_id: Option<String>,

    /// Trade date/time (format: YYYYMMDD;HHMMSS)
    #[serde(rename = "@dateTime", default)]
    pub date_time: Option<String>,

    /// Exchange
    #[serde(rename = "@exchange", default)]
    pub exchange: Option<String>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Price
    #[serde(
        rename = "@price",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub price: Option<Decimal>,

    /// Execution commission
    #[serde(
        rename = "@executionCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub execution_commission: Option<Decimal>,

    /// Clearing commission
    #[serde(
        rename = "@clearingCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub clearing_commission: Option<Decimal>,

    /// Regulatory commission
    #[serde(
        rename = "@regulatoryCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub regulatory_commission: Option<Decimal>,

    /// Third party commission
    #[serde(
        rename = "@thirdPartyCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub third_party_commission: Option<Decimal>,

    /// Third party regulatory commission
    #[serde(
        rename = "@thirdPartyRegulatoryCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub third_party_regulatory_commission: Option<Decimal>,

    /// Total commission
    #[serde(
        rename = "@totalCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_commission: Option<Decimal>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}

/// Client fee (advisory/service fees)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientFee {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// Revenue
    #[serde(
        rename = "@revenue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub revenue: Option<Decimal>,

    /// Expense
    #[serde(
        rename = "@expense",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub expense: Option<Decimal>,

    /// Net
    #[serde(
        rename = "@net",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net: Option<Decimal>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,
}

/// Client fee detail
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientFeesDetail {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// Fee type
    #[serde(rename = "@feeType", default)]
    pub fee_type: Option<String>,

    /// Revenue
    #[serde(
        rename = "@revenue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub revenue: Option<Decimal>,

    /// Expense
    #[serde(
        rename = "@expense",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub expense: Option<Decimal>,

    /// Net
    #[serde(
        rename = "@net",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net: Option<Decimal>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// FX rate to base
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,
}

// =============================================================================
// Securities Lending Types
// =============================================================================

/// Securities lending/borrowing activity
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SLBActivity {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Activity date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Activity type
    #[serde(rename = "@type", default)]
    pub activity_type: Option<String>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Collateral amount
    #[serde(
        rename = "@collateralAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub collateral_amount: Option<Decimal>,

    /// Fee/rate
    #[serde(
        rename = "@feeRate",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fee_rate: Option<Decimal>,

    /// Net lend fee
    #[serde(
        rename = "@netLendFee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub net_lend_fee: Option<Decimal>,

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
}

/// Securities lending fee
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SLBFee {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Value date
    #[serde(
        rename = "@valueDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub value_date: Option<NaiveDate>,

    /// Start date
    #[serde(
        rename = "@startDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub start_date: Option<NaiveDate>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Collateral amount
    #[serde(
        rename = "@collateralAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub collateral_amount: Option<Decimal>,

    /// Fee rate
    #[serde(
        rename = "@feeRate",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fee_rate: Option<Decimal>,

    /// Fee
    #[serde(
        rename = "@fee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fee: Option<Decimal>,

    /// Carry charge
    #[serde(
        rename = "@carryCharge",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub carry_charge: Option<Decimal>,

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
}

/// Hard to borrow stock details
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HardToBorrowDetail {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Value date
    #[serde(
        rename = "@valueDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub value_date: Option<NaiveDate>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Price
    #[serde(
        rename = "@price",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub price: Option<Decimal>,

    /// Value
    #[serde(
        rename = "@value",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub value: Option<Decimal>,

    /// Borrow fee rate
    #[serde(
        rename = "@borrowFeeRate",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub borrow_fee_rate: Option<Decimal>,

    /// Borrow fee
    #[serde(
        rename = "@borrowFee",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub borrow_fee: Option<Decimal>,

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
}

// =============================================================================
// FX Lot Types
// =============================================================================

/// FX position lot detail
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FxLot {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Functional currency
    #[serde(rename = "@functionalCurrency", default)]
    pub functional_currency: Option<String>,

    /// FX currency
    #[serde(rename = "@fxCurrency", default)]
    pub fx_currency: Option<String>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Cost price
    #[serde(
        rename = "@costPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost_price: Option<Decimal>,

    /// Cost basis
    #[serde(
        rename = "@costBasis",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub cost_basis: Option<Decimal>,

    /// Close price
    #[serde(
        rename = "@closePrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub close_price: Option<Decimal>,

    /// Value
    #[serde(
        rename = "@value",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub value: Option<Decimal>,

    /// Unrealized P&L
    #[serde(
        rename = "@unrealizedPL",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub unrealized_pl: Option<Decimal>,

    /// Level of detail
    #[serde(rename = "@levelOfDetail", default)]
    pub level_of_detail: Option<String>,
}

// =============================================================================
// Transfer Types
// =============================================================================

/// Unsettled transfer
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnsettledTransfer {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Direction
    #[serde(rename = "@direction", default)]
    pub direction: Option<String>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Expected date
    #[serde(
        rename = "@expectedDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub expected_date: Option<NaiveDate>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

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
}

/// Trade transfer between accounts/brokers
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TradeTransfer {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Transfer type
    #[serde(rename = "@transferType", default)]
    pub transfer_type: Option<String>,

    /// Direction
    #[serde(rename = "@direction", default)]
    pub direction: Option<String>,

    /// Delivery type
    #[serde(rename = "@deliveryType", default)]
    pub delivery_type: Option<String>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Transfer price
    #[serde(
        rename = "@transferPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub transfer_price: Option<Decimal>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Executing broker
    #[serde(rename = "@executingBroker", default)]
    pub executing_broker: Option<String>,

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
}

// =============================================================================
// Prior Period Types
// =============================================================================

/// Prior period position
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PriorPeriodPosition {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Prior MTM P&L
    #[serde(
        rename = "@priorMtmPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub prior_mtm_pnl: Option<Decimal>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Quantity
    #[serde(
        rename = "@quantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub quantity: Option<Decimal>,

    /// Price
    #[serde(
        rename = "@price",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub price: Option<Decimal>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}

// =============================================================================
// Interest Detail Types
// =============================================================================

/// Tier interest detail
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TierInterestDetail {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,

    /// FX rate to base currency
    #[serde(
        rename = "@fxRateToBase",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub fx_rate_to_base: Option<Decimal>,

    /// Interest type (e.g., "Debit Interest", "Short Credit Interest")
    #[serde(rename = "@interestType", default)]
    pub interest_type: Option<String>,

    /// Report date
    #[serde(
        rename = "@reportDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub report_date: Option<NaiveDate>,

    /// Value date
    #[serde(
        rename = "@valueDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub value_date: Option<NaiveDate>,

    /// Tier break (Roman numerals like "I", "II", etc.)
    #[serde(rename = "@tierBreak", default)]
    pub tier_break: Option<String>,

    /// Balance threshold
    #[serde(
        rename = "@balanceThreshold",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub balance_threshold: Option<Decimal>,

    /// Securities principal
    #[serde(
        rename = "@securitiesPrincipal",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub securities_principal: Option<Decimal>,

    /// Commodities principal
    #[serde(
        rename = "@commoditiesPrincipal",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commodities_principal: Option<Decimal>,

    /// IBUKL principal
    #[serde(
        rename = "@ibuklPrincipal",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ibukl_principal: Option<Decimal>,

    /// Total principal
    #[serde(
        rename = "@totalPrincipal",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_principal: Option<Decimal>,

    /// Interest rate
    #[serde(
        rename = "@rate",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub rate: Option<Decimal>,

    /// Securities interest
    #[serde(
        rename = "@securitiesInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub securities_interest: Option<Decimal>,

    /// Commodities interest
    #[serde(
        rename = "@commoditiesInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub commodities_interest: Option<Decimal>,

    /// IBUKL interest
    #[serde(
        rename = "@ibuklInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub ibukl_interest: Option<Decimal>,

    /// Total interest
    #[serde(
        rename = "@totalInterest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_interest: Option<Decimal>,

    /// Code
    #[serde(rename = "@code", default)]
    pub code: Option<String>,

    /// From account
    #[serde(rename = "@fromAcct", default)]
    pub from_acct: Option<String>,

    /// To account
    #[serde(rename = "@toAcct", default)]
    pub to_acct: Option<String>,

    /// Margin balance
    #[serde(rename = "@marginBalance", default)]
    pub margin_balance: Option<String>,

    // Legacy fields (may appear in older reports)
    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// From date
    #[serde(
        rename = "@fromDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub from_date: Option<NaiveDate>,

    /// To date
    #[serde(
        rename = "@toDate",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub to_date: Option<NaiveDate>,

    /// Balance (legacy)
    #[serde(
        rename = "@balance",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub balance: Option<Decimal>,

    /// Interest rate (legacy field name)
    #[serde(
        rename = "@interestRate",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub interest_rate: Option<Decimal>,

    /// Interest (legacy)
    #[serde(
        rename = "@interest",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub interest: Option<Decimal>,
}

// =============================================================================
// Miscellaneous Activity Types
// =============================================================================

/// Debit card activity
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DebitCardActivity {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Merchant
    #[serde(rename = "@merchant", default)]
    pub merchant: Option<String>,

    /// Category
    #[serde(rename = "@category", default)]
    pub category: Option<String>,

    /// Status
    #[serde(rename = "@status", default)]
    pub status: Option<String>,

    /// Transaction type
    #[serde(rename = "@transactionType", default)]
    pub transaction_type: Option<String>,

    /// Amount
    #[serde(
        rename = "@amount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub amount: Option<Decimal>,

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
}

/// Sales tax
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SalesTax {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Date
    #[serde(
        rename = "@date",
        default,
        deserialize_with = "deserialize_optional_date"
    )]
    pub date: Option<NaiveDate>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Tax type
    #[serde(rename = "@taxType", default)]
    pub tax_type: Option<String>,

    /// Tax amount
    #[serde(
        rename = "@taxAmount",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub tax_amount: Option<Decimal>,

    /// Proceeds
    #[serde(
        rename = "@proceeds",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub proceeds: Option<Decimal>,

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
}

// =============================================================================
// Summary Types
// =============================================================================

/// Symbol summary (aggregated trading data by symbol)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SymbolSummary {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Total buy quantity
    #[serde(
        rename = "@totalBuyQuantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_buy_quantity: Option<Decimal>,

    /// Total sell quantity
    #[serde(
        rename = "@totalSellQuantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_sell_quantity: Option<Decimal>,

    /// Total buy value
    #[serde(
        rename = "@totalBuyValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_buy_value: Option<Decimal>,

    /// Total sell value
    #[serde(
        rename = "@totalSellValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_sell_value: Option<Decimal>,

    /// Total commission
    #[serde(
        rename = "@totalCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_commission: Option<Decimal>,

    /// Realized P&L
    #[serde(
        rename = "@realizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_pnl: Option<Decimal>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}

/// Asset summary (aggregated trading data by asset class)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetSummary {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Total buy quantity
    #[serde(
        rename = "@totalBuyQuantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_buy_quantity: Option<Decimal>,

    /// Total sell quantity
    #[serde(
        rename = "@totalSellQuantity",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_sell_quantity: Option<Decimal>,

    /// Total buy value
    #[serde(
        rename = "@totalBuyValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_buy_value: Option<Decimal>,

    /// Total sell value
    #[serde(
        rename = "@totalSellValue",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_sell_value: Option<Decimal>,

    /// Total commission
    #[serde(
        rename = "@totalCommission",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub total_commission: Option<Decimal>,

    /// Realized P&L
    #[serde(
        rename = "@realizedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub realized_pnl: Option<Decimal>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}

/// Order record
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Order {
    /// Account ID
    #[serde(rename = "@accountId")]
    pub account_id: String,

    /// Account alias
    #[serde(rename = "@acctAlias", default)]
    pub acct_alias: Option<String>,

    /// Model
    #[serde(rename = "@model", default)]
    pub model: Option<String>,

    /// Order ID
    #[serde(rename = "@orderID", default)]
    pub order_id: Option<String>,

    /// Symbol
    #[serde(rename = "@symbol", default)]
    pub symbol: Option<String>,

    /// Description
    #[serde(rename = "@description", default)]
    pub description: Option<String>,

    /// Contract ID
    #[serde(rename = "@conid", default)]
    pub conid: Option<String>,

    /// Asset category
    #[serde(rename = "@assetCategory", default)]
    pub asset_category: Option<AssetCategory>,

    /// Order time
    #[serde(rename = "@orderTime", default)]
    pub order_time: Option<String>,

    /// Order type
    #[serde(rename = "@orderType", default)]
    pub order_type: Option<String>,

    /// Side (buy/sell)
    #[serde(rename = "@side", default)]
    pub side: Option<String>,

    /// Time in force
    #[serde(rename = "@tif", default)]
    pub tif: Option<String>,

    /// Order quantity
    #[serde(
        rename = "@orderQty",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub order_qty: Option<Decimal>,

    /// Limit price
    #[serde(
        rename = "@limitPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub limit_price: Option<Decimal>,

    /// Stop price
    #[serde(
        rename = "@stopPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub stop_price: Option<Decimal>,

    /// Filled quantity
    #[serde(
        rename = "@filledQty",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub filled_qty: Option<Decimal>,

    /// Average fill price
    #[serde(
        rename = "@avgPrice",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub avg_price: Option<Decimal>,

    /// Remaining quantity
    #[serde(
        rename = "@remainingQty",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    pub remaining_qty: Option<Decimal>,

    /// Order status
    #[serde(rename = "@status", default)]
    pub status: Option<String>,

    /// Currency
    #[serde(rename = "@currency", default)]
    pub currency: Option<String>,
}
