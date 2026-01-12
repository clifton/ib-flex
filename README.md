# ib-flex

[![crates.io](https://img.shields.io/crates/v/ib-flex.svg)](https://crates.io/crates/ib-flex)
[![docs.rs](https://docs.rs/ib-flex/badge.svg)](https://docs.rs/ib-flex)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Pure Rust parser for Interactive Brokers FLEX XML statements.

## ⚠️ Work in Progress

This library is currently in early development (v0.1.0-dev). The API is not stable and the parser is not yet implemented.

See [PLAN.md](PLAN.md) for the implementation roadmap.

## Features (Planned)

- 🚀 **Zero-copy parsing** with quick-xml and serde
- 💰 **Financial precision** with rust_decimal for all monetary values
- 📅 **Correct datetime handling** with chrono
- ✅ **Type-safe** enums for asset categories, order types, etc.
- 🔧 **No external dependencies** beyond XML/serde
- 📦 **Supports both Activity and Trade Confirmation FLEX**

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ib-flex = "0.1"
```

## Quick Start (Planned)

```rust
use ib_flex::{parse_activity_flex, AssetCategory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("flex_statement.xml")?;
    let statement = parse_activity_flex(&xml)?;

    println!("Account: {}", statement.account_id);
    println!("Period: {} to {}", statement.from_date, statement.to_date);

    // Filter stock trades
    let stock_trades: Vec<_> = statement.trades
        .iter()
        .filter(|t| t.asset_category == AssetCategory::Stock)
        .collect();

    println!("Stock trades: {}", stock_trades.len());

    // Calculate total commissions
    let total_commission: rust_decimal::Decimal =
        statement.trades.iter().map(|t| t.commission).sum();

    println!("Total commissions: ${}", total_commission);

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

## Supported FLEX Sections (Planned)

### Activity FLEX
- ✅ Trades
- ✅ Open Positions
- ✅ Cash Transactions
- ✅ Corporate Actions
- ✅ Securities Info
- ✅ FX Conversion Rates
- ✅ Change in NAV

### Trade Confirmation FLEX
- ✅ Trade executions with all details
- ✅ Commission breakdown

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

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

This is an open-source project designed to benefit the Rust trading community.

## Documentation

- [API Documentation](https://docs.rs/ib-flex) (coming soon)
- [Implementation Plan](PLAN.md)
- [Project Guide for Claude Code](CLAUDE.md)

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

- Inspired by [csingley/ibflex](https://github.com/csingley/ibflex) (Python)
- Built with [quick-xml](https://github.com/tafia/quick-xml)

---

**Status**: Early development (Phase 0 complete, Phase 1 in progress)
