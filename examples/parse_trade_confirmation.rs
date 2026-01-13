//! Example: Parse Trade Confirmation FLEX statement
//!
//! Trade Confirmation FLEX statements contain real-time trade execution data
//! and are updated immediately after each trade (unlike Activity FLEX which
//! is typically generated end-of-day).
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example parse_trade_confirmation
//! ```

use ib_flex::{parse_trade_confirmation, AssetCategory, BuySell};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample Trade Confirmation FLEX XML
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TradeConfirmationStatement accountId="U1234567">
    <Trades>
        <Trade
            accountId="U1234567"
            symbol="AAPL"
            conid="265598"
            assetCategory="STK"
            description="APPLE INC"
            tradeDate="2025-01-15"
            dateTime="2025-01-15;093015"
            settleDateTarget="2025-01-17"
            buySell="BUY"
            quantity="100"
            price="185.50"
            amount="-18550.00"
            proceeds="-18550.00"
            ibCommission="-1.00"
            ibCommissionCurrency="USD"
            netCash="-18551.00"
            currency="USD"
            fxRateToBase="1"
            multiplier="1"
            transactionID="567890123"
        />
        <Trade
            accountId="U1234567"
            symbol="MSFT"
            conid="272093"
            assetCategory="STK"
            description="MICROSOFT CORP"
            tradeDate="2025-01-15"
            dateTime="2025-01-15;140230"
            settleDateTarget="2025-01-17"
            buySell="SELL"
            quantity="-50"
            price="415.25"
            amount="20762.50"
            proceeds="20762.50"
            ibCommission="-1.00"
            ibCommissionCurrency="USD"
            netCash="20761.50"
            currency="USD"
            fxRateToBase="1"
            multiplier="1"
            transactionID="567890124"
        />
    </Trades>
</TradeConfirmationStatement>"#;

    // Parse the Trade Confirmation FLEX statement
    let statement = parse_trade_confirmation(xml)?;

    println!("=== Trade Confirmation FLEX Statement ===\n");
    println!("Account: {}", statement.account_id);
    println!("Trades: {}\n", statement.trades.items.len());

    // Process each trade
    for (i, trade) in statement.trades.items.iter().enumerate() {
        println!("Trade {}:", i + 1);
        println!("  Symbol: {}", trade.symbol);

        if let Some(ref desc) = trade.description {
            println!("  Description: {}", desc);
        }

        println!("  Asset Category: {:?}", trade.asset_category);
        println!("  Trade Date: {}", trade.trade_date);

        // Trade direction
        if let Some(ref buy_sell) = trade.buy_sell {
            match buy_sell {
                BuySell::Buy | BuySell::CancelBuy => println!("  Direction: BUY"),
                BuySell::Sell | BuySell::CancelSell => println!("  Direction: SELL"),
                _ => println!("  Direction: UNKNOWN"),
            }
        }

        // Quantities and prices
        if let Some(quantity) = trade.quantity {
            println!("  Quantity: {}", quantity);
        }
        if let Some(price) = trade.price {
            println!("  Price: ${}", price);
        }
        println!("  Commission: ${}", trade.commission);
        println!("  Currency: {}", trade.currency);

        println!();
    }

    // Calculate summary statistics
    let total_buys = statement
        .trades
        .items
        .iter()
        .filter(|t| matches!(t.buy_sell, Some(BuySell::Buy)))
        .count();

    let total_sells = statement
        .trades
        .items
        .iter()
        .filter(|t| matches!(t.buy_sell, Some(BuySell::Sell)))
        .count();

    let total_commission: rust_decimal::Decimal =
        statement.trades.items.iter().map(|t| t.commission).sum();

    println!("=== Summary ===");
    println!("Total Buys: {}", total_buys);
    println!("Total Sells: {}", total_sells);
    println!("Total Commission: ${}", total_commission);

    // Asset class breakdown
    println!("\n=== By Asset Class ===");
    let stock_trades = statement
        .trades
        .items
        .iter()
        .filter(|t| t.asset_category == AssetCategory::Stock)
        .count();
    let option_trades = statement
        .trades
        .items
        .iter()
        .filter(|t| t.asset_category == AssetCategory::Option)
        .count();

    println!("Stocks: {}", stock_trades);
    println!("Options: {}", option_trades);

    Ok(())
}
