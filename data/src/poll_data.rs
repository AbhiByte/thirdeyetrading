use reqwest;
use serde_json::Value;
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use std::env;
use dotenv::dotenv;

pub async fn fetch_stock_data(symbol: &str, interval: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ALPHA_VANTAGE_API_KEY").expect("ALPHA_VANTAGE_API_KEY not set");

    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval={}&apikey={}",
        symbol, interval, api_key
    );

    let resp: Value = reqwest::get(&url).await?.json().await?;

    let time_series = resp["Time Series (1min)"].as_object().unwrap();
    let now = Utc::now();
    let date = now - chrono::Duration::days(1);

    for (timestamp, data) in time_series {
        // Parse timestamp as NaiveDateTime first, then convert to DateTime<Utc>
        let naive_date = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
        let one_day_ago: DateTime<Utc> = Utc.from_utc_datetime(&naive_date);

        if date > one_day_ago {
            println!(
                "Time: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
                timestamp,
                data["1. open"].as_str().unwrap(),
                data["2. high"].as_str().unwrap(),
                data["3. low"].as_str().unwrap(),
                data["4. close"].as_str().unwrap(),
                data["5. volume"].as_str().unwrap()
            );
        }
    }

    Ok(())
}
