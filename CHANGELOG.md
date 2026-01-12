# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase 0: Project Setup
- ✅ Created project structure
- ✅ Initialized git repository
- ✅ Added CLAUDE.md (project guide for Claude Code)
- ✅ Added PLAN.md (detailed implementation plan)
- ✅ Added Cargo.toml with metadata
- ✅ Added MIT and Apache-2.0 licenses
- ✅ Created README.md
- ✅ Created CONTRIBUTING.md
- ✅ Created basic type system structure
- ✅ Created parser module structure
- ✅ Project compiles successfully

### To Do
- [ ] Implement Activity FLEX parser
- [ ] Implement Trade Confirmation parser
- [ ] Add comprehensive tests
- [ ] Add example programs
- [ ] Set up CI/CD
- [ ] Publish v0.1.0 to crates.io

## [0.1.0] - Unreleased

Initial release (in development).

### Planned Features
- Parse Activity FLEX XML statements
- Parse Trade Confirmation FLEX statements
- Support for all major asset classes (stocks, options, futures, FX)
- Type-safe parsing with rust_decimal and chrono
- Comprehensive error handling
- Full documentation and examples
