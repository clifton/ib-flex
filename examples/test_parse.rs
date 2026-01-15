use std::fs;

fn main() {
    let xml = fs::read_to_string("tmp/first_statement.xml").unwrap();
    println!("File length: {} bytes", xml.len());
    println!("Last 50 chars: {:?}", &xml[xml.len() - 50..]);

    // Try to parse with ib_flex
    println!("\nTrying ib_flex::parse_activity_flex...");
    match ib_flex::parse_activity_flex(&xml) {
        Ok(stmt) => {
            println!("SUCCESS!");
            println!("Account: {}", stmt.account_id);
            println!("Trades: {}", stmt.trades.items.len());
        }
        Err(e) => {
            println!("FAILED: {:?}", e);

            // Try to find which section is failing
            println!("\nTrying to parse each section progressively...");

            // Just FlexQueryResponse wrapper
            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test1 {
                #[serde(rename = "@queryName")]
                query_name: Option<String>,
                #[serde(rename = "FlexStatements")]
                statements: Test1Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test1Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test1Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test1Statement {
                #[serde(rename = "@accountId")]
                account_id: String,
                // Just catch everything else
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            match quick_xml::de::from_str::<Test1>(&xml) {
                Ok(r) => {
                    println!(
                        "Minimal struct parse OK - account: {}",
                        r.statements
                            .statements
                            .first()
                            .map(|s| s.account_id.as_str())
                            .unwrap_or("none")
                    );
                }
                Err(e) => println!("Minimal struct parse FAILED: {}", e),
            }

            // Try with Trades section
            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test2 {
                #[serde(rename = "FlexStatements")]
                statements: Test2Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test2Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test2Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test2Statement {
                #[serde(rename = "@accountId")]
                account_id: String,
                #[serde(rename = "Trades", default)]
                trades: ib_flex::types::activity::TradesWrapper,
                // Just catch everything else
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying with TradesWrapper...");
            match quick_xml::de::from_str::<Test2>(&xml) {
                Ok(r) => {
                    println!(
                        "With TradesWrapper OK - trades: {}",
                        r.statements
                            .statements
                            .first()
                            .map(|s| s.trades.items.len())
                            .unwrap_or(0)
                    );
                }
                Err(e) => println!("With TradesWrapper FAILED: {}", e),
            }

            // Try with simple Trades (just Trade elements)
            #[derive(Debug, Default, serde::Deserialize)]
            struct SimpleTrades {
                #[serde(rename = "Trade", default)]
                items: Vec<ib_flex::types::activity::Trade>,
            }

            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test3 {
                #[serde(rename = "FlexStatements")]
                statements: Test3Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test3Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test3Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test3Statement {
                #[serde(rename = "@accountId")]
                account_id: String,
                #[serde(rename = "Trades", default)]
                trades: SimpleTrades,
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying with SimpleTrades (just Trade elements)...");
            match quick_xml::de::from_str::<Test3>(&xml) {
                Ok(r) => {
                    println!(
                        "SimpleTrades OK - trades: {}",
                        r.statements
                            .statements
                            .first()
                            .map(|s| s.trades.items.len())
                            .unwrap_or(0)
                    );
                }
                Err(e) => println!("SimpleTrades FAILED: {}", e),
            }

            // Try with Trade + Order
            #[derive(Debug, Default, serde::Deserialize)]
            struct TradesWithOrder {
                #[serde(rename = "Trade", default)]
                items: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "Order", default)]
                orders: Vec<ib_flex::types::activity::Trade>,
            }

            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test4 {
                #[serde(rename = "FlexStatements")]
                statements: Test4Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test4Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test4Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test4Statement {
                #[serde(rename = "Trades", default)]
                trades: TradesWithOrder,
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying with Trade + Order...");
            match quick_xml::de::from_str::<Test4>(&xml) {
                Ok(r) => {
                    let stmt = r.statements.statements.first().unwrap();
                    println!(
                        "Trade + Order OK - trades: {}, orders: {}",
                        stmt.trades.items.len(),
                        stmt.trades.orders.len()
                    );
                }
                Err(e) => println!("Trade + Order FAILED: {}", e),
            }

            // Try with Trade + SymbolSummary
            #[derive(Debug, Default, serde::Deserialize)]
            struct TradesWithSymSummary {
                #[serde(rename = "Trade", default)]
                items: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "SymbolSummary", default)]
                symbol_summaries: Vec<ib_flex::types::activity::Trade>,
            }

            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test5 {
                #[serde(rename = "FlexStatements")]
                statements: Test5Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test5Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test5Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test5Statement {
                #[serde(rename = "Trades", default)]
                trades: TradesWithSymSummary,
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying with Trade + SymbolSummary...");
            match quick_xml::de::from_str::<Test5>(&xml) {
                Ok(r) => {
                    let stmt = r.statements.statements.first().unwrap();
                    println!(
                        "Trade + SymbolSummary OK - trades: {}, symbol_summaries: {}",
                        stmt.trades.items.len(),
                        stmt.trades.symbol_summaries.len()
                    );
                }
                Err(e) => println!("Trade + SymbolSummary FAILED: {}", e),
            }

            // Try with all fields
            #[derive(Debug, Default, serde::Deserialize)]
            struct TradesAll {
                #[serde(rename = "Trade", default)]
                items: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "Order", default)]
                orders: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "SymbolSummary", default)]
                symbol_summaries: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "AssetSummary", default)]
                asset_summaries: Vec<ib_flex::types::activity::Trade>,
                #[serde(rename = "WashSale", default)]
                wash_sales: Vec<ib_flex::types::activity::Trade>,
            }

            #[derive(Debug, serde::Deserialize)]
            #[serde(rename = "FlexQueryResponse")]
            struct Test6 {
                #[serde(rename = "FlexStatements")]
                statements: Test6Statements,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test6Statements {
                #[serde(rename = "FlexStatement")]
                statements: Vec<Test6Statement>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Test6Statement {
                #[serde(rename = "Trades", default)]
                trades: TradesAll,
                #[serde(flatten)]
                _rest: std::collections::HashMap<String, serde::de::IgnoredAny>,
            }

            println!("\nTrying with all 5 trade types...");
            match quick_xml::de::from_str::<Test6>(&xml) {
                Ok(r) => {
                    let stmt = r.statements.statements.first().unwrap();
                    println!(
                        "All 5 types OK - trades: {}, orders: {}, sym: {}, asset: {}, wash: {}",
                        stmt.trades.items.len(),
                        stmt.trades.orders.len(),
                        stmt.trades.symbol_summaries.len(),
                        stmt.trades.asset_summaries.len(),
                        stmt.trades.wash_sales.len()
                    );
                }
                Err(e) => println!("All 5 types FAILED: {}", e),
            }
        }
    }
}
