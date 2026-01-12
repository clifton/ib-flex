# Contributing to ib-flex

Thank you for your interest in contributing to ib-flex! This document provides guidelines for contributing to the project.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:

- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- Your Rust version and platform
- **Anonymized XML sample** (remove account numbers and sensitive data!)

### Requesting Features

Feature requests are welcome! Please describe:

- The use case for the feature
- Proposed API (if applicable)
- Whether you'd like to implement it yourself

### Submitting Pull Requests

1. Fork the repository and create a new branch
2. Make your changes
3. Add tests for new features
4. Ensure all tests pass: `cargo test`
5. Run clippy: `cargo clippy -- -D warnings`
6. Format your code: `cargo fmt`
7. Update CHANGELOG.md
8. Submit a pull request with a clear description

## Development Guidelines

### Code Style

- Follow Rust API guidelines
- Use `cargo fmt` for formatting
- Run `cargo clippy` and fix all warnings
- Add documentation comments for all public items
- Include examples in doc comments where appropriate

### Testing

- All new features must have tests
- Integration tests should use anonymized XML fixtures
- Test error cases, not just happy paths
- Run full test suite before submitting PR

### Documentation

- Document all public APIs with doc comments
- Include `# Arguments`, `# Returns`, `# Errors`, and `# Example` sections
- Update README.md if adding user-facing features
- Keep documentation up to date with code changes

### Dependencies

- Minimize external dependencies
- Only add dependencies that are:
  - Well-maintained
  - Have reasonable version requirements
  - Are necessary for core functionality

### Financial Type Safety

- **Always** use `rust_decimal::Decimal` for monetary values
- **Never** use `f32` or `f64` for money
- Use `chrono::NaiveDate` for dates, `NaiveDateTime` for timestamps
- Test edge cases (zero, negative, very large values)

## Project Structure

```
src/
├── lib.rs              # Public API
├── types/              # Data types
│   ├── common.rs       # Enums
│   ├── activity.rs     # Activity FLEX types
│   └── trade_confirmation.rs
├── parsers/            # XML parsers
│   ├── activity.rs     # Activity parser
│   ├── trade_confirmation.rs
│   └── xml_utils.rs    # Shared utilities
├── error.rs            # Error types
└── version.rs          # Schema version detection
```

## Testing Locally

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_parse_trade

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (Add, Fix, Update, etc.)
- Reference issue numbers when applicable

Examples:
- `Add support for FX conversion rates`
- `Fix decimal parsing for negative values`
- `Update docs for Activity FLEX parser`

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers get started
- Follow Rust community norms

## Questions?

If you have questions about contributing, please:

1. Check the documentation (README.md, PLAN.md, CLAUDE.md)
2. Look at existing issues
3. Open a new issue with your question

## License

By contributing to ib-flex, you agree that your contributions will be licensed under both the MIT and Apache-2.0 licenses.
