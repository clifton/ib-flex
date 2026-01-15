//! Debug full parsing

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = fs::read_to_string("tmp/first_statement.xml")?;

    println!("Testing full parse...");

    match ib_flex::parse_activity_flex(&xml) {
        Ok(stmt) => {
            println!("SUCCESS!");
            println!("Account: {}", stmt.account_id);
            println!("Trades: {}", stmt.trades.items.len());
            println!("Positions: {}", stmt.positions.items.len());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);

            // Try to find which part is failing
            // Test removing the flatten catchall
            use ib_flex::types::activity::*;
            use serde::Deserialize;

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

            // Test with more sections
            #[derive(Debug, Deserialize)]
            struct TestStatement {
                #[serde(rename = "@accountId")]
                account_id: String,
                // Core sections
                #[serde(rename = "Trades", default)]
                trades: TradesWrapper,
                #[serde(rename = "OpenPositions", default)]
                positions: PositionsWrapper,
                #[serde(rename = "CashTransactions", default)]
                cash_transactions: CashTransactionsWrapper,
                #[serde(rename = "CorporateActions", default)]
                corporate_actions: CorporateActionsWrapper,
                #[serde(rename = "SecuritiesInfo", default)]
                securities_info: SecuritiesInfoWrapper,
                #[serde(rename = "ConversionRates", default)]
                conversion_rates: ConversionRatesWrapper,
                // Extended sections
                #[serde(rename = "EquitySummaryInBase", default)]
                equity_summary: EquitySummaryWrapper,
                #[serde(rename = "CashReport", default)]
                cash_report: CashReportWrapper,
                #[serde(rename = "InterestAccruals", default)]
                interest_accruals: InterestAccrualsWrapper,
                #[serde(rename = "MTMPerformanceSummaryInBase", default)]
                mtm_performance: MTMPerformanceSummaryWrapper,
                #[serde(rename = "FIFOPerformanceSummaryInBase", default)]
                fifo_performance: FIFOPerformanceSummaryWrapper,
                #[serde(rename = "MTDYTDPerformanceSummary", default)]
                mtd_ytd_performance: MTDYTDPerformanceSummaryWrapper,
                #[serde(rename = "StmtFunds", default)]
                stmt_funds: StatementOfFundsWrapper,
                #[serde(rename = "ChangeInPositionValues", default)]
                change_in_position_values: ChangeInPositionValueWrapper,
                #[serde(rename = "UnbundledCommissionDetails", default)]
                unbundled_commission: UnbundledCommissionDetailWrapper,
                // Additional sections to test
                #[serde(rename = "ChangeInNAV", default)]
                change_in_nav: Option<ib_flex::types::extended::ChangeInNAV>,
                #[serde(rename = "AccountInformation", default)]
                account_information: Option<ib_flex::types::extended::AccountInformation>,
                // More v0.2.0 sections
                #[serde(rename = "TradeConfirms", default)]
                trade_confirms: ib_flex::types::activity::TradeConfirmsWrapper,
                #[serde(rename = "OptionEAE", default)]
                option_eae: ib_flex::types::activity::OptionEAEWrapper,
                #[serde(rename = "FxTransactions", default)]
                fx_transactions: ib_flex::types::activity::FxTransactionsWrapper,
                #[serde(rename = "ChangeInDividendAccruals", default)]
                change_in_dividend_accruals:
                    ib_flex::types::activity::ChangeInDividendAccrualsWrapper,
                #[serde(rename = "OpenDividendAccruals", default)]
                open_dividend_accruals: ib_flex::types::activity::OpenDividendAccrualsWrapper,
                #[serde(rename = "Transfers", default)]
                transfers: ib_flex::types::activity::TransfersWrapper,
                // v0.3.0 sections - first half
                #[serde(rename = "ClientFees", default)]
                client_fees: ib_flex::types::activity::ClientFeesWrapper,
                #[serde(rename = "ClientFeesDetails", default)]
                client_fees_detail: ib_flex::types::activity::ClientFeesDetailWrapper,
                #[serde(rename = "SLBActivities", default)]
                slb_activities: ib_flex::types::activity::SLBActivitiesWrapper,
                #[serde(rename = "SLBFees", default)]
                slb_fees: ib_flex::types::activity::SLBFeesWrapper,
                #[serde(rename = "HardToBorrowDetails", default)]
                hard_to_borrow_details: ib_flex::types::activity::HardToBorrowDetailsWrapper,
                #[serde(rename = "FxLots", default)]
                fx_lots: ib_flex::types::activity::FxLotsWrapper,
                #[serde(rename = "UnsettledTransfers", default)]
                unsettled_transfers: ib_flex::types::activity::UnsettledTransfersWrapper,
                // v0.3.0 second half - testing each one
                #[serde(rename = "TradeTransfers", default)]
                trade_transfers: ib_flex::types::activity::TradeTransfersWrapper,
                #[serde(rename = "PriorPeriodPositions", default)]
                prior_period_positions: ib_flex::types::activity::PriorPeriodPositionsWrapper,
                // All v0.3.0 sections enabled
                #[serde(rename = "TierInterestDetails", default)]
                tier_interest_details: ib_flex::types::activity::TierInterestDetailsWrapper,
                #[serde(rename = "DebitCardActivities", default)]
                debit_card_activities: ib_flex::types::activity::DebitCardActivitiesWrapper,
                #[serde(rename = "SalesTaxes", default)]
                sales_tax: ib_flex::types::activity::SalesTaxWrapper,
                #[serde(rename = "SymbolSummary", default)]
                symbol_summary: ib_flex::types::activity::SymbolSummaryWrapper,
                #[serde(rename = "AssetSummary", default)]
                asset_summary: ib_flex::types::activity::AssetSummaryWrapper,
                #[serde(rename = "Orders", default)]
                orders: ib_flex::types::activity::OrdersWrapper,
                // Rest ignored with flatten
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying without IgnoredSection, using IgnoredAny directly...");
            match quick_xml::de::from_str::<TestResponse>(&xml) {
                Ok(response) => {
                    println!("  SUCCESS with test struct!");
                    for stmt in &response.statements.statements {
                        println!("    Account: {}", stmt.account_id);
                        println!("    Trades: {}", stmt.trades.items.len());
                    }
                }
                Err(e) => {
                    println!("  FAILED: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
