//! FLEX data types

pub mod activity;
pub mod common;
pub mod extended;
pub mod trade_confirmation;

// Re-export commonly used types
pub use activity::{
    ActivityFlexStatement, CashTransaction, CashTransactionsWrapper, ConversionRate,
    ConversionRatesWrapper, CorporateAction, CorporateActionsWrapper, FlexQueryResponse,
    FlexStatementsWrapper, Position, PositionsWrapper, SecuritiesInfoWrapper, SecurityInfo, Trade,
    TradesWrapper,
};
pub use common::{
    AssetCategory, BuySell, CashAction, CorporateActionType, DeliveredReceived, InOut, LongShort,
    OpenClose, OptionAction, OrderType, PutCall, ToFrom, TradeType, TransactionCode, TransferType,
};
pub use extended::{
    // Account info
    AccountInformation,
    AssetSummary,
    // Cash
    CashReportCurrency,
    // Dividends and interest
    ChangeInDividendAccrual,
    // NAV and equity
    ChangeInNAV,
    // Position changes
    ChangeInPositionValue,
    ClientFee,
    ClientFeesDetail,
    // Miscellaneous
    DebitCardActivity,
    EquitySummaryByReportDateInBase,
    FIFOPerformanceSummaryUnderlying,
    // FX
    FxLot,
    FxTransaction,
    HardToBorrowDetail,
    InterestAccrualsCurrency,
    MTDYTDPerformanceSummary,
    // Performance summaries
    MTMPerformanceSummaryUnderlying,
    OpenDividendAccrual,
    OptionEAE,
    Order,
    // Prior period
    PriorPeriodPosition,
    // Securities lending
    SLBActivity,
    SLBFee,
    SalesTax,
    // Statement of funds
    StatementOfFundsLine,
    // Summaries
    SymbolSummary,
    // Interest details
    TierInterestDetail,
    // Trading
    TradeConfirm,
    TradeTransfer,
    // Transfers
    Transfer,
    // Fee details
    UnbundledCommissionDetail,
    UnsettledTransfer,
};
pub use trade_confirmation::TradeConfirmationStatement;
