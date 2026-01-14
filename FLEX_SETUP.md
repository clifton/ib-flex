# IBKR Flex Query Setup Guide

This guide walks you through setting up Interactive Brokers (IBKR) Flex Queries for comprehensive daily portfolio data capture. The data can be used for transaction cost analysis, risk management, position tracking, P&L reporting, and more.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Understanding Flex Query Types](#understanding-flex-query-types)
3. [Creating an Activity Flex Query](#creating-an-activity-flex-query)
4. [Recommended Sections and Fields](#recommended-sections-and-fields)
5. [Configuration Settings](#configuration-settings)
6. [Setting Up Flex Web Service (API Access)](#setting-up-flex-web-service-api-access)
7. [Using the ib-flex Library](#using-the-ib-flex-library)
8. [Daily Data Feed Workflow](#daily-data-feed-workflow)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before setting up Flex Queries, ensure you have:

- An active Interactive Brokers account
- Access to the IBKR Client Portal (https://portal.interactivebrokers.com)
- Basic understanding of portfolio data requirements

---

## Understanding Flex Query Types

IBKR offers two types of Flex Queries:

### Activity Flex Query (Recommended for Daily Feed)

- **Purpose**: End-of-day snapshot of all portfolio activity
- **Data**: Trades, positions, cash flows, corporate actions, P&L
- **Timing**: Updated after market close
- **Best for**: Daily portfolio reconciliation, historical analysis, tax reporting

### Trade Confirmation Flex Query

- **Purpose**: Near real-time trade confirmations
- **Data**: Trade executions only
- **Timing**: Available ~10 minutes after execution
- **Best for**: Intraday monitoring, execution quality analysis

**For comprehensive portfolio management, we recommend creating an Activity Flex Query with all relevant sections enabled.**

---

## Creating an Activity Flex Query

### Step 1: Navigate to Flex Queries

1. Log in to the [IBKR Client Portal](https://portal.interactivebrokers.com)
2. Click the menu icon (☰) in the top left
3. Navigate to **Performance & Reports** → **Flex Queries**

<!-- Screenshot placeholder: flex_queries_navigation.png -->

### Step 2: Create New Activity Flex Query

1. In the **Activity Flex Query** section, click the **+** (plus) icon
2. Enter a descriptive name for your query (e.g., "Daily Portfolio Data Feed")

<!-- Screenshot placeholder: create_new_query.png -->

### Step 3: Select Sections

Click on each section you want to include. For comprehensive coverage, enable **all sections** listed below.

<img width="1044" height="1181" alt="image" src="https://github.com/user-attachments/assets/8a07870d-0549-4b0f-8b72-c65d4c30b469" />

### Step 4: Configure Fields for Each Section

For each section, a configuration window will appear. Select **all fields** or the recommended fields listed in the [Recommended Sections and Fields](#recommended-sections-and-fields) section.

<!-- Screenshot placeholder: configure_fields.png -->

### Step 5: Set Delivery Configuration

- **Accounts**: Select the account(s) to include
- **Format**: Select **XML** (required for parsing with ib-flex)
- **Period**: Select **Last Business Day** for daily feeds
- **Breakout By Day**: Select **Yes**
- **Include Currency Rates**: Select **Yes**

<img width="700" height="828" alt="image" src="https://github.com/user-attachments/assets/d3f18e76-6364-49d6-8ede-4c4001d9d2d0" />

### Step 6: Set General Configuration

Configure date/time formats as specified in [Configuration Settings](#configuration-settings).

### Step 7: Save the Query

1. Click **Continue** to review your settings
2. Click **Create** to save the query
3. **Note the Query ID** displayed after creation (needed for API access)

<!-- Screenshot placeholder: query_created.png -->

---

## Recommended Sections and Fields

This configuration is optimized for **fund portfolio management** with support for:
- Portfolio tracking and reconciliation
- Transaction cost analysis (TCA)
- Short selling / securities borrowing
- Options trading

We recommend **22 sections** that provide comprehensive coverage without unnecessary bulk.

### Important: Models and Realized P&L

> **Warning**: If you include the "Model" field (for strategy/sleeve tracking), IB disables native Realized P&L sections. This is an IB limitation.
>
> **Workaround**: Calculate realized P&L from the `fifoPnlRealized` field on individual trades in the Trades section.

### Recommended 22 Sections

#### Core Portfolio (15 sections)

| # | Section | Purpose | Key Fields |
|---|---------|---------|------------|
| 1 | **Account Information** | Account identity | ClientAccountID, AccountAlias, CurrencyPrimary, Name, AccountType |
| 2 | **Open Positions** | Current holdings | Symbol, Quantity, MarkPrice, PositionValue, CostBasisMoney, FifoPnlUnrealized |
| 3 | **Trades** | Execution history | TradeID, Symbol, DateTime, Quantity, Price, Proceeds, Commission, FifoPnlRealized |
| 4 | **Cash Transactions** | Dividends, fees, interest | Type, Symbol, Amount, DateTime, Description |
| 5 | **Cash Report** | Cash flow summary | StartingCash, EndingCash, Dividends, Commissions, Deposits, Withdrawals |
| 6 | **Change in NAV** | Performance tracking | StartingValue, EndingValue, Mtm, Realized, ChangeInUnrealized, TWR |
| 7 | **Corporate Actions** | Splits, mergers, spinoffs | Type, Symbol, Quantity, Amount, Proceeds, ActionDescription |
| 8 | **Financial Instrument Information** | Security master data | Conid, Symbol, CUSIP, ISIN, FIGI, AssetClass, Multiplier, Strike, Expiry |
| 9 | **Open Dividend Accruals** | Pending dividends | Symbol, ExDate, PayDate, Quantity, GrossRate, GrossAmount, NetAmount |
| 10 | **Interest Accruals** | Interest tracking | StartingAccrualBalance, InterestAccrued, EndingAccrualBalance |
| 11 | **Transfers** | Asset movements | Type, Direction, Symbol, Quantity, TransferPrice, DateTime |
| 12 | **Equity Summary by Report Date in Base** | EOD equity breakdown | Total, Cash, Stock, Options, Commodities, Bonds, Funds, Notes, InterestAccruals, DividendAccruals |
| 13 | **Net Asset Value (NAV) in Base** | Absolute daily NAV | NAV, CashBalances, StockValue, OptionsValue, CommoditiesValue, BondsValue, FundsValue |
| 14 | **Mark-to-Market Performance Summary in Base** | P&L attribution by asset class | AssetCategory, Mtm, Realized, Unrealized, Dividends, Interest, ChangeInPrice |
| 15 | **Realized and Unrealized Performance Summary in Base** | P&L checksum (may be empty with Models) | AssetCategory, Realized, Unrealized, Total, CostBasis, GainLoss |

> **Note on Margin Monitoring**: The Equity Summary section provides EOD equity breakdown by asset class, useful for tracking portfolio composition. However, **real-time margin requirements** (initial margin, maintenance margin, excess liquidity) are not available in FLEX reports - use the IB API for margin monitoring.

> **Reconciliation Strategy**: Use `Yesterday's NAV + Change in NAV = Today's NAV` as a daily sanity check to prevent drift from rounding errors in calculated P&L.

> **Note on Realized/Unrealized Performance**: This section may return empty if you use IB Models. Keep it enabled as a checksum - costs nothing to include.

#### Transaction Cost Analysis (3 sections)

| # | Section | Purpose | Key Fields |
|---|---------|---------|------------|
| 16 | **Commission Details** | Fee breakdown | BrokerExecutionCharge, BrokerClearingCharge, ThirdPartyExecutionCharge, RegFINRATradingActivityFee, RegSection31TransactionFee |
| 17 | **Transaction Fees** | Taxes and fees | TaxDescription, TaxAmount, TradeID |
| 18 | **Routing Commissions** | Venue analysis | ExecutionExchange, LowestFeeExchange, RoutingFee, ExchangeFee, CreditForLowestExchangeFee |

#### Short Selling (3 sections)

| # | Section | Purpose | Key Fields |
|---|---------|---------|------------|
| 19 | **Borrow Fees Details** | Daily borrow rates | Symbol, Quantity, BorrowFeeRate, BorrowFee, Value |
| 20 | **Securities Borrowed/Lent Fee Details** | Fee breakdown | FeeRate%, MarketFeeRate%, CollateralAmount, NetLendFee, CarryCharge |
| 21 | **Securities Borrowed/Lent Activity** | Borrow activity log | ActivityDescription, Type, Quantity, CollateralAmount, MarkQuantity |

#### Options (1 section)

| # | Section | Purpose | Key Fields |
|---|---------|---------|------------|
| 22 | **Option Exercises, Assignments and Expirations** | Options lifecycle | TransactionType, Symbol, Strike, Expiry, Quantity, Proceeds, RealizedPnl |

### Sections to Skip

These sections are unnecessary for most fund use cases:

| Section | Skip Unless... |
|---------|----------------|
| CFD Charges | Trading CFDs |
| Complex Positions | Multi-leg option strategies needing aggregate view |
| Debit Card Activity | Fund uses IB debit card |
| Deposits on Hold | Need pending deposit visibility |
| FDIC-Insured Deposits by Bank | Need bank sweep details |
| Forex Balances / P&L Details | Active FX trading as profit center |
| Grant Activity | Stock compensation plans |
| HK IPO Subscriptions | Trading Hong Kong IPOs |
| IBG Notes | Using IB notes product |
| Incentive Coupon Accrual | Very specialized |
| Mark-to-Market Performance Summary | Redundant with trade-level MTM |
| Month & Year to Date Performance | Disabled with Models; use Change in NAV |
| Mutual Fund Dividend Details | Holding mutual funds |
| Net Stock Position Summary | Detailed short position reconciliation |
| Prior Period Positions | Historical position reconciliation |
| Realized/Unrealized Performance | Disabled with Models |
| Sales Tax Details | International tax tracking |
| Securities Collateral at IBSS | Securities lending collateral |
| Soft Dollar Activity | Soft dollar arrangements |
| Statement of Funds | Redundant with Cash Report + Trades |
| Unbooked Trades | Trade settlement issues |
| Unsettled Transfers | Pending ACATS transfers |
| Change in Dividend Accruals | Redundant with Open Dividend Accruals |
| Change in Position Value Summary | Nice but adds bulk |

### Detailed Field Recommendations by Section

For each section, select **all fields** unless noted. Key fields are highlighted below.

#### Account Information (Trim Address Fields)

**Keep:**
- `ClientAccountID`, `AccountAlias`, `Model`
- `CurrencyPrimary`, `Name`, `AccountType`, `CustomerType`
- `AccountCapabilities`, `TradingPermissions`
- `DateOpened`, `DateFunded`, `DateClosed`, `LastTradedDate`
- `MasterName`, `IBEntity`, `PrimaryEmail`

**Optional (skip to reduce bulk):**
- Street address fields (Street, Street2, City, State, Country, PostalCode)
- Residential address fields
- AccountRepName, AccountRepPhone

#### Trades Section

**Identification:**
- `transactionID` - Unique identifier (for idempotency)
- `tradeID`, `IBOrderID`, `IBExecID` - Cross-reference IDs
- `conid` - IB contract ID

**Security Details:**
- `symbol`, `description`, `assetCategory`
- `cusip`, `isin`, `figi` - External identifiers
- `underlyingSymbol`, `underlyingConid` - For derivatives

**Derivative Details:**
- `strike`, `expiry`, `putCall`, `multiplier`

**Execution Details (critical for TCA):**
- `tradeDate`, `dateTime`, `orderTime` - Timing analysis
- `buySell`, `openCloseIndicator`
- `quantity`, `tradePrice`, `tradeMoney`, `proceeds`
- `orderType` - LMT, MKT, STP, etc.
- `exchange`, `listingExchange` - Venue analysis

**Costs & Fees:**
- `ibCommission`, `ibCommissionCurrency`
- `taxes`, `netCash`

**P&L:**
- `fifoPnlRealized` - Realized P&L (FIFO basis)
- `mtmPnl` - Mark-to-market P&L
- `costBasis`

**Position Tracking:**
- `settleDateTarget`, `fxRateToBase`
- `origTradePrice`, `origTradeDate`, `origTradeID` - Lot tracking

#### Open Positions Section

- `symbol`, `conid`, `description`, `assetCategory`
- `quantity` (negative for short), `side`
- `markPrice`, `positionValue`
- `costBasisPrice`, `costBasisMoney`, `openPrice`
- `fifoPnlUnrealized`, `percentOfNAV`
- `strike`, `expiry`, `putCall`, `multiplier`
- `underlyingSymbol`, `underlyingConid`
- `currency`, `fxRateToBase`
- `reportDate`

#### Cash Transactions Section

- `transactionID`, `type`
- `dateTime`, `reportDate`, `settleDate`
- `amount`, `currency`, `fxRateToBase`
- `description`
- `symbol`, `conid` - Related security

#### Cash Report Section

- `startingCash`, `endingCash`, `endingSettledCash`
- `commissions`, `dividends`, `brokerInterest`
- `deposits`, `withdrawals`
- `advisorFees`, `otherFees`
- `withholdingTax`, `transactionTax`
- `netTradesSales`, `netTradesPurchases`

#### Equity Summary by Report Date in Base Section

- `reportDate` - Date of the equity snapshot
- `total` - Total equity (net liquidating value)
- `cash` - Cash and cash equivalents
- `stock` - Long/short stock value
- `options` - Options value
- `commodities` - Futures and commodities value
- `bonds` - Bond holdings value
- `funds` - Mutual fund holdings
- `notes` - IB notes
- `interestAccruals` - Accrued interest
- `dividendAccruals` - Accrued dividends

#### Net Asset Value (NAV) in Base Section

- `reportDate` - Date of NAV snapshot
- `nav` - Total net asset value
- `cashBalances` - Cash component
- `stockValue` - Equity positions value
- `optionsValue` - Options positions value
- `commoditiesValue` - Futures/commodities value
- `bondsValue` - Fixed income value
- `fundsValue` - Mutual fund value
- `interestAccruals`, `dividendAccruals`

> **Use Case**: Daily reconciliation. `Yesterday NAV + Change in NAV = Today NAV`

#### Mark-to-Market Performance Summary in Base Section

- `assetCategory` - STK, OPT, FUT, CASH, etc.
- `mtm` - Mark-to-market P&L
- `realized` - Realized P&L
- `unrealized` - Unrealized P&L
- `dividends` - Dividend income
- `interest` - Interest income
- `changeInPrice` - Price change component
- `changeInQuantity` - Quantity change component
- `fees`, `commissions`

> **Use Case**: When NAV drops 2%, identify if it was equity, options, or FX that caused it.

#### Realized and Unrealized Performance Summary in Base Section

- `assetCategory` - Asset class breakdown
- `realized` - Total realized gains/losses
- `unrealized` - Total unrealized gains/losses
- `total` - Combined P&L
- `costBasis` - Cost basis of positions
- `gainLoss` - Net gain/loss

> **Note**: May be empty if using IB Models. Keep enabled as P&L checksum.

#### Change in NAV Section

**Use "Realized & Unrealized" option if available (disabled with Models)**

- `startingValue`, `endingValue`
- `mtm`, `realized`, `changeInUnrealized`
- `dividends`, `withholdingTax`
- `interest`, `changeInInterestAccruals`
- `commissions`, `advisorFees`, `otherFees`
- `depositsWithdrawals`, `assetTransfers`
- `fxTranslation`, `twr` (time-weighted return)

#### Corporate Actions Section

- `transactionID`, `actionID`, `type`
- `symbol`, `conid`, `description`, `actionDescription`
- `quantity`, `amount`, `proceeds`, `value`
- `costBasis`, `fifoPnlRealized`, `mtmPnl`
- `reportDate`, `dateTime`

#### Financial Instrument Information Section

- `conid`, `symbol`, `description`
- `assetCategory`, `subCategory`
- `cusip`, `isin`, `figi`, `securityID`, `securityIDType`
- `listingExchange`, `currency`
- `multiplier`, `strike`, `expiry`, `putCall`
- `underlyingSymbol`, `underlyingConid`
- `maturity`, `issueDate` - For bonds

#### Commission Details Section (TCA)

- `tradeID`, `dateTime`, `symbol`
- `totalCommission`
- `brokerExecutionCharge`, `brokerClearingCharge`
- `thirdPartyExecutionCharge`, `thirdPartyClearingCharge`, `thirdPartyRegulatoryCharge`
- `regFINRATradingActivityFee`, `regSection31TransactionFee`, `regOther`

#### Routing Commissions Section (TCA)

- `tradeID`, `execID`, `symbol`
- `tradeDate`, `tradeTime`, `orderTime`
- `executionExchange`, `lowestFeeExchange`
- `routingFee`, `exchangeFee`, `creditForLowestExchangeFee`
- `quantity`, `price`, `proceeds`

#### Borrow Fees Details Section (Short Selling)

- `symbol`, `conid`, `description`
- `valueDate`, `quantity`, `price`, `value`
- `borrowFeeRate`, `borrowFee`
- `currency`, `fxRateToBase`

#### Securities Borrowed/Lent Fee Details Section (Short Selling)

- `symbol`, `conid`
- `valueDate`, `startDate`, `type`
- `quantity`, `collateralAmount`
- `feeRate%`, `fee`
- `marketFeeRate%`, `netLendFeeRate%`, `netLendFee`
- `carryCharge`, `ticketCharge`, `totalCharges`

---

## Configuration Settings

### Date Format (CRITICAL)

The `ib-flex` library supports specific date formats. **Incorrect formats will cause parsing errors.**

| Format | Example | Recommended |
|--------|---------|-------------|
| `yyyy-MM-dd` | 2026-01-13 | **Yes** (ISO-8601) |
| `yyyyMMdd` | 20260113 | **Yes** (Compact) |
| `MM/dd/yyyy` | 01/13/2026 | No (US format) |
| `dd/MM/yyyy` | 13/01/2026 | **No** (Not supported!) |

**Use `yyyy-MM-dd` or `yyyyMMdd` for best compatibility.**

### Time Format

| Format | Example | Recommended |
|--------|---------|-------------|
| `HH:mm:ss` | 14:30:00 | **Yes** |
| `HHmmss` | 143000 | **Yes** |

### Date/Time Separator

| Separator | Example | Recommended |
|-----------|---------|-------------|
| `;` (semicolon) | 2026-01-13;14:30:00 | **Yes** (Default, fastest) |
| `,` (comma) | 2026-01-13,14:30:00 | Yes |
| ` ` (space) | 2026-01-13 14:30:00 | Yes |

### Output Format

- **Always select XML** for use with the `ib-flex` library
- CSV and Text formats are not supported by the parser

### Period Options

| Period | Description | Use Case |
|--------|-------------|----------|
| Last Business Day | Previous trading day | **Daily feed (recommended)** |
| Month to Date | Current month | Monthly reconciliation |
| Year to Date | Current year | Annual analysis |
| Last 365 Days | Rolling year | Historical analysis |
| Custom | Specific date range | Ad-hoc queries |

---

## Setting Up Flex Web Service (API Access)

The Flex Web Service allows programmatic retrieval of Flex Queries without logging into the Client Portal.

### Step 1: Enable Flex Web Service

1. Navigate to **Performance & Reports** → **Flex Queries**
2. Click the **gear icon** (⚙️) next to "Flex Web Service"
3. Toggle the switch to **Enable** the Flex Web Service

<!-- Screenshot placeholder: flex_web_service_enable.png -->

### Step 2: Generate API Token

1. Click **Generate A New Token**
2. Select token expiration period (recommend: longest available)
3. Optionally restrict to specific IP address for security
4. Click **Save**

**IMPORTANT: Copy and securely store your token immediately!** It will not be shown again.

<!-- Screenshot placeholder: generate_token.png -->

### Step 3: Note Your Query ID

1. Return to the Flex Queries list
2. Find your saved query
3. Note the **Query ID** (numeric identifier)

The Query ID is displayed in the query list or when viewing query details.

<!-- Screenshot placeholder: query_id.png -->

### Security Best Practices

- Store the token in environment variables, not in code
- Use IP restriction if accessing from a fixed server
- Regenerate tokens periodically
- Never commit tokens to version control

---

## Using the ib-flex Library

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ib-flex = { version = "0.1", features = ["api-client"] }
```

### Environment Setup

Set your credentials as environment variables:

```bash
export IB_FLEX_TOKEN="your_token_here"
export IB_FLEX_QUERY_ID="123456"
```

### Basic API Usage

```rust
use ib_flex::api::FlexApiClient;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let token = std::env::var("IB_FLEX_TOKEN")?;
    let query_id = std::env::var("IB_FLEX_QUERY_ID")?;

    // Create API client
    let client = FlexApiClient::new(&token);

    // Step 1: Send request to generate statement
    println!("Requesting Flex statement...");
    let reference_code = client.send_request(&query_id)?;
    println!("Reference code: {}", reference_code);

    // Step 2: Retrieve statement with retry
    // (IB may take a few seconds to generate the report)
    println!("Fetching statement...");
    let xml = client.get_statement_with_retry(
        &reference_code,
        10,                          // max retries
        Duration::from_secs(2),      // delay between retries
    )?;

    // Step 3: Parse the XML
    let statement = ib_flex::parse_activity_flex(&xml)?;

    // Access the data
    println!("Account: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!("Trades: {}", statement.trades.items.len());
    println!("Positions: {}", statement.positions.items.len());
    println!("Cash Transactions: {}", statement.cash_transactions.items.len());

    Ok(())
}
```

### Accessing Trade Data

```rust
use ib_flex::{parse_activity_flex, AssetCategory, BuySell};
use rust_decimal::Decimal;

fn analyze_trades(xml: &str) -> Result<(), Box<dyn std::error::Error>> {
    let statement = parse_activity_flex(xml)?;

    let mut total_commission = Decimal::ZERO;
    let mut total_realized_pnl = Decimal::ZERO;

    for trade in &statement.trades.items {
        // Basic trade info
        println!(
            "{} {} {} @ {}",
            trade.symbol,
            trade.buy_sell.as_ref().map(|b| format!("{:?}", b)).unwrap_or_default(),
            trade.quantity.unwrap_or_default(),
            trade.price.unwrap_or_default()
        );

        // Accumulate commissions (for TCA)
        total_commission += trade.commission;

        // Accumulate realized P&L
        if let Some(pnl) = trade.fifo_pnl_realized {
            total_realized_pnl += pnl;
        }

        // Handle different asset types
        match trade.asset_category {
            AssetCategory::Option => {
                println!(
                    "  Option: {} {} @ {} exp {}",
                    trade.underlying_symbol.as_ref().unwrap_or(&"".to_string()),
                    trade.put_call.as_ref().map(|p| format!("{:?}", p)).unwrap_or_default(),
                    trade.strike.unwrap_or_default(),
                    trade.expiry.map(|d| d.to_string()).unwrap_or_default()
                );
            }
            AssetCategory::Future => {
                println!(
                    "  Future: exp {}",
                    trade.expiry.map(|d| d.to_string()).unwrap_or_default()
                );
            }
            _ => {}
        }
    }

    println!("\nSummary:");
    println!("  Total Commission: {}", total_commission);
    println!("  Total Realized P&L: {}", total_realized_pnl);

    Ok(())
}
```

### Accessing Position Data

```rust
use ib_flex::parse_activity_flex;
use rust_decimal::Decimal;

fn analyze_positions(xml: &str) -> Result<(), Box<dyn std::error::Error>> {
    let statement = parse_activity_flex(xml)?;

    let mut total_value = Decimal::ZERO;
    let mut total_unrealized_pnl = Decimal::ZERO;

    for position in &statement.positions.items {
        total_value += position.position_value;

        if let Some(pnl) = position.fifo_pnl_unrealized {
            total_unrealized_pnl += pnl;
        }

        println!(
            "{}: {} @ {} = {} (P&L: {:?})",
            position.symbol,
            position.quantity,
            position.mark_price,
            position.position_value,
            position.fifo_pnl_unrealized
        );
    }

    println!("\nPortfolio Summary:");
    println!("  Total Value: {}", total_value);
    println!("  Unrealized P&L: {}", total_unrealized_pnl);

    Ok(())
}
```

### Accessing Extended Data

```rust
use ib_flex::parse_activity_flex;

fn analyze_extended_data(xml: &str) -> Result<(), Box<dyn std::error::Error>> {
    let statement = parse_activity_flex(xml)?;

    // NAV Changes
    for nav in &statement.change_in_nav.items {
        println!(
            "NAV: {} -> {} (P&L: {:?})",
            nav.starting_value,
            nav.ending_value,
            nav.realized_pnl
        );
    }

    // Equity Summary
    for equity in &statement.equity_summary.items {
        println!(
            "Equity on {}: Cash={:?}, Stock={:?}, Options={:?}, Total={}",
            equity.report_date,
            equity.cash,
            equity.stock,
            equity.options,
            equity.total
        );
    }

    // Dividend Accruals
    for div in &statement.open_dividend_accruals.items {
        println!(
            "Expected dividend: {} - {} shares @ {} (ex: {}, pay: {:?})",
            div.symbol,
            div.quantity,
            div.gross_rate,
            div.ex_date,
            div.pay_date
        );
    }

    // Option Events
    for option in &statement.option_eae.items {
        println!(
            "Option {}: {} {} @ {:?}",
            option.action_type,
            option.symbol,
            option.quantity,
            option.strike
        );
    }

    Ok(())
}
```

---

## Daily Data Feed Workflow

### Recommended Daily Process

1. **Schedule**: Run after market close (e.g., 6:00 PM ET for US markets)
2. **Fetch**: Use the Flex Web Service API to retrieve the latest data
3. **Parse**: Parse the XML using `ib-flex`
4. **Store**: Save to your database or data warehouse
5. **Validate**: Check for data completeness and consistency

### Example Daily Fetch Script

```rust
use ib_flex::api::FlexApiClient;
use ib_flex::parse_activity_flex;
use std::time::Duration;
use chrono::Local;

fn daily_data_fetch() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("IB_FLEX_TOKEN")?;
    let query_id = std::env::var("IB_FLEX_QUERY_ID")?;

    let client = FlexApiClient::new(&token);

    // Fetch statement
    let reference = client.send_request(&query_id)?;
    let xml = client.get_statement_with_retry(
        &reference,
        15,  // More retries for reliability
        Duration::from_secs(3),
    )?;

    // Parse
    let statement = parse_activity_flex(&xml)?;

    // Log summary
    let now = Local::now();
    println!("[{}] Daily fetch completed", now.format("%Y-%m-%d %H:%M:%S"));
    println!("  Account: {}", statement.account_id);
    println!("  Period: {} to {}", statement.from_date, statement.to_date);
    println!("  Trades: {}", statement.trades.items.len());
    println!("  Positions: {}", statement.positions.items.len());
    println!("  Cash Transactions: {}", statement.cash_transactions.items.len());
    println!("  Corporate Actions: {}", statement.corporate_actions.items.len());

    // Save to file (or database)
    let filename = format!("flex_{}_{}.xml", statement.account_id, statement.to_date);
    std::fs::write(&filename, &xml)?;
    println!("  Saved to: {}", filename);

    Ok(())
}
```

### Data Aggregation Tips

For daily aggregation and analysis:

- **Trades**: Aggregate by symbol, asset class, or strategy
- **P&L**: Track daily realized and unrealized P&L
- **Commissions**: Sum for transaction cost analysis
- **Positions**: Snapshot for risk management
- **Cash Flows**: Track dividends, interest, fees separately

---

## Troubleshooting

### Common Issues

#### "Date format not supported"

**Cause**: Using `dd/MM/yyyy` or other unsupported date format.

**Solution**: Change to `yyyy-MM-dd` or `yyyyMMdd` in your Flex Query configuration.

#### "Statement not ready" errors

**Cause**: IB needs time to generate the statement.

**Solution**: Use `get_statement_with_retry` with adequate retries and delay:

```rust
let xml = client.get_statement_with_retry(
    &reference,
    15,                          // Increase retries
    Duration::from_secs(5),      // Increase delay
)?;
```

#### Empty sections in parsed data

**Cause**: Section not enabled in Flex Query or no data for the period.

**Solution**: 
1. Verify the section is enabled in your query configuration
2. Check the date range includes data (e.g., trades occurred)
3. Ensure you selected fields for each section

#### Token expired

**Cause**: Flex Web Service token has expired.

**Solution**: Generate a new token in the Flex Web Service Configuration.

#### Query ID not found

**Cause**: Query was deleted or Query ID is incorrect.

**Solution**: Verify the Query ID in the Flex Queries list.

### Getting Help

- [IBKR Flex Queries Guide](https://www.ibkrguides.com/clientportal/performanceandstatements/flex.htm)
- [Activity Flex Query Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm)
- [Flex Web Service API](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)
- [ib-flex Library Documentation](https://docs.rs/ib-flex)

---

## Appendix: Complete Field Reference

For the complete list of all available fields in each section, refer to the official [Activity Flex Query Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm) from IBKR.

The `ib-flex` library's type definitions in [`src/types/activity.rs`](src/types/activity.rs) and [`src/types/extended.rs`](src/types/extended.rs) document all supported fields with their Rust types and descriptions.
