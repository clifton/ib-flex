# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2026-01-14

### Changed
- Minor improvements and maintenance
## [0.1.3] - 2026-01-14

### Changed
- Minor improvements and maintenance
## [0.1.2] - 2026-01-14

### Added
- Robust release script for changelog generation and version bumping

## [0.1.1] - 2026-01-14

### Added
- Daily portfolio parsing support ([#7](https://github.com/clifton/ib-flex/pull/7))
- Comprehensive v0.2.0 extended FLEX support ([#3](https://github.com/clifton/ib-flex/pull/3))
- Comprehensive reliability testing suite ([#4](https://github.com/clifton/ib-flex/pull/4))
- FLEX Web Service API client (optional `flex-service` feature) ([#5](https://github.com/clifton/ib-flex/pull/5))
- Trade Confirmation parsing, version detection, type detection ([#6](https://github.com/clifton/ib-flex/pull/6))

### Fixed
- Handle interleaved Trades elements with enum-based parsing ([#8](https://github.com/clifton/ib-flex/pull/8))

## [0.1.0] - 2026-01-14

Initial release.

### Added
- Parse Activity FLEX XML statements
- Parse Trade Confirmation FLEX statements
- Support for all major asset classes (stocks, options, futures, FX)
- Type-safe parsing with rust_decimal and chrono
- Comprehensive error handling
- Documentation and examples
