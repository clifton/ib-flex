# ib-flex - Claude Code Project Guide

**Project**: Interactive Brokers FLEX XML Parser for Rust
**Repository**: Standalone open-source library
**License**: MIT OR Apache-2.0
**Status**: Initial Development (v0.1.0)

---

## Project Overview

`ib-flex` is a pure Rust library for parsing Interactive Brokers FLEX (Flex Web Query) XML statements. This is a **standalone, open-source library** designed to be published on crates.io and used by the broader Rust trading community.

### Key Objectives

1. **Zero dependencies** beyond XML parsing (quick-xml, serde)
2. **Type-safe** parsing with strong financial types (rust_decimal, chrono)
3. **Production-ready** with comprehensive tests, docs, and examples
4. **High performance** using quick-xml with serde support
5. **Community-friendly** with clear documentation and contribution guidelines

### Related Projects

This library is part of a larger portfolio analytics system being built at `/home/clifton/code/convex/mono`. However, **ib-flex must remain completely independent** with zero coupling to internal projects.

---

## Project Structure

```
ib-flex/
├── CLAUDE.md                       # This file - Claude Code guide
├── PLAN.md                         # Detailed implementation plan
├── Cargo.toml                      # Library metadata
├── README.md                       # User-facing documentation
├── LICENSE-MIT                     # MIT license
├── LICENSE-APACHE                  # Apache 2.0 license
├── CHANGELOG.md                    # Version history
├── .gitignore                      # Git ignore rules
├── .github/
│   └── workflows/
│       ├── ci.yml                  # CI: test, clippy, fmt
│       └── release.yml             # Automated crates.io publish
├── src/
│   ├── lib.rs                      # Public API exports
│   ├── types/
│   │   ├── mod.rs                  # Type re-exports
│   │   ├── common.rs               # Shared enums
│   │   ├── activity.rs             # Activity FLEX types
│   │   └── trade_confirmation.rs   # Trade Confirmation types
│   ├── parsers/
│   │   ├── mod.rs                  # Parser traits
│   │   ├── activity.rs             # Activity parser
│   │   ├── trade_confirmation.rs   # Trade Confirmation parser
│   │   └── xml_utils.rs            # Shared XML utilities
│   ├── error.rs                    # Error types
│   └── version.rs                  # Schema version detection
├── examples/
│   ├── parse_activity_statement.rs # Basic usage
│   ├── parse_trade_confirmation.rs # Trade confirmation
│   ├── filter_trades.rs            # Filter trades
│   ├── calculate_pnl.rs            # Calculate P&L
│   └── fixtures/
│       ├── activity_sample.xml     # Sample Activity FLEX
│       └── trade_conf_sample.xml   # Sample Trade Confirmation
├── tests/
│   ├── activity_parsing.rs         # Integration tests
│   ├── trade_confirmation.rs       # Integration tests
│   ├── error_handling.rs           # Error tests
│   └── fixtures/                   # Test XML files
└── benches/
    └── parsing_benchmarks.rs       # Performance benchmarks
```

---

## Development Workflow

### Phase 0: Project Setup (Current)
- [x] Create project structure
- [x] Initialize git repository
- [x] Create CLAUDE.md and PLAN.md
- [ ] Set up Cargo.toml with metadata
- [ ] Create README.md
- [ ] Add licenses (MIT + Apache-2.0)
- [ ] Create .gitignore

### Phase 1: Core Types
- [ ] Implement shared enums (AssetCategory, BuySell, etc.)
- [ ] Implement Activity FLEX types (Trade, Position, etc.)
- [ ] Implement Trade Confirmation types
- [ ] Add comprehensive documentation to all types

### Phase 2: Parsers
- [ ] Implement Activity FLEX parser
- [ ] Implement Trade Confirmation parser
- [ ] Add XML utilities for common parsing tasks
- [ ] Implement error handling

### Phase 3: Testing
- [ ] Add unit tests for all parsers
- [ ] Add integration tests with real XML samples
- [ ] Add error handling tests
- [ ] Add benchmarks

### Phase 4: Examples & Documentation
- [ ] Create example programs
- [ ] Write comprehensive README
- [ ] Add inline documentation to all public APIs
- [ ] Create FLEX query setup guide

### Phase 5: CI/CD & Release
- [ ] Set up GitHub Actions CI
- [ ] Add clippy and rustfmt checks
- [ ] Add MSRV testing
- [ ] Create release workflow
- [ ] Publish v0.1.0 to crates.io

---

## Key Design Principles

### 1. Zero Internal Dependencies
**Rule**: This library must have NO dependencies on the parent monorepo or any internal code.

**Why**: As an open-source library, it must be completely standalone and usable by anyone.

**Enforcement**:
- Only allow dependencies: quick-xml, serde, rust_decimal, chrono, thiserror
- No path dependencies outside this directory
- No git submodules to internal repos

### 2. Financial Type Safety
**Rule**: All monetary values use `rust_decimal::Decimal`, all dates/times use `chrono`.

**Why**: Floating point is unacceptable for financial calculations due to precision loss.

**Enforcement**:
- Never use `f32` or `f64` for money
- Always use `rust_decimal::Decimal` with proper rounding
- Use `NaiveDate` for dates, `NaiveDateTime` for timestamps

### 3. Comprehensive Documentation
**Rule**: Every public type and function must have doc comments with examples.

**Why**: Users need to understand how to use the library without reading the source.

**Format**:
```rust
/// Parse an Activity FLEX XML statement
///
/// # Arguments
/// * `xml` - XML string from IB FLEX query
///
/// # Returns
/// * `Ok(ActivityFlexStatement)` - Successfully parsed statement
/// * `Err(ParseError)` - Parse error with context
///
/// # Errors
/// Returns `ParseError` if XML is malformed or required fields are missing
///
/// # Example
/// ```
/// use ib_flex::parse_activity_flex;
/// let xml = std::fs::read_to_string("statement.xml")?;
/// let statement = parse_activity_flex(&xml)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_activity_flex(xml: &str) -> Result<ActivityFlexStatement>
```

### 4. Test Everything
**Rule**: All parsers must have unit tests, integration tests, and benchmarks.

**Coverage**:
- Unit tests: Test individual parsing functions
- Integration tests: Test with real XML samples
- Error tests: Test all error conditions
- Benchmarks: Measure parsing performance

### 5. Semantic Versioning
**Rule**: Strict semver for all releases.

**Version Format**:
- 0.x.y: Pre-1.0, breaking changes in minor versions
- 1.x.y: Post-1.0, breaking changes only in major
- Patch: Bug fixes only
- Minor: New features, backward compatible
- Major: Breaking changes

---

## FLEX Query Data Model

### Activity FLEX Statement
Top-level structure returned by Activity FLEX queries:

```rust
pub struct ActivityFlexStatement {
    pub account_id: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub when_generated: NaiveDateTime,
    pub trades: Vec<Trade>,
    pub positions: Vec<Position>,
    pub cash_transactions: Vec<CashTransaction>,
    pub corporate_actions: Vec<CorporateAction>,
    pub fx_rates: Vec<FxRate>,
    pub securities: Vec<SecurityInfo>,
}
```

### Trade Structure
Core trade execution data:

```rust
pub struct Trade {
    // IB identifiers
    pub account_id: String,
    pub transaction_id: i64,
    pub ib_order_id: Option<i64>,
    pub exec_id: String,

    // Security
    pub conid: i64,
    pub symbol: String,
    pub asset_category: AssetCategory,
    pub multiplier: Option<Decimal>,

    // Options/Futures
    pub underlying_conid: Option<i64>,
    pub strike: Option<Decimal>,
    pub expiry: Option<NaiveDate>,
    pub put_call: Option<PutCall>,

    // Trade details
    pub trade_date: NaiveDate,
    pub trade_time: NaiveDateTime,
    pub buy_sell: BuySell,
    pub quantity: Decimal,
    pub price: Decimal,

    // Money
    pub proceeds: Decimal,
    pub commission: Decimal,
    pub taxes: Decimal,
    pub net_cash: Decimal,

    // P&L
    pub fifo_pnl_realized: Option<Decimal>,
    pub mtm_pnl: Option<Decimal>,

    // Currency
    pub currency: String,
    pub fx_rate_to_base: Option<Decimal>,
}
```

### Shared Enums

```rust
pub enum AssetCategory {
    Stock,      // STK
    Option,     // OPT
    Future,     // FUT
    FutureOption, // FOP
    Cash,       // CASH/Forex
    Bond,
    Unknown,
}

pub enum BuySell {
    Buy,
    Sell,
}

pub enum OpenClose {
    Open,
    Close,
    CloseOpen, // Same-day round trip
}
```

---

## Interactive Brokers FLEX Reference

### What is FLEX?
FLEX (Flex Web Query) is IB's system for generating customizable XML reports with:
- Trade executions
- Open positions
- Cash transactions
- Corporate actions
- P&L calculations
- Commission details

### FLEX Query Types

**Activity FLEX**:
- Daily EOD data snapshot
- Comprehensive: trades + positions + cash flows + corporate actions
- Best for: Portfolio analytics, tax reporting, reconciliation

**Trade Confirmation FLEX**:
- Real-time trade updates (refreshed immediately after execution)
- Focused on: Trade executions only
- Best for: Intraday monitoring, execution quality analysis

### Date Format Requirements
**CRITICAL**: IB FLEX API only supports:
- ISO-8601: `yyyy-MM-dd` (e.g., "2025-01-15")
- Compact: `yyyyMMdd` (e.g., "20250115")

**NOT SUPPORTED**:
- European format: `dd/MM/yyyy` - Will cause parse errors!

### FLEX Web Service API
Two-step process:
1. **SendRequest**: Submit query ID + token → get reference code
2. **GetStatement**: Poll with reference code → get XML when ready

Base URL: `https://gdcdyn.interactivebrokers.com/Universal/servlet`

---

## Testing Guidelines

### Unit Tests
Located in `src/` alongside implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_trade() {
        let xml = include_str!("../../tests/fixtures/single_trade.xml");
        let result = parse_activity_flex(xml);
        assert!(result.is_ok());
    }
}
```

### Integration Tests
Located in `tests/`:

```rust
// tests/activity_parsing.rs
use ib_flex::parse_activity_flex;

#[test]
fn test_parse_real_statement() {
    let xml = include_str!("fixtures/real_statement.xml");
    let statement = parse_activity_flex(xml).unwrap();
    assert!(!statement.trades.is_empty());
}
```

### Benchmarks
Located in `benches/`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parse(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/100_trades.xml");
    c.bench_function("parse 100 trades", |b| {
        b.iter(|| parse_activity_flex(black_box(xml)))
    });
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
```

---

## Common Tasks

### Adding a New Field to Trade
1. Add field to `Trade` struct in `src/types/activity.rs`
2. Add serde attribute for XML mapping
3. Update tests to verify field parsing
4. Update documentation
5. Add to CHANGELOG.md

### Adding a New FLEX Section
1. Define type in `src/types/activity.rs`
2. Add to `ActivityFlexStatement`
3. Implement parsing in `src/parsers/activity.rs`
4. Add tests with fixtures
5. Document in README.md

### Running Tests
```bash
# All tests
cargo test

# Specific test
cargo test test_parse_basic_trade

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test activity_parsing
```

### Running Benchmarks
```bash
cargo bench
```

### Checking Code Quality
```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Check docs
cargo doc --no-deps --open
```

---

## Release Checklist

Before publishing to crates.io:

### Pre-Release
- [ ] All tests pass (`cargo test`)
- [ ] Clippy clean (`cargo clippy -- -D warnings`)
- [ ] Formatted (`cargo fmt --check`)
- [ ] Documentation complete (`cargo doc --no-deps`)
- [ ] Examples run successfully
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] README.md accurate

### Release
- [ ] Commit: `git commit -am "Release vX.Y.Z"`
- [ ] Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push: `git push && git push --tags`
- [ ] Publish: `cargo publish --dry-run` then `cargo publish`

### Post-Release
- [ ] Verify on crates.io
- [ ] Verify docs.rs builds correctly
- [ ] Announce in project discussions
- [ ] Update dependent projects

---

## Performance Targets

Based on M1 MacBook Pro benchmarks:

- **Small (100 trades)**: < 2ms
- **Medium (1K trades)**: < 10ms
- **Large (10K trades)**: < 15ms
- **Very Large (100K trades)**: < 150ms

Memory usage:
- ~200 bytes per trade
- ~2MB for 10K trades

---

## Known Limitations

### Current Version (0.1.0)
1. **Date formats**: Only ISO-8601 and yyyyMMdd supported (IB limitation)
2. **Schema version**: Only FLEX v3 tested
3. **Multi-leg options**: Basic support only
4. **Complex instruments**: Some exotic derivatives may need special handling

### Future Enhancements
- Support for additional FLEX sections
- More comprehensive option strategy support
- Performance optimizations for very large files
- Streaming parser for memory efficiency

---

## Contributing

This is an open-source project. Contributions welcome!

### Bug Reports
- Include XML sample (anonymize account numbers!)
- Expected vs actual behavior
- Rust version and platform

### Feature Requests
- Describe use case
- Propose API if applicable
- Indicate if you'd like to implement it

### Pull Requests
1. Fork and create branch
2. Add tests for new features
3. Ensure `cargo test` and `cargo clippy` pass
4. Run `cargo fmt`
5. Update CHANGELOG.md
6. Submit PR with clear description

---

## Resources

### Interactive Brokers Documentation
- [FLEX Queries Guide](https://www.ibkrguides.com/orgportal/performanceandstatements/flex.htm)
- [Activity FLEX Reference](https://www.ibkrguides.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm)
- [FLEX Web Service API](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)

### Rust Libraries
- [quick-xml](https://docs.rs/quick-xml) - XML parsing
- [serde](https://docs.rs/serde) - Serialization framework
- [rust_decimal](https://docs.rs/rust_decimal) - Decimal numbers
- [chrono](https://docs.rs/chrono) - Date/time handling
- [thiserror](https://docs.rs/thiserror) - Error derive macros

### Similar Projects
- [csingley/ibflex](https://github.com/csingley/ibflex) - Python FLEX parser (inspiration)
- [alensiljak/interactive-brokers-flex-rs](https://github.com/alensiljak/interactive-brokers-flex-rs) - Rust FLEX parser (existing)

---

## Support

- **Issues**: GitHub issue tracker
- **Discussions**: GitHub discussions
- **Email**: [to be added]

---

*Last Updated: 2026-01-12*
*Project maintained by [Your Name]*
