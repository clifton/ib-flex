//! Common enums used across FLEX statements

use serde::{Deserialize, Serialize};

/// Asset category (security type)
///
/// Maps to IB's AssetCategory field. Represents the type of financial instrument.
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

    /// Treasury Bill (maturity < 1 year)
    #[serde(rename = "BILL")]
    Bill,

    /// Commodity
    #[serde(rename = "CMDTY")]
    Commodity,

    /// Contract for Difference
    #[serde(rename = "CFD")]
    Cfd,

    /// Forex CFD
    #[serde(rename = "FXCFD")]
    ForexCfd,

    /// Warrant
    #[serde(rename = "WAR")]
    Warrant,

    /// Mutual Fund
    #[serde(rename = "FUND")]
    Fund,

    /// Structured Product / Dutch Warrant / Indexed Option
    #[serde(rename = "IOPT")]
    StructuredProduct,

    /// Combination / Basket order (spread, combo legs)
    #[serde(rename = "BAG")]
    Bag,

    /// Cryptocurrency
    #[serde(rename = "CRYPTO")]
    Cryptocurrency,

    /// Physical metals (gold, silver, etc.)
    #[serde(rename = "METAL")]
    Metal,

    /// Exchange for Physical
    #[serde(rename = "EFP")]
    ExchangeForPhysical,

    /// Event Contract
    #[serde(rename = "EC")]
    EventContract,

    /// Index
    #[serde(rename = "IND")]
    Index,

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

    /// Cancelled buy
    #[serde(rename = "BUY (Ca.)")]
    CancelBuy,

    /// Cancelled sell
    #[serde(rename = "SELL (Ca.)")]
    CancelSell,

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

    /// Trailing limit
    #[serde(rename = "TRAIL LMT")]
    TrailingLimit,

    /// Mid-price order
    #[serde(rename = "MIDPX")]
    MidPrice,

    /// Relative order
    #[serde(rename = "REL")]
    Relative,

    /// Multiple order types (complex orders)
    #[serde(rename = "MULTIPLE")]
    Multiple,

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

/// Long or Short position side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum LongShort {
    /// Long position
    Long,

    /// Short position
    Short,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Transaction type for trades
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TradeType {
    /// Exchange trade
    ExchTrade,

    /// Book trade
    BookTrade,

    /// Delivery vs Payment trade
    DvpTrade,

    /// Fractional share trade
    FracShare,

    /// Fractional share cancellation
    FracShareCancel,

    /// Manual adjustment
    Adjustment,

    /// Trade correction
    TradeCorrect,

    /// Trade cancellation
    TradeCancel,

    /// IBKR trade
    IBKRTrade,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Cash transaction action type
///
/// Represents the type of cash transaction (dividend, interest, fee, etc.).
/// This enum is used by the `CashTransaction` struct to classify cash activity
/// in the account statement.
///
/// **XML Mapping**: Maps to the `type` attribute in `<CashTransaction>` elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CashTransactionType {
    /// Deposits and withdrawals
    #[serde(rename = "Deposits & Withdrawals")]
    DepositsWithdrawals,

    /// Dividend payments
    Dividends,

    /// Withholding tax
    WithholdingTax,

    /// Broker interest paid
    #[serde(rename = "Broker Interest Paid")]
    BrokerInterestPaid,

    /// Broker interest received
    #[serde(rename = "Broker Interest Received")]
    BrokerInterestReceived,

    /// Bond interest received
    #[serde(rename = "Bond Interest Received")]
    BondInterestReceived,

    /// Bond interest paid
    #[serde(rename = "Bond Interest Paid")]
    BondInterestPaid,

    /// Bond interest (generic)
    #[serde(rename = "Bond Interest")]
    BondInterest,

    /// Payment in lieu of dividends
    #[serde(rename = "Payment In Lieu Of Dividends")]
    PaymentInLieuOfDividends,

    /// Other fees
    #[serde(rename = "Other Fees")]
    OtherFees,

    /// Commission adjustments
    #[serde(rename = "Commission Adjustments")]
    CommissionAdjustments,

    /// Advisor fees
    #[serde(rename = "Advisor Fees")]
    AdvisorFees,

    /// Cash receipts
    #[serde(rename = "Cash Receipts")]
    CashReceipts,

    /// Fees
    Fees,

    /// Unknown type
    #[serde(other)]
    Unknown,
}

/// Corporate action reorganization type
///
/// Represents the type of corporate action (split, merger, spinoff, etc.).
/// This enum is used by the `CorporateAction` struct to classify corporate events
/// that affect security positions and holdings.
///
/// **XML Mapping**: Maps to the `type` attribute in `<CorporateAction>` elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CorporateActionType {
    /// Stock split (forward split)
    #[serde(rename = "Stock Split")]
    StockSplit,

    /// Forward split (issue)
    #[serde(rename = "Forward Split (Issue)")]
    ForwardSplitIssue,

    /// Forward split
    #[serde(rename = "Forward Split")]
    ForwardSplit,

    /// Reverse split
    #[serde(rename = "Reverse Split")]
    ReverseSplit,

    /// Merger
    Merger,

    /// Spinoff
    Spinoff,

    /// Contract spinoff
    #[serde(rename = "Contract Spinoff")]
    ContractSpinoff,

    /// Stock dividend
    #[serde(rename = "Stock Dividend")]
    StockDividend,

    /// Cash dividend
    #[serde(rename = "Cash Dividend")]
    CashDividend,

    /// Choice dividend
    #[serde(rename = "Choice Dividend")]
    ChoiceDividend,

    /// Choice dividend (delivery)
    #[serde(rename = "Choice Dividend (Delivery)")]
    ChoiceDivDelivery,

    /// Choice dividend (issue)
    #[serde(rename = "Choice Dividend (Issue)")]
    ChoiceDivIssue,

    /// Dividend rights issue
    #[serde(rename = "Dividend Rights Issue")]
    DivRightsIssue,

    /// Expired dividend right
    #[serde(rename = "Expired Dividend Right")]
    ExpiredDivRight,

    /// Delisted
    Delisted,

    /// Delist (worthless)
    #[serde(rename = "Delist (Worthless)")]
    DelistWorthless,

    /// Name change
    #[serde(rename = "Name Change")]
    NameChange,

    /// Symbol change
    #[serde(rename = "Symbol Change")]
    SymbolChange,

    /// Issue change
    #[serde(rename = "Issue Change")]
    IssueChange,

    /// Bond conversion
    #[serde(rename = "Bond Conversion")]
    BondConversion,

    /// Bond maturity
    #[serde(rename = "Bond Maturity")]
    BondMaturity,

    /// T-Bill maturity
    #[serde(rename = "T-Bill Maturity")]
    TBillMaturity,

    /// Convertible issue
    #[serde(rename = "Convertible Issue")]
    ConvertibleIssue,

    /// Coupon payment
    #[serde(rename = "Coupon Payment")]
    CouponPayment,

    /// Contract consolidation
    #[serde(rename = "Contract Consolidation")]
    ContractConsolidation,

    /// Contract split
    #[serde(rename = "Contract Split")]
    ContractSplit,

    /// Contract termination
    #[serde(rename = "CFD Termination")]
    CfdTermination,

    /// Fee allocation
    #[serde(rename = "Fee Allocation")]
    FeeAllocation,

    /// Rights issue
    #[serde(rename = "Rights Issue")]
    RightsIssue,

    /// Subscribe rights
    #[serde(rename = "Subscribe Rights")]
    SubscribeRights,

    /// Tender
    Tender,

    /// Tender (issue)
    #[serde(rename = "Tender (Issue)")]
    TenderIssue,

    /// Proxy vote
    #[serde(rename = "Proxy Vote")]
    ProxyVote,

    /// Generic voluntary
    #[serde(rename = "Generic Voluntary")]
    GenericVoluntary,

    /// Asset purchase
    #[serde(rename = "Asset Purchase")]
    AssetPurchase,

    /// Purchase (issue)
    #[serde(rename = "Purchase (Issue)")]
    PurchaseIssue,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Option action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum OptionAction {
    /// Assignment
    Assignment,

    /// Exercise
    Exercise,

    /// Expiration
    Expiration,

    /// Expire (alternate form)
    Expire,

    /// Cash settlement
    #[serde(rename = "Cash Settlement")]
    CashSettlement,

    /// Buy to open/close
    Buy,

    /// Sell to open/close
    Sell,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Transfer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TransferType {
    /// ACATS transfer
    ACATS,

    /// ATON transfer
    ATON,

    /// Free of payment
    FOP,

    /// Internal transfer
    INTERNAL,

    /// Delivery vs payment
    DVP,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Transaction code
///
/// Comprehensive list of IB transaction classification codes.
/// These codes appear in `notes` fields and can be combined (e.g., "C;W" for closing + wash sale).
/// They provide critical context for tax reporting and trade classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TransactionCode {
    /// Assignment - Option assignment triggering stock delivery
    #[serde(rename = "A")]
    Assignment,

    /// Adjustment - Manual adjustment affecting cost basis
    #[serde(rename = "Adj")]
    Adjustment,

    /// Allocation - Trade allocation to sub-account (master/sub allocation)
    #[serde(rename = "Al")]
    Allocation,

    /// Auto Exercise - Automatic exercise (dividend-related, exercise before ex-div)
    #[serde(rename = "Ae")]
    AutoExercise,

    /// Auto FX - AutoFX currency conversion for settlement
    #[serde(rename = "Af")]
    AutoFx,

    /// Away Trade - Trade executed away from IB (third-party execution)
    #[serde(rename = "Aw")]
    AwayTrade,

    /// Buy-In - Forced purchase to cover failed delivery (forced short cover)
    #[serde(rename = "B")]
    BuyIn,

    /// Borrow - Securities borrowing fee (lending charge)
    #[serde(rename = "Bo")]
    BorrowFee,

    /// Cancellation - Trade cancelled/busted (trade reversed)
    #[serde(rename = "Ca")]
    Cancelled,

    /// Closing - Closing trade (reduces position)
    #[serde(rename = "C")]
    Closing,

    /// Cash Delivery - Cash delivery for exercise (cash vs physical)
    #[serde(rename = "Cd")]
    CashDelivery,

    /// Complex Position - Complex/combo position (multi-leg strategy)
    #[serde(rename = "Cp")]
    ComplexPosition,

    /// Correction - Trade correction (amended execution)
    #[serde(rename = "Cr")]
    Correction,

    /// Crossing - Internal IB cross (matched internally)
    #[serde(rename = "Cs")]
    Crossing,

    /// Dual Agent - IB dual agent capacity (disclosed dual role)
    #[serde(rename = "D")]
    DualAgent,

    /// ETF - ETF creation/redemption (in-kind basket)
    #[serde(rename = "Et")]
    Etf,

    /// Expired - From expired position (option/warrant expiry)
    #[serde(rename = "Ex")]
    Expired,

    /// Exercise - Option exercise (long option exercised)
    #[serde(rename = "O")]
    Exercise,

    /// Guaranteed - Guaranteed account segment (special margin)
    #[serde(rename = "G")]
    Guaranteed,

    /// Highest Cost - Highest cost tax lot (tax lot selection)
    #[serde(rename = "Hc")]
    HighestCost,

    /// HF Investment - Hedge fund investment (fund subscription)
    #[serde(rename = "Hi")]
    HfInvestment,

    /// HF Redemption - Hedge fund redemption (fund redemption)
    #[serde(rename = "Hr")]
    HfRedemption,

    /// Internal - Internal transfer (between IB accounts)
    #[serde(rename = "I")]
    InternalTransfer,

    /// Affiliate - Affiliate execution (related party trade)
    #[serde(rename = "Ia")]
    Affiliate,

    /// Investor - Investment from investor (capital contribution)
    #[serde(rename = "Iv")]
    Investor,

    /// Margin Violation - Liquidation due to margin (forced liquidation)
    #[serde(rename = "L")]
    MarginLiquidation,

    /// LIFO - LIFO tax lot (tax lot selection)
    #[serde(rename = "Li")]
    Lifo,

    /// Loan - Securities lending income (lending income)
    #[serde(rename = "Ln")]
    Loan,

    /// Long-Term - Long-term gain/loss (holding > 1 year)
    #[serde(rename = "Lt")]
    LongTermGain,

    /// Manual - Manual IB entry (manual adjustment)
    #[serde(rename = "M")]
    ManualEntry,

    /// Max Loss - Maximize losses (tax optimization)
    #[serde(rename = "Ml")]
    MaxLoss,

    /// Min LT Gain - Minimize long-term gain (tax optimization)
    #[serde(rename = "Mn")]
    MinLongTermGain,

    /// Max ST Gain - Maximize short-term gain (tax optimization)
    #[serde(rename = "Ms")]
    MaxShortTermGain,

    /// Min ST Gain - Minimize short-term gain (tax optimization)
    #[serde(rename = "Mi")]
    MinShortTermGain,

    /// Manual Exercise - Manual exercise (discretionary exercise)
    #[serde(rename = "Mx")]
    ManualExercise,

    /// Opening - Opening trade (new position)
    #[serde(rename = "P")]
    Opening,

    /// Partial - Partial execution (partial fill)
    #[serde(rename = "Pt")]
    Partial,

    /// Frac Riskless - Fractional riskless principal (fractional share method)
    #[serde(rename = "Fr")]
    FracRiskless,

    /// Frac Principal - Fractional principal (fractional share method)
    #[serde(rename = "Fp")]
    FracPrincipal,

    /// Price Improvement - Better than quoted (price improvement)
    #[serde(rename = "Pi")]
    PriceImprovement,

    /// Post Accrual - Accrual posting (accrual entry)
    #[serde(rename = "Pa")]
    PostAccrual,

    /// Principal - IB principal execution (principal trade)
    #[serde(rename = "Pr")]
    Principal,

    /// Reinvestment - Dividend reinvestment (DRIP)
    #[serde(rename = "Re")]
    Reinvestment,

    /// Redemption - Capital distribution (fund redemption)
    #[serde(rename = "Rd")]
    Redemption,

    /// Reopen - Position reopened (wash sale reopen)
    #[serde(rename = "R")]
    Reopen,

    /// Reverse - Accrual reversal (accounting reversal)
    #[serde(rename = "Rv")]
    Reverse,

    /// Reimbursement - Fee refund (expense refund)
    #[serde(rename = "Ri")]
    Reimbursement,

    /// Solicited IB - IB solicited order (IB-initiated)
    #[serde(rename = "Si")]
    SolicitedIb,

    /// Specific Lot - Specific tax lot (tax lot selection)
    #[serde(rename = "Sp")]
    SpecificLot,

    /// Solicited Other - Third-party solicited (broker-solicited)
    #[serde(rename = "So")]
    SolicitedOther,

    /// Short Settlement - T+0 or T+1 settlement (accelerated settle)
    #[serde(rename = "Ss")]
    ShortSettlement,

    /// Short-Term - Short-term gain/loss (holding <= 1 year)
    #[serde(rename = "St")]
    ShortTermGain,

    /// Stock Yield - Stock yield eligible (lending eligible)
    #[serde(rename = "Sy")]
    StockYield,

    /// Transfer - Position transfer
    #[serde(rename = "T")]
    Transfer,

    /// Wash Sale - Wash sale (loss disallowed)
    #[serde(rename = "W")]
    WashSale,

    /// Unknown code
    #[serde(other)]
    Unknown,
}

/// Direction (To/From)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ToFrom {
    /// To
    To,

    /// From
    From,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Direction (In/Out)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum InOut {
    /// Incoming
    IN,

    /// Outgoing
    OUT,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Delivered or Received
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeliveredReceived {
    /// Delivered
    Delivered,

    /// Received
    Received,

    /// Unknown
    #[serde(other)]
    Unknown,
}

/// Level of detail for reporting
///
/// Specifies the granularity of data in FLEX reports.
/// Used by `Trade`, `Position`, and `CashTransaction` structs to indicate
/// the level of detail requested in the FLEX query.
///
/// **XML Mapping**: Maps to the `levelOfDetail` attribute in various elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum LevelOfDetail {
    /// Summary level - aggregated data with minimal details
    Summary,

    /// Detail level - standard reporting with all key fields
    Detail,

    /// Execution level - detailed execution information including time and venue
    Execution,

    /// Lot level - tax lot level details for cost basis tracking
    Lot,

    /// Unknown or unrecognized level of detail
    #[serde(other)]
    Unknown,
}

/// Security identifier type
///
/// Specifies the type of security identifier used in the `securityID` field.
/// Different identifiers are used in different markets and contexts.
///
/// **XML Mapping**: Maps to the `securityIDType` attribute in various elements.
///
/// **Used by**: `Trade`, `SecurityInfo`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SecurityIdType {
    /// CUSIP - Committee on Uniform Securities Identification Procedures
    /// 9-character alphanumeric identifier for North American securities
    #[serde(rename = "CUSIP")]
    Cusip,

    /// ISIN - International Securities Identification Number
    /// 12-character alphanumeric code (ISO 6166 standard)
    #[serde(rename = "ISIN")]
    Isin,

    /// FIGI - Financial Instrument Global Identifier
    /// 12-character alphanumeric identifier (Bloomberg Open Symbology)
    #[serde(rename = "FIGI")]
    Figi,

    /// SEDOL - Stock Exchange Daily Official List
    /// 7-character alphanumeric identifier for UK and Irish securities
    #[serde(rename = "SEDOL")]
    Sedol,

    /// Unknown or unrecognized security ID type
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_category_basic() {
        // Test enum construction and comparison
        let stock = AssetCategory::Stock;
        assert_eq!(stock, AssetCategory::Stock);
        assert_ne!(stock, AssetCategory::Option);
    }

    #[test]
    fn test_buy_sell_basic() {
        // Test enum construction and comparison
        let buy = BuySell::Buy;
        assert_eq!(buy, BuySell::Buy);
        assert_ne!(buy, BuySell::Sell);
    }

    #[test]
    fn test_open_close_basic() {
        let open = OpenClose::Open;
        assert_eq!(open, OpenClose::Open);
        assert_ne!(open, OpenClose::Close);
    }

    #[test]
    fn test_put_call_basic() {
        let call = PutCall::Call;
        assert_eq!(call, PutCall::Call);
        assert_ne!(call, PutCall::Put);
    }
}
