mod read_data;
mod poll_data;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let symbol = "IBM";  // You can change this to any stock symbol you're interested in
    let interval = "1min";  // You can change this to "5min", "15min", "30min", or "60min"

    println!("Fetching stock data for {} with {} interval...", symbol, interval);
    
    match poll_data::fetch_stock_data(symbol, interval).await {
        Ok(_) => println!("Stock data fetched successfully!"),
        Err(e) => eprintln!("Error fetching stock data: {}", e),
    }

    Ok(())
}