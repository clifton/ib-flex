# ib-flex

[![crates.io](https://img.shields.io/crates/v/ib-flex.svg)](https://crates.io/crates/ib-flex)
[![docs.rs](https://docs.rs/ib-flex/badge.svg)](https://docs.rs/ib-flex)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Pure Rust parser for Interactive Brokers FLEX XML statements with comprehensive type safety and edge case coverage.

**Status**: v0.2.0 - Extended FLEX Support

Fast, type-safe parser for Interactive Brokers FLEX (Flex Web Query) XML statements. Built with zero external dependencies beyond XML/serde, featuring 100+ enum variants, 10+ extended FLEX sections, comprehensive edge case handling, and excellent performance (~6.5µs for minimal parsing).

## Features

- 🚀 **High performance** with quick-xml and serde (~6.5µs for minimal parsing)
- 💰 **Financial precision** with rust_decimal for all monetary values (no floating point!)
- 📅 **Correct datetime handling** with chrono
- ✅ **Type-safe** with 15 enums covering 100+ variants
- 🔧 **Zero dependencies** beyond XML/serde ecosystem
- 📦 **Comprehensive coverage** of Activity FLEX statements
- 🛡️ **Well-tested**
- 🎯 **Edge case handling** for warrants, T-Bills, CFDs, fractional shares, cancelled trades
- 🌐 **Optional API client** for programmatic FLEX statement retrieval

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ib-flex = "0.1"
```

## Quick Start

```rust
use ib_flex::{parse_activity_flex, AssetCategory, BuySell};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("flex_statement.xml")?;
    let response = parse_activity_flex(&xml)?;
    let statement = &response.flex_statements[0];

    println!("Account: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);
    println!("Generated: {}", response.when_generated);

    // Filter stock trades
    let stock_trades: Vec<_> = statement.trades
        .iter()
        .filter(|t| matches!(t.asset_category, Some(AssetCategory::STK)))
        .collect();

    println!("Stock trades: {}", stock_trades.len());

    // Calculate total P&L
    let total_pnl: rust_decimal::Decimal = statement.trades
        .iter()
        .filter_map(|t| t.fifo_pnl_realized)
        .sum();

    println!("Total realized P&L: ${}", total_pnl);

    // Find all buy orders
    let buys: Vec<_> = statement.trades
        .iter()
        .filter(|t| matches!(t.buy_sell, Some(BuySell::BUY)))
        .collect();

    println!("Buy orders: {}", buys.len());

    Ok(())
}
```

## FLEX Query Setup

Interactive Brokers FLEX queries must be configured in the IB Client Portal:

1. Navigate to: Reports → Flex Queries → Create Activity Flex Query
2. Select required sections (Trades, Positions, Cash Transactions, etc.)
3. Choose date format: ISO-8601 (`yyyy-MM-dd`) or compact (`yyyyMMdd`)
4. Set output format to XML
5. Save query and note the Query ID

**Important**: European date formats (`dd/MM/yyyy`) are NOT supported by the IB FLEX API.

## FLEX Web Service API Client (Optional)

The `api-client` feature provides programmatic access to fetch FLEX statements directly from Interactive Brokers without manual downloads.

### Installation with API Client

```toml
[dependencies]
ib-flex = { version = "0.1", features = ["api-client"] }
```

### API Setup

1. Log in to IB Account Management
2. Navigate to: Reports → Settings → FlexWeb Service
3. Generate a FLEX Web Service token (keep it secure!)
4. Note your FLEX Query ID from the Flex Queries page

### Usage Example

```rust
use ib_flex::api::FlexApiClient;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with your token
    let client = FlexApiClient::new("YOUR_TOKEN");

    // Step 1: Send request with your query ID
    let reference_code = client.send_request("123456")?;

    // Step 2: Get statement with automatic retry
    let xml = client.get_statement_with_retry(
        &reference_code,
        10,                           // max retries
        Duration::from_secs(2)        // delay between retries
    )?;

    // Step 3: Parse the statement
    let statement = ib_flex::parse_activity_flex(&xml)?;
    println!("Trades: {}", statement.trades.items.len());

    Ok(())
}
```

### API Examples

Run the API examples (requires IB credentials):

```bash
export IB_FLEX_TOKEN="your_token"
export IB_FLEX_QUERY_ID="your_query_id"
cargo run --example fetch_flex_statement --features api-client
cargo run --example api_simple_usage --features api-client
cargo run --example api_with_retry --features api-client
```

## Supported FLEX Sections

### Activity FLEX (v0.1.0 - Core Types)
- ✅ **Trades** - Executions with 40+ fields including P&L, commissions, dates, security details
- ✅ **Open Positions** - Current holdings with 30+ fields
- ✅ **Cash Transactions** - Deposits, withdrawals, interest, fees, dividends
- ✅ **Corporate Actions** - Splits, mergers, spinoffs, dividends (36 action types)
- ✅ **Securities Info** - Reference data for all traded instruments
- ✅ **FX Conversion Rates** - Currency conversion rates for multi-currency accounts

### Extended FLEX Sections (v0.2.0+)
- ✅ **Account Information** - Account metadata and configuration
- ✅ **Change in NAV** - Net asset value changes with transfers and P&L breakdown
- ✅ **Equity Summary** - Asset allocation by category (cash, stocks, options, bonds)
- ✅ **Cash Report** - Detailed cash flow by currency
- ✅ **Trade Confirmations** - Real-time trade execution confirmations
- ✅ **Option EAE** - Option exercises, assignments, and expirations
- ✅ **FX Transactions** - Foreign exchange conversions
- ✅ **Dividend Accruals** - Accrued and open dividend tracking
- ✅ **Interest Accruals** - Interest accrual tracking by currency
- ✅ **Transfers** - Security transfers (ACATS, ATON, FOP, etc.)

### Asset Classes Supported
- ✅ **Stocks (STK)** - Including fractional shares
- ✅ **Options (OPT)** - Calls, puts, assignments, expirations
- ✅ **Futures (FUT)** - All major contracts (ES, NQ, CL, GC, etc.)
- ✅ **Forex (CASH)** - FX trades and positions
- ✅ **Bonds (BOND)** - Treasuries, corporate, municipal
- ✅ **Treasury Bills (BILL)** - With maturity handling
- ✅ **Warrants (WAR)** - Equity warrants
- ✅ **CFDs** - Contract for difference
- ✅ **Funds, Commodities, and more** - 20 asset categories total

### Trade Confirmation FLEX
- 🚧 **Planned for v0.2.0** - Trade execution confirmations

## Performance

Benchmarked on M1 MacBook Pro:

| Statement Type | Transactions | Parse Time |
|---------------|--------------|------------|
| Minimal | 1 trade | ~6.5 µs |
| Options | 4 trades | ~65 µs |
| Cash | 15 transactions | ~71 µs |

Memory efficient with approximately 200 bytes per trade. Parsing 10,000 trades uses ~2MB of memory.

## Type Safety

All financial values use `rust_decimal::Decimal` for precise calculations without floating-point errors. The library includes 15 comprehensive enums with 100+ variants covering:

- **AssetCategory** (20 variants) - STK, OPT, FUT, CASH, BOND, BILL, WAR, CFD, etc.
- **Reorg** (36 variants) - All corporate action types
- **Code** (50+ variants) - Transaction classification codes
- **CashAction** (13 variants) - Cash transaction types
- **OrderType** (13 variants) - Market, limit, stop, etc.
- **BuySell, OpenClose, PutCall, LongShort, TradeType, OptionAction, and more**

## Examples

The repository includes several complete example programs:

### Parsing Examples
1. **parse_activity_statement.rs** - Basic parsing and display
2. **filter_trades.rs** - Filter by asset class, side, symbol, quantity, P&L, date range
3. **calculate_commissions.rs** - Analyze commission costs by category

Run parsing examples:
```bash
cargo run --example parse_activity_statement
cargo run --example filter_trades
cargo run --example calculate_commissions
```

### API Client Examples (requires `api-client` feature)
4. **fetch_flex_statement.rs** - Complete API workflow with detailed output
5. **api_simple_usage.rs** - Minimal API client usage
6. **api_with_retry.rs** - API client with automatic retry logic

Run API examples:
```bash
export IB_FLEX_TOKEN="your_token"
export IB_FLEX_QUERY_ID="your_query_id"
cargo run --example fetch_flex_statement --features api-client
```

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Benchmark

```bash
cargo bench
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy -- -D warnings
```

## Testing

The library has comprehensive test coverage including stress tests and property-based testing:

- **94 tests total** (100% passing)
- **47 integration tests** covering all asset classes and edge cases
- **13 extended types tests** for v0.2.0+ FLEX sections
- **11 error tests** for malformed XML and invalid data
- **11 unit tests** for custom deserializers
- **12 doc tests** in inline documentation
- **15 XML fixtures** including extended types, warrants, T-Bills, CFDs, fractional shares, cancelled trades
- 
### Reliability Testing

The library includes comprehensive reliability tests using:
- **Property-based testing** with proptest for random inputs
- **Stress tests** for large XML files (1000+ trades, 500+ positions)
- **Concurrency tests** for thread safety
- **Memory efficiency tests** for repeated parsing
- **Edge case fuzzing** for malformed inputs

Run tests with:
```bash
cargo test           # All tests
cargo test --doc     # Documentation tests only
```

## Contributing

Contributions welcome! This is an open-source project designed to benefit the Rust trading community.

**Before submitting a PR:**
1. Ensure all tests pass: `cargo test`
2. Run clippy: `cargo clippy -- -D warnings`
3. Format code: `cargo fmt`
4. Add tests for new features
5. Update CHANGELOG.md

Bug reports and feature requests are appreciated. When reporting bugs, please include:
- Anonymized XML sample demonstrating the issue
- Expected vs actual behavior
- Rust version and platform

## Documentation

- [API Documentation](https://docs.rs/ib-flex)
- [Implementation Plan](PLAN.md) - Full implementation details and statistics
- [Project Guide for Claude Code](CLAUDE.md) - Development guide
- [Edge Cases Summary](EDGE_CASES_SUMMARY.md) - Comprehensive edge case analysis
- [Types Analysis](TYPES_ANALYSIS.md) - Type system breakdown

## Resources

- [IB FLEX Queries Guide](https://www.ibkrguides.com/orgportal/performanceandstatements/flex.htm)
- [Activity FLEX Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm)
- [FLEX Web Service API](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by [csingley/ibflex](https://github.com/csingley/ibflex) (Python) - Comprehensive enum research
- Built with [quick-xml](https://github.com/tafia/quick-xml) - Fast XML parsing with serde
- [rust_decimal](https://github.com/paupino/rust-decimal) - Financial precision
- [chrono](https://github.com/chronotope/chrono) - Date/time handling

---

**Status**: v0.2.0 - Extended FLEX Support (94/94 tests passing, zero warnings)

See [PLAN.md](PLAN.md) for detailed implementation statistics and [EDGE_CASES_SUMMARY.md](EDGE_CASES_SUMMARY.md) for edge case coverage.
