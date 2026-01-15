# IB FLEX Type Reference

**Version**: 0.1.5
**Last Updated**: 2026-01-14
**Source**: Interactive Brokers FLEX API + [csingley/ibflex](https://github.com/csingley/ibflex) Python reference

This document serves as the authoritative reference for all IB FLEX types supported by this library.

---

## Table of Contents

1. [Overview & Quick Reference](#overview--quick-reference)
2. [Enum Reference](#enum-reference)
3. [Struct Reference](#struct-reference)
4. [Gaps Analysis](#gaps-analysis)
5. [Implementation Checklist](#implementation-checklist)

---

## Overview & Quick Reference

### Type Counts

| Category | Count | Status |
|----------|-------|--------|
| **Enums** | 15 | Implemented |
| **Core Structs** | 7 | Implemented |
| **Extended Structs** | 33 | Implemented |
| **Wrapper Types** | 26 | Implemented |
| **Unparsed Sections** | 23 | Not implemented |

### Enum Summary

| Enum | Variants | Purpose |
|------|----------|---------|
| `AssetCategory` | 19 | Security type (STK, OPT, FUT, etc.) |
| `BuySell` | 5 | Trade direction |
| `OpenClose` | 4 | Position lifecycle |
| `OrderType` | 14 | Order type |
| `PutCall` | 3 | Option type |
| `LongShort` | 3 | Position side |
| `TradeType` | 10 | Trade classification |
| `CashAction` | 14 | Cash transaction type |
| `Reorg` | 39 | Corporate action type |
| `OptionAction` | 8 | Option event type |
| `TransferType` | 6 | Transfer method |
| `Code` | 45 | Transaction codes |
| `ToFrom` | 3 | Direction |
| `InOut` | 3 | Flow direction |
| `DeliveredReceived` | 3 | Delivery status |

### Core Struct Summary

| Struct | Fields | Description |
|--------|--------|-------------|
| `Trade` | 77 | Trade execution details |
| `Position` | 58 | Open position |
| `CashTransaction` | 48 | Cash activity |
| `CorporateAction` | 57 | Corporate action |
| `SecurityInfo` | 29 | Security reference data |
| `ConversionRate` | 4 | FX conversion rate |

---

## Enum Reference

### AssetCategory

Security type classification. Maps to IB's `assetCategory` XML attribute.

**Used by**: `Trade`, `Position`, `CashTransaction`, `CorporateAction`, `SecurityInfo`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Stock` | `STK` | Common/preferred stock, ETFs, ADRs |
| `Option` | `OPT` | Equity and index options |
| `Future` | `FUT` | Futures contracts |
| `FutureOption` | `FOP` | Options on futures |
| `Cash` | `CASH` | Forex/currency pairs |
| `Bond` | `BOND` | Corporate and government bonds |
| `Bill` | `BILL` | Treasury bills (maturity < 1 year) |
| `Commodity` | `CMDTY` | Physical commodities |
| `Cfd` | `CFD` | Contract for difference |
| `ForexCfd` | `FXCFD` | Forex CFD |
| `Warrant` | `WAR` | Warrants |
| `Fund` | `FUND` | Mutual funds |
| `StructuredProduct` | `IOPT` | Structured products, Dutch warrants |
| `Bag` | `BAG` | Combination/spread orders |
| `Cryptocurrency` | `CRYPTO` | Cryptocurrency |
| `Metal` | `METAL` | Physical metals (gold, silver) |
| `ExchangeForPhysical` | `EFP` | Exchange for physical |
| `EventContract` | `EC` | Event contracts |
| `Index` | `IND` | Index |
| `Unknown` | `*` | Fallback for unrecognized types |

---

### BuySell

Trade direction indicator.

**Used by**: `Trade`, `TradeConfirm`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Buy` | `BUY` | Purchase transaction |
| `Sell` | `SELL` | Sale transaction |
| `CancelBuy` | `BUY (Ca.)` | Cancelled buy (trade bust) |
| `CancelSell` | `SELL (Ca.)` | Cancelled sell (trade bust) |
| `Unknown` | `*` | Fallback |

---

### OpenClose

Indicates whether trade opens or closes a position. Critical for options/futures.

**Used by**: `Trade`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Open` | `O` | Opens new position |
| `Close` | `C` | Closes existing position |
| `CloseOpen` | `C;O` | Same-day round trip (close then open) |
| `Unknown` | `*` | Fallback |

---

### OrderType

Order type for the execution.

**Used by**: `Trade`, `Order`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Market` | `MKT` | Market order - execute at best available price |
| `Limit` | `LMT` | Limit order - execute at specified price or better |
| `Stop` | `STP` | Stop order - becomes market when stop price hit |
| `StopLimit` | `STP LMT` | Stop limit - becomes limit when stop price hit |
| `MarketOnClose` | `MOC` | Execute at closing price |
| `LimitOnClose` | `LOC` | Limit order for closing auction |
| `MarketIfTouched` | `MIT` | Becomes market when trigger price touched |
| `LimitIfTouched` | `LIT` | Becomes limit when trigger price touched |
| `TrailingStop` | `TRAIL` | Stop that trails the market price |
| `TrailingLimit` | `TRAIL LMT` | Trailing stop with limit |
| `MidPrice` | `MIDPX` | Execute at midpoint of bid-ask |
| `Relative` | `REL` | Pegged to NBBO |
| `Multiple` | `MULTIPLE` | Complex order with multiple types |
| `Unknown` | `*` | Fallback |

---

### PutCall

Option type indicator.

**Used by**: `Trade`, `Position`, `CorporateAction`, `SecurityInfo`, `OptionEAE`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Put` | `P` | Put option - right to sell at strike |
| `Call` | `C` | Call option - right to buy at strike |
| `Unknown` | `*` | Fallback |

---

### LongShort

Position side indicator.

**Used by**: `Position`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Long` | `Long` | Long position (own the security) |
| `Short` | `Short` | Short position (borrowed and sold) |
| `Unknown` | `*` | Fallback |

---

### TradeType

Classification of how the trade was executed.

**Used by**: `Trade`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `ExchTrade` | `ExchTrade` | Standard exchange execution |
| `BookTrade` | `BookTrade` | Internal book trade |
| `DvpTrade` | `DvpTrade` | Delivery vs Payment |
| `FracShare` | `FracShare` | Fractional share execution |
| `FracShareCancel` | `FracShareCancel` | Cancelled fractional share |
| `Adjustment` | `Adjustment` | Manual adjustment |
| `TradeCorrect` | `TradeCorrect` | Trade correction |
| `TradeCancel` | `TradeCancel` | Trade cancellation |
| `IBKRTrade` | `IBKRTrade` | IBKR internal trade |
| `Unknown` | `*` | Fallback |

---

### CashAction

Type of cash transaction.

**Used by**: `CashTransaction`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `DepositsWithdrawals` | `Deposits & Withdrawals` | Cash deposits or withdrawals |
| `Dividends` | `Dividends` | Dividend payments |
| `WithholdingTax` | `Withholding Tax` | Tax withheld on dividends/interest |
| `BrokerInterestPaid` | `Broker Interest Paid` | Interest paid to broker (margin) |
| `BrokerInterestReceived` | `Broker Interest Received` | Interest received on cash balance |
| `BondInterestReceived` | `Bond Interest Received` | Coupon payments received |
| `BondInterestPaid` | `Bond Interest Paid` | Bond interest paid (short) |
| `BondInterest` | `Bond Interest` | Generic bond interest |
| `PaymentInLieuOfDividends` | `Payment In Lieu Of Dividends` | PIL for lent securities |
| `OtherFees` | `Other Fees` | Miscellaneous fees |
| `CommissionAdjustments` | `Commission Adjustments` | Commission corrections |
| `AdvisorFees` | `Advisor Fees` | Advisory/management fees |
| `CashReceipts` | `Cash Receipts` | Cash receipts |
| `Fees` | `Fees` | Generic fees |
| `Unknown` | `*` | Fallback |

---

### Reorg (Corporate Action Type)

Type of corporate action reorganization.

**Used by**: `CorporateAction`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `StockSplit` | `Stock Split` | Forward stock split |
| `ForwardSplitIssue` | `Forward Split (Issue)` | Split share issuance |
| `ForwardSplit` | `Forward Split` | Forward split |
| `ReverseSplit` | `Reverse Split` | Reverse stock split |
| `Merger` | `Merger` | Company merger |
| `Spinoff` | `Spinoff` | Corporate spinoff |
| `ContractSpinoff` | `Contract Spinoff` | Derivative contract spinoff |
| `StockDividend` | `Stock Dividend` | Dividend paid in shares |
| `CashDividend` | `Cash Dividend` | Regular cash dividend |
| `ChoiceDividend` | `Choice Dividend` | Dividend with cash/stock choice |
| `ChoiceDivDelivery` | `Choice Dividend (Delivery)` | Choice dividend delivery |
| `ChoiceDivIssue` | `Choice Dividend (Issue)` | Choice dividend issuance |
| `DivRightsIssue` | `Dividend Rights Issue` | Dividend rights |
| `ExpiredDivRight` | `Expired Dividend Right` | Expired dividend right |
| `Delisted` | `Delisted` | Security delisted |
| `DelistWorthless` | `Delist (Worthless)` | Delisted with zero value |
| `NameChange` | `Name Change` | Company name change |
| `SymbolChange` | `Symbol Change` | Ticker symbol change |
| `IssueChange` | `Issue Change` | Security issue change |
| `BondConversion` | `Bond Conversion` | Convertible bond conversion |
| `BondMaturity` | `Bond Maturity` | Bond reached maturity |
| `TBillMaturity` | `T-Bill Maturity` | T-Bill maturity |
| `ConvertibleIssue` | `Convertible Issue` | Convertible security issued |
| `CouponPayment` | `Coupon Payment` | Bond coupon payment |
| `ContractConsolidation` | `Contract Consolidation` | Derivative consolidation |
| `ContractSplit` | `Contract Split` | Derivative split |
| `CfdTermination` | `CFD Termination` | CFD contract ended |
| `FeeAllocation` | `Fee Allocation` | Fee allocation |
| `RightsIssue` | `Rights Issue` | Rights offering |
| `SubscribeRights` | `Subscribe Rights` | Rights subscription |
| `Tender` | `Tender` | Tender offer |
| `TenderIssue` | `Tender (Issue)` | Tender issuance |
| `ProxyVote` | `Proxy Vote` | Proxy voting |
| `GenericVoluntary` | `Generic Voluntary` | Other voluntary action |
| `AssetPurchase` | `Asset Purchase` | Asset acquisition |
| `PurchaseIssue` | `Purchase (Issue)` | Purchase issuance |
| `Unknown` | `*` | Fallback |

---

### OptionAction

Option-specific event type.

**Used by**: `OptionEAE`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Assignment` | `Assignment` | Short option assigned |
| `Exercise` | `Exercise` | Long option exercised |
| `Expiration` | `Expiration` | Option expired |
| `Expire` | `Expire` | Option expired (alternate) |
| `CashSettlement` | `Cash Settlement` | Cash-settled expiration |
| `Buy` | `Buy` | Option purchase |
| `Sell` | `Sell` | Option sale |
| `Unknown` | `*` | Fallback |

---

### TransferType

Method of security transfer.

**Used by**: `Transfer`, `UnsettledTransfer`, `TradeTransfer`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `ACATS` | `ACATS` | Automated Customer Account Transfer Service |
| `ATON` | `ATON` | Account Transfer on Request |
| `FOP` | `FOP` | Free of Payment (no cash) |
| `INTERNAL` | `INTERNAL` | Internal IB transfer |
| `DVP` | `DVP` | Delivery vs Payment |
| `Unknown` | `*` | Fallback |

---

### Code (Transaction Codes)

Transaction classification codes providing critical context for tax reporting and trade classification. These appear in `notes` fields and can be combined (e.g., "C;W" for closing + wash sale).

**Used by**: `Trade.notes`, `Position.code`, `CashTransaction.code`, `CorporateAction.code`

| Variant | XML | Full Name | Description | Tax Impact |
|---------|-----|-----------|-------------|------------|
| `A` | `A` | Assignment | Option assignment | Triggers stock delivery |
| `Adj` | `Adj` | Adjustment | Manual adjustment | Affects cost basis |
| `Al` | `Al` | Allocation | Trade allocation to sub-account | Master/sub allocation |
| `Ae` | `Ae` | Auto Exercise | Automatic exercise (dividend-related) | Exercise before ex-div |
| `Af` | `Af` | Auto FX | AutoFX currency conversion | FX for settlement |
| `Aw` | `Aw` | Away Trade | Trade executed away from IB | Third-party execution |
| `B` | `B` | Buy-In | Forced purchase to cover failed delivery | Forced short cover |
| `Bo` | `Bo` | Borrow | Securities borrowing fee | Lending charge |
| `Ca` | `Ca` | Cancellation | Trade cancelled/busted | Trade reversed |
| `C` | `C` | Closing | Closing trade | Reduces position |
| `Cd` | `Cd` | Cash Delivery | Cash delivery for exercise | Cash vs physical |
| `Cp` | `Cp` | Complex Position | Complex/combo position | Multi-leg strategy |
| `Cr` | `Cr` | Correction | Trade correction | Amended execution |
| `Cs` | `Cs` | Crossing | Internal IB cross | Matched internally |
| `D` | `D` | Dual Agent | IB dual agent capacity | Disclosed dual role |
| `Et` | `Et` | ETF | ETF creation/redemption | In-kind basket |
| `Ex` | `Ex` | Expired | From expired position | Option/warrant expiry |
| `O` | `O` | Exercise | Option exercise | Long option exercised |
| `G` | `G` | Guaranteed | Guaranteed account segment | Special margin |
| `Hc` | `Hc` | Highest Cost | Highest cost tax lot | Tax lot selection |
| `Hi` | `Hi` | HF Investment | Hedge fund investment | Fund subscription |
| `Hr` | `Hr` | HF Redemption | Hedge fund redemption | Fund redemption |
| `I` | `I` | Internal | Internal transfer | Between IB accounts |
| `Ia` | `Ia` | Affiliate | Affiliate execution | Related party trade |
| `Iv` | `Iv` | Investor | Investment from investor | Capital contribution |
| `L` | `L` | Margin Violation | Liquidation (margin) | Forced liquidation |
| `Li` | `Li` | LIFO | LIFO tax lot | Tax lot selection |
| `Ln` | `Ln` | Loan | Securities lending income | Lending income |
| `Lt` | `Lt` | Long-Term | Long-term gain/loss | Holding > 1 year |
| `M` | `M` | Manual | Manual IB entry | Manual adjustment |
| `Ml` | `Ml` | Max Loss | Maximize losses | Tax optimization |
| `Mn` | `Mn` | Min LT Gain | Minimize long-term gain | Tax optimization |
| `Ms` | `Ms` | Max ST Gain | Maximize short-term gain | Tax optimization |
| `Mi` | `Mi` | Min ST Gain | Minimize short-term gain | Tax optimization |
| `Mx` | `Mx` | Manual Exercise | Manual exercise | Discretionary exercise |
| `P` | `P` | Opening | Opening trade | New position |
| `Pt` | `Pt` | Partial | Partial execution | Partial fill |
| `Fr` | `Fr` | Frac Riskless | Fractional riskless principal | Frac share method |
| `Fp` | `Fp` | Frac Principal | Fractional principal | Frac share method |
| `Pi` | `Pi` | Price Improvement | Better than quoted | Price improvement |
| `Pa` | `Pa` | Post Accrual | Accrual posting | Accrual entry |
| `Pr` | `Pr` | Principal | IB principal execution | Principal trade |
| `Re` | `Re` | Reinvestment | Dividend reinvestment | DRIP |
| `Rd` | `Rd` | Redemption | Capital distribution | Fund redemption |
| `R` | `R` | Reopen | Position reopened | Wash sale reopen |
| `Rv` | `Rv` | Reverse | Accrual reversal | Accounting reversal |
| `Ri` | `Ri` | Reimbursement | Fee refund | Expense refund |
| `Si` | `Si` | Solicited IB | IB solicited order | IB-initiated |
| `Sp` | `Sp` | Specific Lot | Specific tax lot | Tax lot selection |
| `So` | `So` | Solicited Other | Third-party solicited | Broker-solicited |
| `Ss` | `Ss` | Short Settlement | T+0 or T+1 settlement | Accelerated settle |
| `St` | `St` | Short-Term | Short-term gain/loss | Holding <= 1 year |
| `Sy` | `Sy` | Stock Yield | Stock yield eligible | Lending eligible |
| `T` | `T` | Transfer | Position transfer | Transfer |
| `W` | `W` | Wash Sale | Wash sale | **Loss disallowed** |
| `Unknown` | `*` | Unknown | Unrecognized code | Fallback |

**Example**: A trade with `notes="C;P;W"` indicates: Closing trade, Opening new position (same-day round trip), with Wash Sale deferral.

---

### ToFrom

Direction indicator for transfers.

**Used by**: `Transfer`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `To` | `To` | Transfer to another account |
| `From` | `From` | Transfer from another account |
| `Unknown` | `*` | Fallback |

---

### InOut

Flow direction indicator.

**Used by**: `Transfer`, `TradeTransfer`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `IN` | `IN` | Incoming |
| `OUT` | `OUT` | Outgoing |
| `Unknown` | `*` | Fallback |

---

### DeliveredReceived

Delivery status indicator.

**Used by**: `Transfer`, `TradeTransfer`

| Variant | XML Value | Description |
|---------|-----------|-------------|
| `Delivered` | `Delivered` | Security delivered out |
| `Received` | `Received` | Security received in |
| `Unknown` | `*` | Fallback |

---

## Struct Reference

### Trade (77 fields)

The most complex type, representing a single trade execution.

**XML Element**: `<Trade>` within `<Trades>`

#### Account Identification (2 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `account_id` | `Option<String>` | `accountId` | IB account number |
| `acct_alias` | `Option<String>` | `acctAlias` | Account alias |

#### Security Identification (11 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `symbol` | `Option<String>` | `symbol` | Ticker symbol |
| `description` | `Option<String>` | `description` | Security description |
| `conid` | `Option<String>` | `conid` | IB contract ID |
| `asset_category` | `Option<AssetCategory>` | `assetCategory` | Security type |
| `cusip` | `Option<String>` | `cusip` | CUSIP (US) |
| `isin` | `Option<String>` | `isin` | ISIN (international) |
| `figi` | `Option<String>` | `figi` | FIGI identifier |
| `security_id` | `Option<String>` | `securityID` | Generic security ID |
| `security_id_type` | `Option<String>` | `securityIDType` | Type of security_id |
| `issuer` | `Option<String>` | `issuer` | Security issuer |
| `listing_exchange` | `Option<String>` | `listingExchange` | Primary listing exchange |

#### Derivative Fields (7 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `multiplier` | `Option<Decimal>` | `multiplier` | Contract multiplier (100 for options) |
| `strike` | `Option<Decimal>` | `strike` | Strike price |
| `expiry` | `Option<NaiveDate>` | `expiry` | Expiration date |
| `put_call` | `Option<PutCall>` | `putCall` | Put or Call |
| `underlying_conid` | `Option<String>` | `underlyingConid` | Underlying contract ID |
| `underlying_symbol` | `Option<String>` | `underlyingSymbol` | Underlying ticker |
| `underlying_security_id` | `Option<String>` | `underlyingSecurityID` | Underlying security ID |

#### Trade Execution (12 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `trade_date` | `Option<NaiveDate>` | `tradeDate` | Execution date |
| `trade_time` | `Option<NaiveTime>` | `tradeTime` | Execution time |
| `date_time` | `Option<NaiveDateTime>` | `dateTime` | Combined date/time |
| `settle_date` | `Option<NaiveDate>` | `settleDateTarget` | Settlement date |
| `buy_sell` | `Option<BuySell>` | `buySell` | Trade direction |
| `open_close` | `Option<OpenClose>` | `openCloseIndicator` | Open/close indicator |
| `transaction_type` | `Option<String>` | `transactionType` | Trade type |
| `order_type` | `Option<OrderType>` | `orderType` | Order type |
| `trade_id` | `Option<String>` | `tradeID` | Trade ID |
| `transaction_id` | `Option<String>` | `transactionID` | Transaction ID |
| `ib_order_id` | `Option<String>` | `ibOrderID` | IB order ID |
| `exchange` | `Option<String>` | `exchange` | Execution venue |

#### Quantities & Prices (5 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `quantity` | `Option<Decimal>` | `quantity` | Shares/contracts (negative for sells) |
| `price` | `Option<Decimal>` | `tradePrice` | Trade price per unit |
| `proceeds` | `Option<Decimal>` | `proceeds` | Trade proceeds (negative for buys) |
| `cost` | `Option<Decimal>` | `cost` | Cost basis |
| `trade_money` | `Option<Decimal>` | `tradeMoney` | Total trade value |

#### Fees & Taxes (4 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `commission` | `Option<Decimal>` | `ibCommission` | IB commission (usually negative) |
| `commission_currency` | `Option<String>` | `ibCommissionCurrency` | Commission currency |
| `taxes` | `Option<Decimal>` | `taxes` | Taxes paid |
| `net_cash` | `Option<Decimal>` | `netCash` | Net cash impact |

#### P&L Fields (4 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `fifo_pnl_realized` | `Option<Decimal>` | `fifoPnlRealized` | Realized P&L (FIFO) |
| `mtm_pnl` | `Option<Decimal>` | `mtmPnl` | Mark-to-market P&L |
| `fx_pnl` | `Option<Decimal>` | `fxPnl` | FX-related P&L |
| `capital_gains_pnl` | `Option<Decimal>` | `capitalGainsPnl` | Capital gains P&L |

#### Tax Lot Tracking (8 fields)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `orig_trade_date` | `Option<NaiveDate>` | `origTradeDate` | Original purchase date |
| `orig_trade_price` | `Option<Decimal>` | `origTradePrice` | Original purchase price |
| `orig_trade_id` | `Option<String>` | `origTradeID` | Original trade ID |
| `open_date_time` | `Option<NaiveDateTime>` | `openDateTime` | Position open timestamp |
| `holding_period_date_time` | `Option<NaiveDateTime>` | `holdingPeriodDateTime` | For LT/ST determination |
| `when_realized` | `Option<NaiveDateTime>` | `whenRealized` | When P&L realized |
| `when_reopened` | `Option<NaiveDateTime>` | `whenReopened` | Wash sale reopen date |
| `level_of_detail` | `Option<String>` | `levelOfDetail` | Detail level |

#### Notes (1 field)

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `notes` | `Option<String>` | `notes` | Transaction codes (e.g., "C;W") |

---

### Position (58 fields)

Open position in the portfolio.

**XML Element**: `<OpenPosition>` within `<OpenPositions>`

#### Key Fields

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `symbol` | `Option<String>` | `symbol` | Ticker symbol |
| `conid` | `Option<String>` | `conid` | IB contract ID |
| `asset_category` | `Option<AssetCategory>` | `assetCategory` | Security type |
| `side` | `Option<LongShort>` | `side` | Long or Short |
| `quantity` | `Option<Decimal>` | `position` | Position size |
| `mark_price` | `Option<Decimal>` | `markPrice` | Current market price |
| `position_value` | `Option<Decimal>` | `positionValue` | Market value |
| `cost_basis_price` | `Option<Decimal>` | `costBasisPrice` | Average cost per unit |
| `cost_basis_money` | `Option<Decimal>` | `costBasisMoney` | Total cost basis |
| `fifo_pnl_unrealized` | `Option<Decimal>` | `fifoPnlUnrealized` | Unrealized P&L |
| `percent_of_nav` | `Option<Decimal>` | `percentOfNAV` | % of portfolio |

---

### CashTransaction (48 fields)

Cash activity (dividends, deposits, withdrawals, fees, etc.).

**XML Element**: `<CashTransaction>` within `<CashTransactions>`

#### Key Fields

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `type_` | `Option<CashAction>` | `type` | Transaction type |
| `amount` | `Option<Decimal>` | `amount` | Cash amount |
| `currency` | `Option<String>` | `currency` | Currency code |
| `description` | `Option<String>` | `description` | Description |
| `date_time` | `Option<NaiveDateTime>` | `dateTime` | Transaction time |
| `symbol` | `Option<String>` | `symbol` | Related security |
| `conid` | `Option<String>` | `conid` | Related contract ID |
| `fx_rate_to_base` | `Option<Decimal>` | `fxRateToBase` | FX rate to base currency |

---

### CorporateAction (57 fields)

Corporate action (splits, dividends, mergers, etc.).

**XML Element**: `<CorporateAction>` within `<CorporateActions>`

#### Key Fields

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `type_` | `Option<Reorg>` | `type` | Action type |
| `symbol` | `Option<String>` | `symbol` | Security symbol |
| `description` | `Option<String>` | `description` | Action description |
| `quantity` | `Option<Decimal>` | `quantity` | Shares affected |
| `amount` | `Option<Decimal>` | `amount` | Cash amount |
| `proceeds` | `Option<Decimal>` | `proceeds` | Proceeds |
| `date_time` | `Option<NaiveDateTime>` | `dateTime` | Action date |
| `fifo_pnl_realized` | `Option<Decimal>` | `fifoPnlRealized` | Realized P&L |

---

### SecurityInfo (29 fields)

Security reference data.

**XML Element**: `<SecurityInfo>` within `<SecuritiesInfo>`

#### Key Fields

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `symbol` | `Option<String>` | `symbol` | Ticker symbol |
| `conid` | `Option<String>` | `conid` | IB contract ID |
| `description` | `Option<String>` | `description` | Security name |
| `asset_category` | `Option<AssetCategory>` | `assetCategory` | Security type |
| `cusip` | `Option<String>` | `cusip` | CUSIP |
| `isin` | `Option<String>` | `isin` | ISIN |
| `figi` | `Option<String>` | `figi` | FIGI |
| `multiplier` | `Option<Decimal>` | `multiplier` | Contract multiplier |
| `strike` | `Option<Decimal>` | `strike` | Strike price |
| `expiry` | `Option<NaiveDate>` | `expiry` | Expiration date |
| `put_call` | `Option<PutCall>` | `putCall` | Put or Call |

---

### ConversionRate (4 fields)

FX conversion rate for multi-currency reporting.

**XML Element**: `<ConversionRate>` within `<ConversionRates>`

| Field | Type | XML Attr | Description |
|-------|------|----------|-------------|
| `report_date` | `NaiveDate` | `reportDate` | Rate date |
| `from_currency` | `String` | `fromCurrency` | Source currency |
| `to_currency` | `String` | `toCurrency` | Target currency |
| `rate` | `Decimal` | `rate` | Conversion rate |

---

## Gaps Analysis

### Unparsed XML Sections (23 sections)

These sections appear in FLEX statements but are currently ignored:

| Section | XML Element | Priority | Description |
|---------|-------------|----------|-------------|
| Complex Positions | `ComplexPositions` | Medium | Multi-leg option spreads |
| FX Positions | `FxPositions` | Medium | Forex position details |
| Net Stock Positions | `NetStockPositions` | Low | Stock borrowing/lending summary |
| CFD Charges | `CFDCharges` | Low | CFD-specific fees |
| Commission Credits | `CommissionCredits` | Low | Rebates and credits |
| FDIC Deposits | `FdicInsuredDepositsByBank` | Low | Bank deposit insurance info |
| HK IPO Open | `HKIPOOpenSubscriptions` | Very Low | Hong Kong IPO subscriptions |
| HK IPO Activity | `HKIPOSubscriptionActivity` | Very Low | HK IPO activity |
| IBG Notes | `IBGNoteTransactions` | Very Low | IB Notes products |
| Incentive Coupons | `IncentiveCouponAccruals` | Very Low | Promotional coupons |
| Mutual Fund Dividends | `MutualFundDividends` | Low | Fund distributions |
| Net Stock Summary | `NetStockPositionSummary` | Low | Borrowing summary |
| Pending Exercises | `PendingExcercises` | Medium | Upcoming option exercises |
| Routing Commissions | `RoutingCommissions` | Low | Exchange routing fees |
| SLB Collaterals | `SLBCollaterals` | Low | Lending collateral |
| SLB Open Contracts | `SLBOpenContracts` | Low | Active lending contracts |
| Soft Dollars | `SoftDollars` | Very Low | Research credits |
| Stock Grants | `StockGrantActivities` | Low | RSU/stock grants |
| Transaction Taxes | `TransactionTaxes` | Medium | Detailed tax breakdown |
| Untracked Trades | `UntrackedTrades` | Low | Unbooked trades |
| Deposits On Hold | `DepositsOnHold` | Low | Pending deposits |

---

## Implementation Checklist

### Phase 1: Enum Improvements

**Rename existing enums for clarity:**
- [x] `Code` → `TransactionCode`
- [x] `Reorg` → `CorporateActionType`
- [x] `CashAction` → `CashTransactionType`

**Rename `TransactionCode` variants (cryptic → descriptive):**
- [x] `A` → `Assignment`, `Ae` → `AutoExercise`, `Af` → `AutoFx`
- [x] `B` → `BuyIn`, `Bo` → `BorrowFee`, `C` → `Closing`, `Ca` → `Cancelled`
- [x] `D` → `DualAgent`, `Ex` → `Expired`, `G` → `Guaranteed`
- [x] `Hc` → `HighestCost`, `I` → `InternalTransfer`, `L` → `MarginLiquidation`
- [x] `Li` → `Lifo`, `Lt` → `LongTermGain`, `M` → `ManualEntry`
- [x] `O` → `Exercise`, `P` → `Opening`, `R` → `Reopen`
- [x] `St` → `ShortTermGain`, `T` → `Transfer`, `W` → `WashSale`
- [x] All remaining variants renamed (45 total variants completed)

**Create new enums:**
- [x] `LevelOfDetail` { Summary, Detail, Execution, Lot }
- [x] `SecurityIdType` { Cusip, Isin, Figi, Sedol }
- [x] `SubCategory` { Etf, Adr, Reit, Preferred, Common, ... }

**Create `DerivativeInfo` enum:**
```rust
pub enum DerivativeInfo {
    Option {
        strike: Decimal,
        expiry: NaiveDate,
        put_call: PutCall,
        underlying_symbol: String,
        underlying_conid: Option<String>,
    },
    Future {
        expiry: NaiveDate,
        underlying_symbol: String,
        underlying_conid: Option<String>,
    },
    FutureOption {
        strike: Decimal,
        expiry: NaiveDate,
        put_call: PutCall,
        underlying_symbol: String,
        underlying_conid: Option<String>,
    },
    Warrant {
        strike: Option<Decimal>,
        expiry: Option<NaiveDate>,
        underlying_symbol: Option<String>,
    },
}
```

### Phase 2: Field Type Changes

**Use existing enums:**
- [ ] `Trade.transaction_type`: `String` → `TradeType`

**Use new enums:**
- [ ] `Trade.level_of_detail`: `String` → `LevelOfDetail`
- [ ] `Position.level_of_detail`: `String` → `LevelOfDetail`
- [ ] `SecurityInfo.security_id_type`: `String` → `SecurityIdType`
- [ ] `SecurityInfo.sub_category`: `String` → `SubCategory`

**Fix primitive types:**
- [ ] `Trade.is_api_order`: `String` → `bool` (Y/N)

**Add derivative info:**
- [ ] `Trade.derivative`: `Option<DerivativeInfo>` (based on asset_category)
- [ ] `Position.derivative`: `Option<DerivativeInfo>`
- [ ] Remove flat derivative fields from Trade/Position

### Phase 3: Remove Unnecessary Option<> Wrappers

Based on analysis of 2614 trades, 3455 positions, 55 cash transactions.

**Trade - make these required (100% present in data):**
- [ ] `account_id`, `acct_alias`, `symbol`, `description`, `conid`
- [ ] `asset_category`, `buy_sell`, `currency`, `exchange`
- [ ] `trade_date`, `settle_date`, `date_time`, `quantity`, `price`, `proceeds`, `cost`
- [ ] `commission`, `commission_currency`, `taxes`, `net_cash`
- [ ] `trade_id`, `transaction_id`, `ib_order_id`, `transaction_type`
- [ ] `multiplier`, `fx_rate_to_base`, `fifo_pnl_realized`, `mtm_pnl`
- [ ] `level_of_detail`, `report_date`, `is_api_order`

**Position - make these required (100% present):**
- [ ] `account_id`, `acct_alias`, `symbol`, `description`, `conid`
- [ ] `asset_category`, `currency`, `side`
- [ ] `position`, `mark_price`, `position_value`, `cost_basis_price`, `cost_basis_money`
- [ ] `fifo_pnl_unrealized`, `open_price`, `multiplier`, `fx_rate_to_base`
- [ ] `level_of_detail`, `report_date`, `figi`, `listing_exchange`

**CashTransaction - make these required (100% present):**
- [ ] `account_id`, `acct_alias`, `type_`, `description`, `amount`, `currency`
- [ ] `date_time`, `settle_date`, `report_date`, `transaction_id`
- [ ] `multiplier`, `fx_rate_to_base`, `level_of_detail`

**Keep as Option<> (conditional/rare):**
- `cusip`, `isin`, `figi` - security identifiers (70-99%)
- `notes`, `order_type`, `brokerage_order_id` - (80-93%)
- `holding_period_date_time`, `when_realized`, `when_reopened` - tax lot (0%)

**Keep as String (unbounded values):**
- `exchange`, `listing_exchange` - hundreds of venues
- `currency` - standard 3-letter ISO codes
- `notes` - semicolon-separated transaction codes

### Phase 4: Documentation
- [ ] Doc comments on all `TransactionCode` variants
- [ ] Doc comments on all `CorporateActionType` variants
- [ ] Examples in struct-level documentation

---

## References

- [Activity FLEX Query Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm)
- [FLEX Queries Guide](https://www.ibkrguides.com/orgportal/performanceandstatements/flex.htm)
- [FLEX Web Service API](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)
- [csingley/ibflex Python Library](https://github.com/csingley/ibflex)

---

*This document is the authoritative reference for IB FLEX types in this library.*
