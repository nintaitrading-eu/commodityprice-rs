use chrono::{DateTime, TimeZone, Utc};
use chrono::prelude::{Datelike};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::YResponse;
use docopt::Docopt;
use tokio_test;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Commodityprice

Usage:
    commodityprice --tickers=<json> [--year=<year>]
    commodityprice (-h | --help)
    commodityprice --version

Options:
    --tickers=<json>  Json file with ticker symbols to download.
    [--year=<year>]  Get commodity prices for the given year. If no year is given, the current year is used.
    -h --help  Show this screen.
    --version  Show version.
";

#[derive(Deserialize, Debug)]
struct Ticker
{
    yahoo: String,
    local: String,
    currency: String,
    active: bool,
}

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version")
    {
        println!("commodityprice v{}", VERSION);
        std::process::exit(0);
    }

    let current_year: i32 = Utc::now().year();
    let year = match args.get_str("--year").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => current_year,
    };

    let json = args.get_str("--tickers");
    if !(json.len() > 0) || !Path::new(json).exists()
    {
        println!("File {} not found.", json);
        std::process::exit(1);
    };

    let tickers_json = match File::open(json)
    {
        Ok(file) => file,
        Err(_) =>
        {
            println!("Error: Could not open file {}.", json);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(tickers_json);
    let tickers: Vec<Ticker> = match serde_json::from_reader(reader)
    {
        Ok(data) => data,
        Err(_) =>
        {
            println!("Error: Could not parse json from reader.");
            std::process::exit(1);
        }
    };

    for ticker in tickers.iter()
    {
        process(ticker, year)
    }
}

fn process(aticker: &Ticker, ayear: i32)
{
    if !(aticker.active)
    {
        return;
    }
    retrieve(aticker, ayear);
}

fn retrieve(aticker: &Ticker, ayear: i32)
{
    let start: DateTime<Utc> = Utc.ymd(ayear, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end: DateTime<Utc> = Utc.ymd(ayear, 12, 31).and_hms_milli(23, 59, 59, 999);
    let provider = yahoo::YahooConnector::new();

    match tokio_test::block_on(provider.get_quote_history(aticker.yahoo.as_str(), start, end))
    {
        Ok(t) => print(aticker, t),
        Err(_) => () // Ignore exceptions
    };
}

fn print(aticker: &Ticker, adata: YResponse)
{
    // print the ledger price database line for each day we got a price from the api.
    for item in &adata.quotes().unwrap()
    {
       let formatted_timestamp = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(item.timestamp)).format("%Y-%m-%d");
       println!("P {} {} {:.2} {}", formatted_timestamp, aticker.local, item.close, aticker.currency)
    }
}
