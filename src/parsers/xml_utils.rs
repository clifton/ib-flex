//! XML parsing utilities and custom deserializers

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

/// Parse a date string in either YYYY-MM-DD or YYYYMMDD format
fn parse_flex_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    // Try ISO format first (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(date);
    }
    // Try compact format (YYYYMMDD)
    NaiveDate::parse_from_str(s, "%Y%m%d")
}

/// Deserialize a NaiveDate from either YYYY-MM-DD or YYYYMMDD format
pub fn deserialize_flex_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_flex_date(&s).map_err(serde::de::Error::custom)
}

/// Deserialize an optional Decimal, treating empty strings as None
pub fn deserialize_optional_decimal<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => s
            .parse::<Decimal>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// Deserialize an optional NaiveDate, treating empty strings as None
/// Supports both YYYY-MM-DD and YYYYMMDD formats
pub fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => parse_flex_date(s)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// Deserialize an optional string, treating empty strings as None
pub fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref() {
        None | Some("") => Ok(None),
        Some(_) => Ok(s),
    }
}

/// Deserialize an optional boolean from IB's Y/N format
///
/// Interactive Brokers uses "Y" for true and "N" for false in XML attributes.
/// Empty strings or missing attributes are treated as None.
pub fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref() {
        None | Some("") => Ok(None),
        Some("Y") | Some("y") => Ok(Some(true)),
        Some("N") | Some("n") => Ok(Some(false)),
        Some(other) => Err(serde::de::Error::custom(format!(
            "Invalid boolean value '{}', expected 'Y' or 'N'",
            other
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TestStruct {
        #[serde(
            rename = "@value",
            default,
            deserialize_with = "deserialize_optional_decimal"
        )]
        value: Option<Decimal>,

        #[serde(
            rename = "@date",
            default,
            deserialize_with = "deserialize_optional_date"
        )]
        date: Option<NaiveDate>,

        #[serde(
            rename = "@text",
            default,
            deserialize_with = "deserialize_optional_string"
        )]
        text: Option<String>,

        #[serde(
            rename = "@flag",
            default,
            deserialize_with = "deserialize_optional_bool"
        )]
        flag: Option<bool>,
    }

    #[test]
    fn test_empty_string_decimal() {
        let xml = r#"<TestStruct value="" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, None);
    }

    #[test]
    fn test_valid_decimal() {
        let xml = r#"<TestStruct value="123.45" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, Some(Decimal::new(12345, 2)));
    }

    #[test]
    fn test_empty_string_date() {
        let xml = r#"<TestStruct date="" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().date, None);
    }

    #[test]
    fn test_empty_string_text() {
        let xml = r#"<TestStruct text="" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().text, None);
    }

    #[test]
    fn test_bool_y() {
        let xml = r#"<TestStruct flag="Y" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().flag, Some(true));
    }

    #[test]
    fn test_bool_n() {
        let xml = r#"<TestStruct flag="N" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().flag, Some(false));
    }

    #[test]
    fn test_bool_empty() {
        let xml = r#"<TestStruct flag="" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().flag, None);
    }

    #[test]
    fn test_bool_missing() {
        let xml = r#"<TestStruct />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().flag, None);
    }

    #[test]
    fn test_bool_invalid() {
        let xml = r#"<TestStruct flag="X" />"#;
        let result: Result<TestStruct, _> = quick_xml::de::from_str(xml);
        assert!(result.is_err());
    }
}
