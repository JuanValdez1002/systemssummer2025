use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Utc};

// Trait for pricing functionality
trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>>;
    fn get_name(&self) -> &str;
}

// Bitcoin struct
#[derive(Debug)]
struct Bitcoin;

// Ethereum struct
#[derive(Debug)]
struct Ethereum;

// S&P 500 struct
#[derive(Debug)]
struct SP500;

// Response structures for API parsing
#[derive(Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Option<CoinPrice>,
    ethereum: Option<CoinPrice>,
}

#[derive(Deserialize)]
struct CoinPrice {
    usd: f64,
}

#[derive(Deserialize)]
struct YahooFinanceResponse {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Deserialize)]
struct ChartResult {
    meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: f64,
}

// Implement Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        println!("Fetching Bitcoin price...");
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()?;
        
        let body: CoinGeckoResponse = response.into_json()?;
        
        match body.bitcoin {
            Some(coin) => Ok(coin.usd),
            None => Err("Bitcoin price not found in response".into()),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp: DateTime<Utc> = Utc::now();
        let data = format!("{}: ${:.2}\n", timestamp.format("%Y-%m-%d %H:%M:%S UTC"), price);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin_prices.txt")?;
        
        file.write_all(data.as_bytes())?;
        println!("Bitcoin price ${:.2} saved to bitcoin_prices.txt", price);
        Ok(())
    }

    fn get_name(&self) -> &str {
        "Bitcoin"
    }
}

// Implement Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        println!("Fetching Ethereum price...");
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()?;
        
        let body: CoinGeckoResponse = response.into_json()?;
        
        match body.ethereum {
            Some(coin) => Ok(coin.usd),
            None => Err("Ethereum price not found in response".into()),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp: DateTime<Utc> = Utc::now();
        let data = format!("{}: ${:.2}\n", timestamp.format("%Y-%m-%d %H:%M:%S UTC"), price);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ethereum_prices.txt")?;
        
        file.write_all(data.as_bytes())?;
        println!("Ethereum price ${:.2} saved to ethereum_prices.txt", price);
        Ok(())
    }

    fn get_name(&self) -> &str {
        "Ethereum"
    }
}

// Implement Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        println!("Fetching S&P 500 price...");
        
        let response = ureq::get("https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC")
            .call()?;
        
        let body: YahooFinanceResponse = response.into_json()?;
        
        if let Some(result) = body.chart.result.first() {
            Ok(result.meta.regular_market_price)
        } else {
            Err("S&P 500 price not found in response".into())
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp: DateTime<Utc> = Utc::now();
        let data = format!("{}: ${:.2}\n", timestamp.format("%Y-%m-%d %H:%M:%S UTC"), price);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sp500_prices.txt")?;
        
        file.write_all(data.as_bytes())?;
        println!("S&P 500 price ${:.2} saved to sp500_prices.txt", price);
        Ok(())
    }

    fn get_name(&self) -> &str {
        "S&P 500"
    }
}

// Function to fetch and save price for a given asset
fn fetch_and_save_price(asset: &dyn Pricing) {
    match asset.fetch_price() {
        Ok(price) => {
            if let Err(e) = asset.save_to_file(price) {
                eprintln!("❌ Error saving {} price: {}", asset.get_name(), e);
            }
        }
        Err(e) => {
            if e.to_string().contains("429") {
                eprintln!("⚠️  Rate limited for {}: Too many requests, will retry next cycle", asset.get_name());
            } else {
                eprintln!("❌ Error fetching {} price: {}", asset.get_name(), e);
            }
        }
    }
}

fn main() {
    println!("🚀 Starting Financial Data Fetcher");
    println!("Fetching Bitcoin, Ethereum, and S&P 500 prices every 30 seconds...");
    println!("Press Ctrl+C to stop the program\n");

    // Create vector of assets implementing the Pricing trait
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    // Main data fetching loop
    loop {
        println!("📊 Fetching prices at {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        
        // Fetch and save price for each asset
        for asset in &assets {
            fetch_and_save_price(asset.as_ref());
        }
        
        println!("✅ Fetch cycle completed. Waiting 30 seconds...\n");
        
        // Wait 30 seconds before next fetch
        thread::sleep(Duration::from_secs(30));
    }
}
