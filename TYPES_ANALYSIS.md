# IBKR FLEX Types - Comprehensive Analysis

**Source**: Analysis of [csingley/ibflex](https://github.com/csingley/ibflex) Python library
**Date**: 2026-01-12
**Purpose**: Ensure ib-flex Rust library has feature parity with the mature Python implementation

---

## Overview

The ibflex Python library supports **41 distinct data types** across Activity FLEX statements. Our Rust library must support all of these for comprehensive FLEX query parsing.

---

## Type Categories

### 1. Top-Level Container Types

| Type | Description | Priority |
|------|-------------|----------|
| `FlexQueryResponse` | Root element containing one or more statements | **Critical** |
| `FlexStatement` | Individual statement with account data | **Critical** |

### 2. Account & Summary Data (8 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `AccountInformation` | Account details and contact info | accountId, currency, accountType, name, address | High |
| `ChangeInNAV` | Period-over-period portfolio value changes | startingValue, transfers, mtm, realized, ending | High |
| `EquitySummaryByReportDateInBase` | Asset breakdown by value | cash, stock, options, bonds, total | High |
| `MTMPerformanceSummaryUnderlying` | Mark-to-market performance per security | symbol, proceeds, commissions, mtmPnl | Medium |
| `MTDYTDPerformanceSummaryUnderlying` | Month/year-to-date performance | mtdPnl, ytdPnl, inception | Medium |
| `FIFOPerformanceSummaryUnderlying` | FIFO accounting performance | realizedST, realizedLT, unrealized | Medium |
| `CashReportCurrency` | Cash activity by currency | startingCash, commissions, deposits, dividends | High |
| `NetStockPosition` | Borrowed/lent share summary | symbol, quantity, value, shortBalance | Medium |

### 3. Positions & Holdings (3 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `OpenPosition` | Current holdings | symbol, position, markPrice, costBasis, fifoPnl | **Critical** |
| `FxLot` | Foreign exchange position lots | openDate, quantity, cost, proceeds | Medium |
| `PriorPeriodPosition` | Prior period position data | priorMtmPnl, priorRealizedPnl | Low |

### 4. Trade Executions (4 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `Trade` | Executed trades | symbol, buySell, quantity, price, commission, fifoPnl | **Critical** |
| `TradeConfirm` | Trade confirmations | execID, orderID, tradeDate, tradeTime | High |
| `TradeTransfer` | Inter-broker transfers | direction, accountTo, accountFrom | Medium |
| `OptionEAE` | Option exercise/assignment/expiration | actionType, quantity, strike, underlying | High |

### 5. Cash Transactions (2 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `CashTransaction` | Cash activities (deposits, dividends, etc.) | type, amount, currency, description, dateTime | **Critical** |
| `FxTransaction` | Currency conversions | fromCurrency, toCurrency, quantity, proceeds, fxRate | High |

### 6. Transfers (2 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `Transfer` | Security transfers | type (ACATS, FOP), symbol, quantity, direction | Medium |
| `UnsettledTransfer` | Pending transfers | symbol, quantity, date | Low |

### 7. Fees & Charges (7 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `UnbundledCommissionDetail` | Commission component breakdown | execution, clearing, regulatory, taxes | Medium |
| `ClientFee` | Advisory/service fees | amount, description, acctAlias | Medium |
| `ClientFeesDetail` | Detailed fee charges | feeType, amount, basis | Medium |
| `TierInterestDetail` | Tiered interest rates | tier, rate, balance | Low |
| `HardToBorrowDetail` | Short-borrow fees | symbol, rate, dailyCharge | Medium |
| `SLBActivity` | Securities lending activity | symbol, quantity, collateral, income | Low |
| `SLBFee` | Lending fees | symbol, fee, currency | Low |

### 8. Corporate Actions & Accruals (4 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `CorporateAction` | Stock splits, dividends, mergers | type, actionID, quantity, proceeds, fifoPnl | **Critical** |
| `ChangeInDividendAccrual` | Dividend accrual changes | exDate, payDate, grossRate, netAmount | High |
| `OpenDividendAccrual` | Pending dividends | symbol, exDate, payDate, quantity, rate | High |
| `InterestAccrualsCurrency` | Interest accruals by currency | fromDate, toDate, startBalance, accrued, ending | High |

### 9. Reference Data (2 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `SecurityInfo` | Security identification/characteristics | conid, symbol, cusip, isin, multiplier, strike, expiry | **Critical** |
| `ConversionRate` | FX conversion rates | reportDate, fromCurrency, toCurrency, rate | **Critical** |

### 10. Other Activity (4 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `StatementOfFundsLine` | Fund flow details | activityCode, amount, date, description | Medium |
| `ChangeInPositionValue` | Position value reconciliation | priorValue, transactions, mtm, endingValue | Medium |
| `DebitCardActivity` | Debit card transactions | postingDate, description, amount | Low |
| `SalesTax` | Transaction taxes | symbol, taxAmount, date | Low |

### 11. Trade Confirmation Specific (3 types)

| Type | Description | Fields | Priority |
|------|-------------|--------|----------|
| `SymbolSummary` | Symbol-level trade summary | symbol, totalQuantity, avgPrice | Medium |
| `AssetSummary` | Asset class summary | assetCategory, priorValue, tradeActivity | Medium |
| `Order` | Order records | orderID, orderType, status, limitPrice | Medium |

---

## Shared Enums (15 types)

| Enum | Values | Usage |
|------|--------|-------|
| `AssetClass` | STK, OPT, FUT, FOP, CASH, BOND, CMDTY, CFD, etc. | All security types |
| `BuySell` | BUY, SELL, BUY_CANCEL, SELL_CANCEL | Trade direction |
| `OpenClose` | Open, Close, CloseOpen, Unknown | Position lifecycle |
| `PutCall` | P, C | Option type |
| `LongShort` | Long, Short | Position side |
| `TradeType` | ExchTrade, BookTrade, FracShare, Adjustment, etc. | Trade classification |
| `OrderType` | LMT, MKT, STP, TRAIL, MOC, etc. | Order types |
| `CashAction` | Deposits, Dividends, Interest, Fees, Taxes, etc. | Cash transaction types |
| `Reorg` | Merger, Spinoff, Split, StockDividend, etc. | Corporate action types |
| `OptionAction` | Assignment, Exercise, Expiration, Buy, Sell | Option events |
| `TransferType` | ACATS, ATON, FOP, INTERNAL | Transfer methods |
| `Code` | A, C, Ex, P, Ca, D, O, etc. | Transaction codes |
| `ToFrom` | To, From | Direction |
| `InOut` | IN, OUT | Flow direction |
| `DeliveredReceived` | Delivered, Received | Delivery status |

---

## Field Type Mapping

### Rust Types to Use

| IB XML Type | Rust Type | Notes |
|-------------|-----------|-------|
| Date (YYYY-MM-DD) | `chrono::NaiveDate` | ISO 8601 |
| DateTime | `chrono::NaiveDateTime` | With time component |
| Time | `chrono::NaiveTime` | Time only |
| Decimal | `rust_decimal::Decimal` | **NEVER** f32/f64 for money |
| Integer | `i64` | IB uses large transaction IDs |
| String | `String` | Symbols, descriptions |
| Boolean | `bool` | Flags |
| Currency | `String` (3-char code) | USD, EUR, GBP, etc. |

---

## Implementation Priority

### Phase 1: Core Trading (v0.1.0)
**MUST HAVE** for minimum viable library:
- FlexQueryResponse / FlexStatement
- Trade
- OpenPosition
- CashTransaction
- CorporateAction
- SecurityInfo
- ConversionRate
- All shared enums

### Phase 2: Comprehensive Support (v0.2.0)
**SHOULD HAVE** for full Activity FLEX:
- AccountInformation
- ChangeInNAV
- EquitySummaryByReportDateInBase
- CashReportCurrency
- TradeConfirm
- OptionEAE
- FxTransaction
- ChangeInDividendAccrual
- OpenDividendAccrual
- InterestAccrualsCurrency

### Phase 3: Advanced Features (v0.3.0)
**NICE TO HAVE** for complete coverage:
- Performance summaries (MTM, FIFO, MTD/YTD)
- Fee details (UnbundledCommission, ClientFees, etc.)
- Securities lending (SLBActivity, SLBFee)
- Transfers (ACATS, FOP)
- Trade Confirmation types (Order, SymbolSummary, etc.)

### Phase 4: Edge Cases (v0.4.0)
**OPTIONAL** for niche use cases:
- NetStockPosition
- PriorPeriodPosition
- DebitCardActivity
- SalesTax
- HardToBorrowDetail
- TierInterestDetail

---

## Key Differences from Original Plan

### Original CLAUDE.md Scope (6 types)
- Trade
- Position
- CashTransaction
- CorporateAction
- FxRate
- SecurityInfo

### Actual Required Scope (41 types)
**Core types (Phase 1)**: 7 types
**Comprehensive (Phase 2)**: +10 types (17 total)
**Advanced (Phase 3)**: +14 types (31 total)
**Complete (Phase 4)**: +10 types (41 total)

**Conclusion**: Original plan covered only ~15% of full FLEX functionality. We need to expand significantly.

---

## Critical Fields by Type

### Trade (Most Complex)
- **93 fields total** in ibflex implementation
- Key fields: transactionID, symbol, conid, buySell, quantity, tradePrice, proceeds, commission, fifoPnlRealized
- Options: strike, expiry, putCall, underlying*
- P&L: mtmPnl, fifoPnlRealized, capitalGainsPnl, fxPnl
- Timestamps: tradeDate, tradeTime, settleDateTarget, openDateTime, whenRealized

### OpenPosition (Second Most Complex)
- **58 fields total**
- Key fields: position, markPrice, costBasisPrice, fifoPnlUnrealized
- Options: strike, expiry, putCall, underlying*
- Timestamps: openDateTime, holdingPeriodDateTime

### CashTransaction (Third Most Complex)
- **48 fields total**
- Key fields: type, amount, dateTime, description
- Links: transactionID, tradeID, actionID

---

## Testing Requirements

For comprehensive testing, we need XML fixtures covering:

1. **Basic trades**: Stock, option, futures
2. **Corporate actions**: Splits, mergers, dividends
3. **Cash flows**: Deposits, withdrawals, interest, dividends
4. **Positions**: Long/short, across asset classes
5. **Multi-currency**: FX trades and conversions
6. **Complex instruments**: Multi-leg options, futures spreads
7. **Edge cases**: Fractional shares, assignment/exercise, transfers

---

## Official IB Documentation

- [Activity FLEX Query Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm)
- [FLEX Queries Guide](https://www.ibkrguides.com/orgportal/performanceandstatements/flex.htm)
- [FLEX Web Service API](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)

---

## Next Steps

1. ✅ Analyze ibflex Python library for completeness
2. ⏳ Update PLAN.md with phased approach (v0.1 → v0.4)
3. ⏳ Update CLAUDE.md with comprehensive type list
4. ⏳ Create type definitions for Phase 1 (core types)
5. ⏳ Gather real XML examples for test fixtures
6. ⏳ Implement parsers with comprehensive tests

---

*Last Updated: 2026-01-12*
