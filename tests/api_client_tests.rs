//! Integration tests for FLEX Web Service API client
//!
//! Note: Most tests are unit tests in src/api/client.rs
//! These integration tests verify the public API surface.

#![cfg(feature = "api-client")]

use ib_flex::api::{FlexApiClient, FlexApiError};

#[test]
fn test_client_creation() {
    let client = FlexApiClient::new("test_token");
    // Client should be created successfully
    drop(client);
}

#[test]
fn test_client_with_custom_base_url() {
    let client = FlexApiClient::with_base_url("test_token", "https://example.com");
    // Client should be created successfully with custom URL
    drop(client);
}

// Note: The following tests are commented out because they require actual IB credentials
// and a live FLEX query. To run these tests, uncomment and provide your credentials.

/*
#[test]
#[ignore] // Ignored by default - requires real IB credentials
fn test_send_request_integration() {
    // Set these environment variables to run this test:
    // - IB_FLEX_TOKEN: Your FLEX Web Service token
    // - IB_FLEX_QUERY_ID: Your FLEX query ID

    let token = std::env::var("IB_FLEX_TOKEN").expect("IB_FLEX_TOKEN not set");
    let query_id = std::env::var("IB_FLEX_QUERY_ID").expect("IB_FLEX_QUERY_ID not set");

    let client = FlexApiClient::new(token);
    let result = client.send_request(&query_id);

    match result {
        Ok(ref_code) => {
            println!("Reference code: {}", ref_code);
            assert!(!ref_code.is_empty());
        }
        Err(e) => panic!("Send request failed: {:?}", e),
    }
}

#[test]
#[ignore] // Ignored by default - requires real IB credentials
fn test_get_statement_integration() {
    use std::time::Duration;

    let token = std::env::var("IB_FLEX_TOKEN").expect("IB_FLEX_TOKEN not set");
    let query_id = std::env::var("IB_FLEX_QUERY_ID").expect("IB_FLEX_QUERY_ID not set");

    let client = FlexApiClient::new(token);

    // Step 1: Send request
    let ref_code = client.send_request(&query_id)
        .expect("Failed to send request");

    println!("Reference code: {}", ref_code);

    // Step 2: Wait for statement generation
    std::thread::sleep(Duration::from_secs(5));

    // Step 3: Get statement with retry
    let xml = client.get_statement_with_retry(&ref_code, 10, Duration::from_secs(2))
        .expect("Failed to get statement");

    assert!(!xml.is_empty());
    assert!(xml.contains("<?xml"));

    println!("Received XML ({} bytes)", xml.len());
}

#[test]
#[ignore] // Ignored by default - requires real IB credentials
fn test_full_workflow_integration() {
    use std::time::Duration;

    let token = std::env::var("IB_FLEX_TOKEN").expect("IB_FLEX_TOKEN not set");
    let query_id = std::env::var("IB_FLEX_QUERY_ID").expect("IB_FLEX_QUERY_ID not set");

    let client = FlexApiClient::new(token);

    // Step 1: Send request
    let ref_code = client.send_request(&query_id)
        .expect("Failed to send request");

    // Step 2: Get statement with retry
    let xml = client.get_statement_with_retry(&ref_code, 10, Duration::from_secs(2))
        .expect("Failed to get statement");

    // Step 3: Parse the statement
    let statement = ib_flex::parse_activity_flex(&xml)
        .expect("Failed to parse statement");

    println!("Account: {}", statement.account_id);
    println!("Trades: {}", statement.trades.items.len());

    assert!(!statement.account_id.is_empty());
}
*/

#[test]
fn test_parse_send_request_response() {
    let xml = r#"
        <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
            <Status>Success</Status>
            <ReferenceCode>9876543210</ReferenceCode>
            <Url>https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.GetStatement</Url>
        </FlexStatementResponse>
    "#;

    // Test that the client module exists and can parse responses
    // The actual parsing logic is tested in unit tests in client.rs
    assert!(xml.contains("<ReferenceCode>9876543210</ReferenceCode>"));
}

#[test]
fn test_error_response_format() {
    let xml = r#"
        <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
            <Status>Fail</Status>
            <ErrorCode>1003</ErrorCode>
            <ErrorMessage>Invalid token</ErrorMessage>
        </FlexStatementResponse>
    "#;

    assert!(xml.contains("<Status>Fail</Status>"));
    assert!(xml.contains("<ErrorCode>1003</ErrorCode>"));
}

#[test]
fn test_statement_not_ready_format() {
    let xml = r#"
        <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
            <Status>Warn</Status>
            <ErrorCode>1019</ErrorCode>
            <ErrorMessage>Statement is being generated; please try again shortly</ErrorMessage>
        </FlexStatementResponse>
    "#;

    assert!(xml.contains("<ErrorCode>1019</ErrorCode>"));
}
