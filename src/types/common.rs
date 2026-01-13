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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CashAction {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Reorg {
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
/// Comprehensive list of IB transaction classification codes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Code {
    /// Assignment
    A,

    /// Adjustment
    Adj,

    /// Allocation
    Al,

    /// Auto exercise
    Ae,

    /// Auto FX
    Af,

    /// Away trade
    Aw,

    /// Buy-in
    B,

    /// Borrow fee
    Bo,

    /// Cancellation
    Ca,

    /// Closing
    C,

    /// Cash delivery
    Cd,

    /// Complex position
    Cp,

    /// Correct
    Cr,

    /// Crossing
    Cs,

    /// Dual
    D,

    /// ETF creation
    Et,

    /// Expired
    Ex,

    /// Exercise
    O,

    /// Guaranteed
    G,

    /// Highest cost
    Hc,

    /// HF investment
    Hi,

    /// HF redemption
    Hr,

    /// Internal transfer
    I,

    /// Affiliated account transfer
    Ia,

    /// Investor
    Iv,

    /// Margin low
    L,

    /// LIFO (Last In First Out)
    Li,

    /// Loan
    Ln,

    /// Long-term capital gain
    Lt,

    /// Maximum loss
    M,

    /// Maximum long-term capital gain
    Ml,

    /// Minimum long-term capital gain
    Mn,

    /// Maximum short-term capital gain
    Ms,

    /// Minimum short-term capital gain
    Mi,

    /// Manual exercise
    Mx,

    /// Opening
    P,

    /// Partial execution
    Pt,

    /// Fractional risk-less principal
    Fr,

    /// Fractional principal
    Fp,

    /// Price improvement
    Pi,

    /// Post accrual
    Pa,

    /// Principal
    Pr,

    /// Reinvestment
    Re,

    /// Redemption
    Rd,

    /// Reopen
    R,

    /// Reverse
    Rv,

    /// Reimbursement
    Ri,

    /// Solicited IB
    Si,

    /// Specific lot
    Sp,

    /// Solicited other
    So,

    /// Shortened settlement
    Ss,

    /// Short-term capital gain
    St,

    /// Stock yield
    Sy,

    /// Transfer
    T,

    /// Wash sale
    W,

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
