//! FLEX Web Service API client implementation

use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

/// Base URL for IB FLEX Web Service API
const FLEX_BASE_URL: &str = "https://gdcdyn.interactivebrokers.com/Universal/servlet";

/// FLEX Web Service API errors
#[derive(Debug, Error)]
pub enum FlexApiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// IB API returned an error
    #[error("IB API error: {0}")]
    ApiError(String),

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlError(String),

    /// Statement not ready yet
    #[error("Statement not ready (try again later)")]
    StatementNotReady,

    /// Invalid response format
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}

/// Result type for FLEX API operations
pub type Result<T> = std::result::Result<T, FlexApiError>;

/// FLEX Web Service API client
///
/// Provides async programmatic access to Interactive Brokers FLEX statements
/// using the FLEX Web Service API.
///
/// # Authentication
///
/// Requires a FLEX Web Service token from IB Account Management:
/// 1. Log in to IB Account Management
/// 2. Navigate to: Reports → Settings → FlexWeb Service
/// 3. Generate a token (keep it secure!)
///
/// # Example
///
/// ```rust,no_run
/// use ib_flex::api::FlexApiClient;
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = FlexApiClient::new("YOUR_TOKEN");
///
/// // Send request
/// let ref_code = client.send_request("QUERY_ID").await?;
///
/// // Wait for report generation
/// tokio::time::sleep(Duration::from_secs(5)).await;
///
/// // Get statement
/// let xml = client.get_statement(&ref_code).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FlexApiClient {
    /// FLEX Web Service token
    token: String,
    /// Base URL for API endpoints
    base_url: String,
    /// HTTP client
    client: Client,
}

impl FlexApiClient {
    /// Create a new FLEX API client with the given token
    ///
    /// # Arguments
    ///
    /// * `token` - Your FLEX Web Service token from IB Account Management
    ///
    /// # Example
    ///
    /// ```rust
    /// use ib_flex::api::FlexApiClient;
    ///
    /// let client = FlexApiClient::new("YOUR_TOKEN_HERE");
    /// ```
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: FLEX_BASE_URL.to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// Create a client with a custom base URL (for testing)
    ///
    /// # Arguments
    ///
    /// * `token` - Your FLEX Web Service token
    /// * `base_url` - Custom base URL for the API
    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: base_url.into(),
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// Send a FLEX query request
    ///
    /// Initiates a FLEX query execution on IB servers. Returns a reference code
    /// that can be used to retrieve the generated statement.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Your FLEX query ID from IB Account Management
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Reference code for retrieving the statement
    /// * `Err(FlexApiError)` - If the request fails or IB returns an error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ib_flex::api::FlexApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FlexApiClient::new("YOUR_TOKEN");
    /// let reference_code = client.send_request("123456").await?;
    /// println!("Reference code: {}", reference_code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_request(&self, query_id: &str) -> Result<String> {
        let url = format!(
            "{}/FlexStatementService.SendRequest?t={}&q={}&v=3",
            self.base_url, self.token, query_id
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;

        // Parse XML response
        self.parse_send_request_response(&body)
    }

    /// Get a FLEX statement by reference code
    ///
    /// Retrieves the XML statement for a previously submitted query.
    /// The statement must be ready (typically takes a few seconds).
    ///
    /// # Arguments
    ///
    /// * `reference_code` - Reference code from `send_request()`
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - XML statement content
    /// * `Err(FlexApiError::StatementNotReady)` - If statement is not ready yet
    /// * `Err(FlexApiError)` - If the request fails or IB returns an error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ib_flex::api::FlexApiClient;
    /// # use std::time::Duration;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FlexApiClient::new("YOUR_TOKEN");
    /// let ref_code = client.send_request("123456").await?;
    ///
    /// // Wait for statement generation
    /// tokio::time::sleep(Duration::from_secs(5)).await;
    ///
    /// let xml = client.get_statement(&ref_code).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_statement(&self, reference_code: &str) -> Result<String> {
        let url = format!(
            "{}/FlexStatementService.GetStatement?t={}&q={}&v=3",
            self.base_url, self.token, reference_code
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;

        // Check if this is an error response
        if body.contains("<Status>") {
            self.parse_get_statement_response(&body)
        } else {
            // This is the actual XML statement
            Ok(body)
        }
    }

    /// Get a FLEX statement with automatic retry
    ///
    /// Like `get_statement()` but automatically retries if the statement is not ready yet.
    ///
    /// # Arguments
    ///
    /// * `reference_code` - Reference code from `send_request()`
    /// * `max_retries` - Maximum number of retry attempts
    /// * `retry_delay` - Delay between retries
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - XML statement content
    /// * `Err(FlexApiError)` - If max retries exceeded or other error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ib_flex::api::FlexApiClient;
    /// # use std::time::Duration;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FlexApiClient::new("YOUR_TOKEN");
    /// let ref_code = client.send_request("123456").await?;
    ///
    /// // Automatically retry up to 10 times with 2-second delays
    /// let xml = client.get_statement_with_retry(&ref_code, 10, Duration::from_secs(2)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_statement_with_retry(
        &self,
        reference_code: &str,
        max_retries: usize,
        retry_delay: Duration,
    ) -> Result<String> {
        for attempt in 0..max_retries {
            match self.get_statement(reference_code).await {
                Ok(xml) => return Ok(xml),
                Err(FlexApiError::StatementNotReady) => {
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(retry_delay).await;
                        continue;
                    } else {
                        return Err(FlexApiError::StatementNotReady);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Err(FlexApiError::StatementNotReady)
    }

    /// Parse SendRequest XML response
    ///
    /// Expected format:
    /// ```xml
    /// <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
    ///   <Status>Success</Status>
    ///   <ReferenceCode>1234567890</ReferenceCode>
    ///   <Url>https://...</Url>
    /// </FlexStatementResponse>
    /// ```
    fn parse_send_request_response(&self, xml: &str) -> Result<String> {
        // Simple XML parsing - look for ReferenceCode
        if let Some(start) = xml.find("<ReferenceCode>") {
            if let Some(end) = xml[start..].find("</ReferenceCode>") {
                let ref_code = &xml[start + 15..start + end];
                return Ok(ref_code.to_string());
            }
        }

        // Check for error status
        if xml.contains("<Status>Fail</Status>") || xml.contains("<Status>Warn</Status>") {
            if let Some(start) = xml.find("<ErrorMessage>") {
                if let Some(end) = xml[start..].find("</ErrorMessage>") {
                    let error = &xml[start + 14..start + end];
                    return Err(FlexApiError::ApiError(error.to_string()));
                }
            }
            return Err(FlexApiError::ApiError("Unknown error".to_string()));
        }

        Err(FlexApiError::InvalidResponse(
            "Could not parse reference code".to_string(),
        ))
    }

    /// Parse GetStatement status response
    ///
    /// Expected format when not ready:
    /// ```xml
    /// <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
    ///   <Status>Warn</Status>
    ///   <ErrorCode>1019</ErrorCode>
    ///   <ErrorMessage>Statement is being generated; please try again shortly</ErrorMessage>
    /// </FlexStatementResponse>
    /// ```
    fn parse_get_statement_response(&self, xml: &str) -> Result<String> {
        // Check for "statement not ready" error (code 1019)
        if xml.contains("<ErrorCode>1019</ErrorCode>") {
            return Err(FlexApiError::StatementNotReady);
        }

        // Check for other errors
        if xml.contains("<Status>Fail</Status>") || xml.contains("<Status>Warn</Status>") {
            if let Some(start) = xml.find("<ErrorMessage>") {
                if let Some(end) = xml[start..].find("</ErrorMessage>") {
                    let error = &xml[start + 14..start + end];
                    return Err(FlexApiError::ApiError(error.to_string()));
                }
            }
            return Err(FlexApiError::ApiError("Unknown error".to_string()));
        }

        Err(FlexApiError::InvalidResponse(
            "Unexpected response format".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_send_request_success() {
        let client = FlexApiClient::new("test_token");
        let xml = r#"
            <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
                <Status>Success</Status>
                <ReferenceCode>1234567890</ReferenceCode>
                <Url>https://example.com</Url>
            </FlexStatementResponse>
        "#;

        let result = client.parse_send_request_response(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1234567890");
    }

    #[test]
    fn test_parse_send_request_error() {
        let client = FlexApiClient::new("test_token");
        let xml = r#"
            <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
                <Status>Fail</Status>
                <ErrorCode>1003</ErrorCode>
                <ErrorMessage>Invalid token</ErrorMessage>
            </FlexStatementResponse>
        "#;

        let result = client.parse_send_request_response(xml);
        assert!(result.is_err());
        match result {
            Err(FlexApiError::ApiError(msg)) => assert_eq!(msg, "Invalid token"),
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_parse_get_statement_not_ready() {
        let client = FlexApiClient::new("test_token");
        let xml = r#"
            <FlexStatementResponse timestamp='01 January, 2025 12:00 AM EDT'>
                <Status>Warn</Status>
                <ErrorCode>1019</ErrorCode>
                <ErrorMessage>Statement is being generated; please try again shortly</ErrorMessage>
            </FlexStatementResponse>
        "#;

        let result = client.parse_get_statement_response(xml);
        assert!(result.is_err());
        match result {
            Err(FlexApiError::StatementNotReady) => (),
            _ => panic!("Expected StatementNotReady"),
        }
    }

    #[test]
    fn test_client_creation() {
        let client = FlexApiClient::new("my_token");
        assert_eq!(client.token, "my_token");
        assert_eq!(client.base_url, FLEX_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_url() {
        let client = FlexApiClient::with_base_url("my_token", "https://custom.url");
        assert_eq!(client.token, "my_token");
        assert_eq!(client.base_url, "https://custom.url");
    }
}
