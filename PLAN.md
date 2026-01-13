# ib-flex Implementation Plan

**Project**: Interactive Brokers FLEX XML Parser
**Version**: 0.1.0
**Status**: Phase 0 - Project Setup

---

## Executive Summary

Build a production-ready, open-source Rust library for parsing Interactive Brokers FLEX XML statements. The library will support **ALL** FLEX sections with complete feature parity with the mature Python [ibflex library](https://github.com/csingley/ibflex).

**Comprehensive Scope**: Based on analysis of ibflex, we need to support **41 distinct data types** across Activity FLEX statements (see TYPES_ANALYSIS.md for full breakdown).

**Key Goals**:
1. Parse **all** FLEX sections - 41 types including trades, positions, cash flows, corporate actions, fees, accruals, transfers, and more
2. Type-safe with rust_decimal for financial precision
3. Well-documented with examples for all use cases
4. High performance (< 15ms for 10K trades)
5. Zero external dependencies beyond XML/serde
6. Feature parity with Python ibflex library

**Phased Approach**: Given the scope (41 types vs. originally planned 6), we'll release incrementally:
- **v0.1.0**: Core trading types (7 types) - MVP for basic trade/position analysis
- **v0.2.0**: Comprehensive support (17 types) - Full Activity FLEX coverage
- **v0.3.0**: Advanced features (31 types) - Fees, performance summaries, lending
- **v0.4.0**: Complete coverage (41 types) - All edge cases and niche features

---

## Implementation Phases

### Phase 0: Project Setup ✓ (Current)

**Goal**: Initialize repository and project structure

**Tasks**:
- [x] Create CLAUDE.md (project guide)
- [x] Create PLAN.md (this file)
- [ ] Initialize git repository
- [ ] Create Cargo.toml with metadata
- [ ] Create README.md
- [ ] Add MIT + Apache-2.0 licenses
- [ ] Create .gitignore
- [ ] Set up directory structure

**Deliverables**:
- Git repository initialized
- Basic project files in place
- Ready for development

**Estimated Time**: 1-2 hours

---

### Phase 1: Core Type System (v0.1.0)

**Goal**: Define all Rust types for MVP - Core trading functionality

**Target**: 7 critical types + 15 enums for v0.1.0 release

**Tasks**:

#### 1.1 Shared Enums (`src/types/enums.rs`) - **15 enums total**
Based on ibflex/enums.py analysis:
- [ ] AssetClass (STK, OPT, FUT, FOP, CASH, BOND, CMDTY, CFD, etc.)
- [ ] BuySell (BUY, SELL, BUY_CANCEL, SELL_CANCEL)
- [ ] OpenClose (Open, Close, CloseOpen, Unknown)
- [ ] PutCall (P, C)
- [ ] LongShort (Long, Short)
- [ ] TradeType (ExchTrade, BookTrade, FracShare, Adjustment, etc.)
- [ ] OrderType (LMT, MKT, STP, TRAIL, MOC, etc.)
- [ ] CashAction (Deposits, Dividends, Interest, Fees, Taxes, etc.)
- [ ] Reorg (Merger, Spinoff, Split, StockDividend, etc.)
- [ ] OptionAction (Assignment, Exercise, Expiration, Buy, Sell)
- [ ] TransferType (ACATS, ATON, FOP, INTERNAL)
- [ ] Code (A, C, Ex, P, Ca, D, O, etc. - transaction codes)
- [ ] ToFrom (To, From)
- [ ] InOut (IN, OUT)
- [ ] DeliveredReceived (Delivered, Received)
- [ ] Add serde derives with proper rename attributes
- [ ] Add comprehensive doc comments with IB field mappings

#### 1.2 Core Activity FLEX Types (`src/types/activity.rs`) - **v0.1.0 types**
**Critical types for MVP (7 types)**:
- [ ] FlexQueryResponse (top-level container)
- [ ] FlexStatement (statement wrapper)
- [ ] Trade (93 fields!) - symbol, conid, buySell, quantity, price, commission, fifoPnl, etc.
- [ ] OpenPosition (58 fields) - position, markPrice, costBasis, unrealizedPnl, etc.
- [ ] CashTransaction (48 fields) - type, amount, description, dateTime, etc.
- [ ] CorporateAction (46 fields) - actionID, type, quantity, proceeds, etc.
- [ ] SecurityInfo (42 fields) - conid, symbol, cusip, isin, multiplier, strike, expiry
- [ ] ConversionRate (4 fields) - reportDate, fromCurrency, toCurrency, rate
- [ ] Ensure all fields use correct types (Decimal for money, NaiveDate for dates)
- [ ] Add Option<T> for all optional fields
- [ ] Follow ibflex field naming exactly

#### 1.3 Extended Types for v0.2.0+ (`src/types/extended.rs`) - **Future**
Document but don't implement yet (for v0.2.0):
- [ ] AccountInformation (account metadata)
- [ ] ChangeInNAV (portfolio value changes)
- [ ] EquitySummaryByReportDateInBase (asset breakdown)
- [ ] CashReportCurrency (cash flow by currency)
- [ ] TradeConfirm (trade confirmations)
- [ ] OptionEAE (exercise/assignment/expiration)
- [ ] FxTransaction (currency conversions)
- [ ] ChangeInDividendAccrual / OpenDividendAccrual
- [ ] InterestAccrualsCurrency
- [ ] Plus 20+ more types for v0.3.0 and v0.4.0

#### 1.4 Type Module Organization (`src/types/mod.rs`)
- [ ] Re-export commonly used types
- [ ] Organize into v0.1, v0.2, v0.3, v0.4 feature flags
- [ ] Add module-level documentation
- [ ] Create type aliases for convenience

#### 1.5 Field Validation
- [ ] Add validation for required fields
- [ ] Add range checks where appropriate
- [ ] Add custom deserializers for complex fields

**Deliverables**:
- Complete type system for v0.1.0 (7 types + 15 enums)
- All types documented with field descriptions
- Compiles without warnings
- Types match ibflex Python library structure

**Estimated Time**: 1-2 weeks (complexity increased due to 93+ fields in Trade)

**Critical Files**:
- `src/types/enums.rs` (15 enums)
- `src/types/activity.rs` (v0.1.0 types)
- `src/types/extended.rs` (v0.2.0+ types, stubs only)
- `src/types/mod.rs`

**References**:
- See TYPES_ANALYSIS.md for complete field lists
- See ibflex/Types.py for Python implementation
- See ibflex/enums.py for enum definitions

---

### Phase 2: XML Parsers

**Goal**: Implement parsers for Activity and Trade Confirmation FLEX

**Tasks**:

#### 2.1 Error Handling (`src/error.rs`)
- [ ] Define ParseError enum with thiserror
- [ ] XmlError (deserialization failures)
- [ ] InvalidDate (date format errors)
- [ ] InvalidDecimal (decimal format errors)
- [ ] MissingField (required fields missing)
- [ ] UnknownEnumVariant (unknown enum values)
- [ ] UnsupportedSchemaVersion
- [ ] Add contextual error messages

#### 2.2 Activity FLEX Parser (`src/parsers/activity.rs`)
- [ ] Implement parse_activity_flex(xml: &str) -> Result<ActivityFlexStatement>
- [ ] Use quick-xml with serde for deserialization
- [ ] Handle optional sections (trades, positions, etc.)
- [ ] Add proper error context for debugging
- [ ] Handle edge cases (empty sections, missing optional fields)

#### 2.3 Trade Confirmation Parser (`src/parsers/trade_confirmation.rs`)
- [ ] Implement parse_trade_confirmation(xml: &str) -> Result<TradeConfirmationStatement>
- [ ] Similar structure to Activity parser
- [ ] Focus on real-time trade data

#### 2.4 XML Utilities (`src/parsers/xml_utils.rs`)
- [ ] Shared deserialization helpers
- [ ] Custom deserializers for Decimal fields
- [ ] Custom deserializers for date/time fields
- [ ] Utility functions for common XML patterns

#### 2.5 Schema Version Detection (`src/version.rs`)
- [ ] Detect FLEX schema version from XML
- [ ] Return error if unsupported version
- [ ] Support FLEX v3 initially

#### 2.6 Public API (`src/lib.rs`)
- [ ] Re-export parse functions
- [ ] Re-export common types
- [ ] Add module-level documentation
- [ ] Add usage examples in lib.rs docs

**Deliverables**:
- Working parsers for Activity and Trade Confirmation FLEX
- Comprehensive error handling
- Clean public API

**Estimated Time**: 1-2 weeks

**Critical Files**:
- `src/lib.rs`
- `src/error.rs`
- `src/parsers/activity.rs`
- `src/parsers/trade_confirmation.rs`

---

### Phase 3: Testing

**Goal**: Comprehensive test coverage with real-world XML samples

**Tasks**:

#### 3.1 Test Fixtures
- [ ] Create anonymized sample XMLs in `tests/fixtures/`
- [ ] Single trade statement
- [ ] Multiple trades with options
- [ ] Trades with futures
- [ ] Full statement with all sections
- [ ] Large statement (100+ trades)
- [ ] Error cases (malformed XML, missing fields)

#### 3.2 Unit Tests
- [ ] Test enum parsing (AssetCategory, BuySell, etc.)
- [ ] Test decimal parsing edge cases
- [ ] Test date/time parsing
- [ ] Test optional field handling
- [ ] Add tests to each module (`src/types/common.rs`, etc.)

#### 3.3 Integration Tests (`tests/`)
- [ ] `activity_parsing.rs` - Parse complete Activity statements
- [ ] `trade_confirmation.rs` - Parse Trade Confirmation statements
- [ ] `error_handling.rs` - Test error conditions
- [ ] Test all asset classes (stocks, options, futures, FOP)
- [ ] Test multi-currency statements
- [ ] Test edge cases (empty sections, optional fields)

#### 3.4 Property-Based Testing (Optional)
- [ ] Use proptest for fuzzing
- [ ] Generate random valid XML
- [ ] Test parser robustness

**Deliverables**:
- 80%+ code coverage
- All major use cases tested
- All error paths tested

**Estimated Time**: 3-5 days

**Critical Files**:
- `tests/activity_parsing.rs`
- `tests/trade_confirmation.rs`
- `tests/error_handling.rs`
- `tests/fixtures/*.xml`

---

### Phase 4: Examples & Documentation

**Goal**: Make the library easy to use with comprehensive examples and docs

**Tasks**:

#### 4.1 Example Programs (`examples/`)
- [ ] `parse_activity_statement.rs` - Basic parsing and summary
- [ ] `parse_trade_confirmation.rs` - Real-time trade parsing
- [ ] `filter_trades.rs` - Filter trades by criteria (symbol, date, etc.)
- [ ] `calculate_pnl.rs` - Calculate P&L by symbol
- [ ] `calculate_commissions.rs` - Analyze commission costs
- [ ] Add anonymized sample XML files in `examples/fixtures/`

#### 4.2 README.md
- [ ] Project overview and features
- [ ] Installation instructions
- [ ] Quick start example
- [ ] API documentation link
- [ ] FLEX query setup instructions
- [ ] Supported sections and limitations
- [ ] Performance benchmarks
- [ ] Contributing guidelines
- [ ] License information

#### 4.3 FLEX Setup Guide
- [ ] Create `FLEX_SETUP.md` or add to README
- [ ] How to create Activity FLEX query in IB Client Portal
- [ ] How to create Trade Confirmation query
- [ ] Required fields and settings
- [ ] Date format requirements
- [ ] How to generate API token

#### 4.4 Inline Documentation
- [ ] Add doc comments to all public types
- [ ] Add doc comments to all public functions
- [ ] Include examples in doc comments
- [ ] Document error conditions
- [ ] Add module-level docs

#### 4.5 Contributing Guide
- [ ] Create CONTRIBUTING.md
- [ ] How to report bugs
- [ ] How to request features
- [ ] How to submit PRs
- [ ] Code style guidelines

**Deliverables**:
- Comprehensive README
- 5+ working example programs
- Complete inline documentation
- Contributing guidelines

**Estimated Time**: 3-5 days

**Critical Files**:
- `README.md`
- `examples/*.rs`
- `CONTRIBUTING.md`

---

### Phase 5: Performance & Benchmarks

**Goal**: Optimize parser performance and establish benchmarks

**Tasks**:

#### 5.1 Benchmark Suite (`benches/parsing_benchmarks.rs`)
- [ ] Benchmark small statements (10 trades)
- [ ] Benchmark medium statements (100 trades)
- [ ] Benchmark large statements (1K trades)
- [ ] Benchmark very large statements (10K trades)
- [ ] Use Criterion.rs for statistical analysis
- [ ] Document baseline performance

#### 5.2 Performance Optimization
- [ ] Profile parser with flamegraph
- [ ] Identify hot paths
- [ ] Optimize XML deserialization if needed
- [ ] Consider streaming parser for very large files (future)

#### 5.3 Memory Usage Analysis
- [ ] Measure memory usage per trade
- [ ] Test with large statements (100K+ trades)
- [ ] Document memory characteristics

**Deliverables**:
- Benchmark suite with Criterion
- Performance documentation in README
- Meets performance targets:
  - 100 trades: < 2ms
  - 10K trades: < 15ms
  - 100K trades: < 150ms

**Estimated Time**: 2-3 days

**Critical Files**:
- `benches/parsing_benchmarks.rs`

---

### Phase 6: CI/CD & Release Preparation

**Goal**: Set up automation and prepare for crates.io release

**Tasks**:

#### 6.1 GitHub Actions CI (`.github/workflows/ci.yml`)
- [ ] Test on stable, beta, and MSRV (1.70)
- [ ] Run cargo test --all-features
- [ ] Run cargo clippy -- -D warnings
- [ ] Run cargo fmt --all -- --check
- [ ] Run cargo doc --no-deps
- [ ] Test on Linux, macOS, Windows

#### 6.2 Benchmark CI (`.github/workflows/bench.yml`)
- [ ] Run benchmarks on stable
- [ ] Compare against baseline
- [ ] Fail if performance regression > 10%

#### 6.3 Release Workflow (`.github/workflows/release.yml`)
- [ ] Automated on git tag push
- [ ] Run full test suite
- [ ] Publish to crates.io
- [ ] Create GitHub release

#### 6.4 CHANGELOG.md
- [ ] Create CHANGELOG.md
- [ ] Document all changes for v0.1.0
- [ ] Follow Keep a Changelog format

#### 6.5 Crates.io Metadata
- [ ] Ensure Cargo.toml has all required fields
- [ ] Add keywords for discoverability
- [ ] Set repository, homepage, documentation URLs
- [ ] Add categories

#### 6.6 Pre-Release Checklist
- [ ] All tests pass
- [ ] Clippy clean
- [ ] Formatted
- [ ] Documentation complete
- [ ] Examples work
- [ ] CHANGELOG updated
- [ ] Version bumped
- [ ] README accurate

**Deliverables**:
- Automated CI/CD pipeline
- Ready for v0.1.0 release
- All quality checks passing

**Estimated Time**: 2-3 days

**Critical Files**:
- `.github/workflows/ci.yml`
- `CHANGELOG.md`
- `Cargo.toml`

---

## Detailed Technical Specifications

### Cargo.toml Structure

```toml
[package]
name = "ib-flex"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"
rust-version = "1.70"  # MSRV
description = "Pure Rust parser for Interactive Brokers FLEX XML statements"
documentation = "https://docs.rs/ib-flex"
homepage = "https://github.com/your-org/ib-flex"
repository = "https://github.com/your-org/ib-flex"
license = "MIT OR Apache-2.0"
keywords = ["interactive-brokers", "flex", "parser", "trading", "finance"]
categories = ["parser-implementations", "finance"]
readme = "README.md"

[dependencies]
quick-xml = { version = "0.38", features = ["serialize"] }
serde = { version = "1.0", features = ["derive"] }
rust_decimal = { version = "1.36", features = ["serde-with-str"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"

[dev-dependencies]
anyhow = "1.0"
criterion = "0.5"

[[bench]]
name = "parsing_benchmarks"
harness = false
```

### Directory Structure

```
src/
├── lib.rs              # Public API, re-exports
├── types/
│   ├── mod.rs         # Type re-exports
│   ├── common.rs      # AssetCategory, BuySell, etc.
│   ├── activity.rs    # ActivityFlexStatement, Trade, Position
│   └── trade_confirmation.rs  # TradeConfirmation types
├── parsers/
│   ├── mod.rs         # Parser re-exports
│   ├── activity.rs    # parse_activity_flex()
│   ├── trade_confirmation.rs  # parse_trade_confirmation()
│   └── xml_utils.rs   # Shared XML utilities
├── error.rs           # ParseError enum
└── version.rs         # Schema version detection
```

---

## Key Implementation Details

### Decimal Handling

All monetary values must use `rust_decimal::Decimal`:

```rust
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    #[serde(rename = "tradePrice")]
    pub price: Decimal,

    #[serde(rename = "ibCommission")]
    pub commission: Decimal,
}
```

### Date/Time Handling

Use chrono for dates and times:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    #[serde(rename = "tradeDate")]
    pub trade_date: NaiveDate,  // 2025-01-15

    #[serde(rename = "tradeTime")]
    pub trade_time: NaiveDateTime,  // 2025-01-15T09:30:00
}
```

### Enum Deserialization

Handle IB's enum formats:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetCategory {
    STK,   // Stock
    OPT,   // Option
    FUT,   // Future
    FOP,   // Future Option
    CASH,  // Forex
    BOND,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum BuySell {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(other)]
    Unknown,
}
```

### Optional Fields

Many FLEX fields are optional:

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    // Always present
    pub symbol: String,
    pub quantity: Decimal,

    // Optional (may be null/missing)
    pub ib_order_id: Option<i64>,
    pub fifo_pnl_realized: Option<Decimal>,

    // Options-specific (None for non-options)
    pub strike: Option<Decimal>,
    pub expiry: Option<NaiveDate>,
}
```

### Error Context

Provide helpful error messages:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML deserialization error: {message}")]
    XmlError {
        message: String,
        location: Option<String>,
    },

    #[error("Missing required field: {field} in {context}")]
    MissingField {
        field: String,
        context: String,
    },
}

// Usage:
if trade.symbol.is_empty() {
    return Err(ParseError::MissingField {
        field: "symbol".to_string(),
        context: "Trade".to_string(),
    });
}
```

---

## Testing Strategy

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_trade() {
        let xml = r#"
        <FlexQueryResponse>
            <FlexStatements>
                <FlexStatement accountId="U1234567" ...>
                    <Trades>
                        <Trade symbol="AAPL" quantity="100" ... />
                    </Trades>
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>
        "#;

        let result = parse_activity_flex(xml);
        assert!(result.is_ok());
        let statement = result.unwrap();
        assert_eq!(statement.trades.len(), 1);
        assert_eq!(statement.trades[0].symbol, "AAPL");
    }
}
```

### Integration Test Example

```rust
// tests/activity_parsing.rs
use ib_flex::parse_activity_flex;

#[test]
fn test_parse_real_statement() {
    let xml = include_str!("fixtures/real_activity_statement.xml");
    let statement = parse_activity_flex(xml).unwrap();

    assert!(!statement.trades.is_empty());
    assert!(!statement.positions.is_empty());

    // Verify data integrity
    let total_commission: rust_decimal::Decimal =
        statement.trades.iter().map(|t| t.commission).sum();
    assert!(total_commission > rust_decimal::Decimal::ZERO);
}
```

### Benchmark Example

```rust
// benches/parsing_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ib_flex::parse_activity_flex;

fn benchmark_parse_100_trades(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/100_trades.xml");

    c.bench_function("parse 100 trades", |b| {
        b.iter(|| parse_activity_flex(black_box(xml)))
    });
}

criterion_group!(benches, benchmark_parse_100_trades);
criterion_main!(benches);
```

---

## Risk Management

### Potential Issues

1. **XML Format Changes**: IB may change FLEX XML schema
   - **Mitigation**: Version detection, error messages for unsupported versions

2. **Performance Regressions**: Parsing slows down over time
   - **Mitigation**: Benchmark CI, fail on >10% regression

3. **Incomplete Field Coverage**: Missing some FLEX fields
   - **Mitigation**: Comprehensive test fixtures, community feedback

4. **Decimal Precision Loss**: Incorrect decimal handling
   - **Mitigation**: Use rust_decimal everywhere, test edge cases

### Quality Gates

All gates must pass before release:
- ✅ All tests pass (unit + integration)
- ✅ Clippy clean with -D warnings
- ✅ Formatted with rustfmt
- ✅ Documentation complete
- ✅ Examples run successfully
- ✅ Benchmarks meet targets
- ✅ No known bugs

---

## Success Criteria

### v0.1.0 Release Goals

**Functionality**:
- ✅ Parse Activity FLEX statements
- ✅ Parse Trade Confirmation statements
- ✅ Support all major FLEX sections
- ✅ Handle all asset classes (stocks, options, futures, FX)

**Quality**:
- ✅ 80%+ test coverage
- ✅ Zero clippy warnings
- ✅ Comprehensive documentation
- ✅ 5+ working examples

**Performance**:
- ✅ < 15ms for 10K trades
- ✅ < 150ms for 100K trades
- ✅ ~2MB memory for 10K trades

**Usability**:
- ✅ Clear error messages
- ✅ Easy to use API
- ✅ Well-documented

**Community**:
- ✅ Published on crates.io
- ✅ CI/CD pipeline running
- ✅ Contributing guidelines

---

## Post-Release Roadmap

### v0.2.0 (Future)
- Streaming parser for very large files
- Support for older FLEX schema versions
- More comprehensive option strategy support
- Performance optimizations

### v0.3.0 (Future)
- Multi-currency handling improvements
- Advanced P&L calculation helpers
- Integration with other trading libraries

### v1.0.0 (Future)
- Stable API
- Full FLEX v3 support
- Production-grade reliability

---

## Timeline

**Total Estimated Time**: 3-4 weeks

- Phase 0: Project Setup - 1-2 hours ✓
- Phase 1: Core Types - 1 week
- Phase 2: Parsers - 1-2 weeks
- Phase 3: Testing - 3-5 days
- Phase 4: Examples & Docs - 3-5 days
- Phase 5: Performance - 2-3 days
- Phase 6: CI/CD & Release - 2-3 days

---

## Next Steps

Immediate actions:
1. ✅ Create CLAUDE.md and PLAN.md
2. Initialize git repository
3. Set up Cargo.toml
4. Create basic project structure
5. Start Phase 1: Core Types

---

*Last Updated: 2026-01-12*
