# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-01-14

### Added
- Add robust release script
- Add robust release script
- Add daily portfolio parsing support and comprehensive tests (#7) ([#7](https://github.com/clifton/ib-flex/pull/7))
- Add comprehensive v0.2.0 extended FLEX support (#3) ([#3](https://github.com/clifton/ib-flex/pull/3))
- Add comprehensive reliability testing suite (#4) ([#4](https://github.com/clifton/ib-flex/pull/4))

### Fixed
- Handle interleaved Trades elements with enum-based parsing (#8) ([#8](https://github.com/clifton/ib-flex/pull/8))
- Fix formatting in README for automatic detection feature
- Remove unused import from reliability tests

### Documentation
- Add mandatory pre-commit requirements to CLAUDE.md
- Add mandatory pre-commit requirements to CLAUDE.md
- Remove version numbers and test counts from README

### Changed
- dont commit local ralph loop stuff
- Complete All Library TODOs - Trade Confirmation, Version Detection, Type Detection (#6) ([#6](https://github.com/clifton/ib-flex/pull/6))
- FLEX Web Service API Client (Optional Feature) (#5) ([#5](https://github.com/clifton/ib-flex/pull/5))
- Remove explicit "production ready" references throughout documentation
- Update README to reflect v0.1.0 production-ready status
- One shot the flex query parser (#2) ([#2](https://github.com/clifton/ib-flex/pull/2))
- "Claude Code Review workflow"
- "Claude PR Assistant workflow"
## [0.1.0] - Unreleased

Initial release (in development).

### Planned Features
- Parse Activity FLEX XML statements
- Parse Trade Confirmation FLEX statements
- Support for all major asset classes (stocks, options, futures, FX)
- Type-safe parsing with rust_decimal and chrono
- Comprehensive error handling
- Full documentation and examples
