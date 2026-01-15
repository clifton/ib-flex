//! Minimal debug tool to find parsing issues

use ib_flex::types::activity::*;
use serde::Deserialize;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = fs::read_to_string("tmp/first_statement.xml")?;

    println!("Testing each wrapper section individually...\n");

    // Test each section by creating a minimal struct with just that section
    macro_rules! test_section {
        ($name:expr, $field:ident, $wrapper:ty) => {{
            #[derive(Debug, Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct TestResponse {
                #[serde(rename = "FlexStatements")]
                statements: TestStatements,
            }

            #[derive(Debug, Deserialize)]
            struct TestStatements {
                #[serde(rename = "FlexStatement", default)]
                statements: Vec<TestStatement>,
            }

            #[allow(dead_code)]
            #[derive(Debug, Deserialize)]
            struct TestStatement {
                #[serde(rename = "@accountId")]
                account_id: String,
                #[serde(rename = $name, default)]
                $field: $wrapper,
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            match quick_xml::de::from_str::<TestResponse>(&xml) {
                Ok(response) => {
                    let count: usize = response
                        .statements
                        .statements
                        .iter()
                        .map(|s| s.$field.items.len())
                        .sum();
                    println!("{}: OK ({} items)", $name, count);
                }
                Err(e) => {
                    println!("{}: FAILED - {:?}", $name, e);
                }
            }
        }};
    }

    test_section!("EquitySummaryInBase", equity_summary, EquitySummaryWrapper);
    test_section!("CashReport", cash_report, CashReportWrapper);
    test_section!(
        "InterestAccruals",
        interest_accruals,
        InterestAccrualsWrapper
    );
    test_section!(
        "MTMPerformanceSummaryInBase",
        mtm_performance,
        MTMPerformanceSummaryWrapper
    );
    test_section!(
        "FIFOPerformanceSummaryInBase",
        fifo_performance,
        FIFOPerformanceSummaryWrapper
    );
    test_section!(
        "MTDYTDPerformanceSummary",
        mtd_ytd_performance,
        MTDYTDPerformanceSummaryWrapper
    );
    test_section!("StmtFunds", stmt_funds, StatementOfFundsWrapper);
    test_section!(
        "ChangeInPositionValues",
        change_in_position_values,
        ChangeInPositionValueWrapper
    );
    test_section!(
        "UnbundledCommissionDetails",
        unbundled_commission,
        UnbundledCommissionDetailWrapper
    );
    test_section!("Trades", trades, TradesWrapper);
    test_section!("OpenPositions", positions, PositionsWrapper);
    test_section!(
        "CashTransactions",
        cash_transactions,
        CashTransactionsWrapper
    );
    test_section!(
        "CorporateActions",
        corporate_actions,
        CorporateActionsWrapper
    );
    test_section!("SecuritiesInfo", securities_info, SecuritiesInfoWrapper);
    test_section!("ConversionRates", conversion_rates, ConversionRatesWrapper);

    Ok(())
}
