//! FLEX Web Service API client
//!
//! This module provides a client for the Interactive Brokers FLEX Web Service API,
//! allowing programmatic retrieval of FLEX statements without manual downloads.
//!
//! The FLEX Web Service API uses a two-step process:
//! 1. **SendRequest**: Submit a query ID with your token → receive a reference code
//! 2. **GetStatement**: Poll with the reference code → receive XML when ready
//!
//! ## Feature Flag
//!
//! This module requires the `api-client` feature:
//!
//! ```toml
//! [dependencies]
//! ib-flex = { version = "0.1", features = ["api-client"] }
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "api-client")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use ib_flex::api::FlexApiClient;
//! use std::time::Duration;
//!
//! // Create client with your token
//! let client = FlexApiClient::new("YOUR_TOKEN_HERE");
//!
//! // Step 1: Send request with your query ID
//! let reference_code = client.send_request("123456")?;
//! println!("Reference code: {}", reference_code);
//!
//! // Step 2: Wait a moment for IB to generate the report
//! std::thread::sleep(Duration::from_secs(5));
//!
//! // Step 3: Get the statement XML
//! let xml = client.get_statement(&reference_code)?;
//!
//! // Step 4: Parse the XML
//! let statement = ib_flex::parse_activity_flex(&xml)?;
//! println!("Trades: {}", statement.trades.items.len());
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "api-client"))]
//! # fn main() {}
//! ```
//!
//! ## API Endpoints
//!
//! - **Base URL**: `https://gdcdyn.interactivebrokers.com/Universal/servlet`
//! - **SendRequest**: `FlexStatementService.SendRequest?t=TOKEN&q=QUERY_ID&v=3`
//! - **GetStatement**: `FlexStatementService.GetStatement?t=TOKEN&q=REFERENCE_CODE&v=3`
//!
//! ## References
//!
//! - [FLEX Web Service Documentation](https://www.interactivebrokers.com/campus/ibkr-api-page/flex-web-service/)

#[cfg(feature = "api-client")]
mod client;

#[cfg(feature = "api-client")]
pub use client::{FlexApiClient, FlexApiError};
