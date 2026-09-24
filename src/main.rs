use chrono::{DateTime, Utc, Datelike};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::YResponse;
use docopt::Docopt;
use tokio_test;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use time::{macros::datetime, OffsetDateTime};

const VERSION: &'static str = "0.2.1";
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
    let args = match Docopt::new(USAGE).and_then(|dopt| dopt.parse())
    {
        Ok(args) => args,
        Err(_) => std::process::exit(1),
    };

    let mut logfile = create_log_file();

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
        log_message(&mut logfile, format!("Ticker file not found: {}", json));
        std::process::exit(1);
    };

    let tickers_json = match File::open(json)
    {
        Ok(file) => file,
        Err(_) =>
        {
            log_message(&mut logfile, format!("Could not open ticker file: {}", json));
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(tickers_json);
    let tickers: Vec<Ticker> = match serde_json::from_reader(reader)
    {
        Ok(data) => data,
        Err(_) =>
        {
            log_message(&mut logfile, "Could not parse ticker JSON.");
            std::process::exit(1);
        }
    };

    for ticker in tickers.iter()
    {
        process(ticker, year, &mut logfile)
    }
}

fn process(aticker: &Ticker, ayear: i32, alogfile: &mut Option<File>)
{
    if !(aticker.active)
    {
        return;
    }
    retrieve(aticker, ayear, alogfile);
}

fn retrieve(aticker: &Ticker, ayear: i32, alogfile: &mut Option<File>)
{
    let start: OffsetDateTime = datetime!(2000-1-1 0:00 UTC).replace_year(ayear).unwrap();
    let end: OffsetDateTime = datetime!(2000-12-31 0:00 UTC).replace_year(ayear).unwrap();
    let provider = match yahoo::YahooConnector::new()
    {
        Ok(provider) => provider,
        Err(e) =>
        {
            log_message(
                alogfile,
                format!("Could not create Yahoo provider for '{}': {}", aticker.yahoo, e),
            );
            return;
        }
    };

    match tokio_test::block_on(provider.get_quote_history(aticker.yahoo.as_str(), start, end))
    {
        Ok(t) =>
        {
            print(aticker, t)
        },
        Err(e) => log_message(
            alogfile,
            format!("Failed to retrieve ticker '{}': {}", aticker.yahoo, e),
        ),
    };
}

fn create_log_file() -> Option<File>
{
    let log_path = format!(
        "/var/log/commodityprice_{}.log",
        Utc::now().format("%Y%m%d"),
    );
    let path = Path::new(&log_path);

    // Keep appending if this timestamped logfile already exists.
    // Otherwise a new one is created.
    let result = if path.exists()
    {
        OpenOptions::new().append(true).open(path)
    }
    else
    {
        OpenOptions::new().write(true).create(true).open(path)
    };

    result.ok()
}

fn log_message(alogfile: &mut Option<File>, message: impl AsRef<str>)
{
    if let Some(logfile) = alogfile.as_mut()
    {
        let _ = writeln!(logfile, "{} {}", Utc::now().to_rfc3339(), message.as_ref());
    }
}

fn print(aticker: &Ticker, adata: YResponse)
{
    // Print the ledger price database line for each day we got a price from the api.
    for item in &adata.quotes().unwrap()
    {
       let formatted_timestamp = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(item.timestamp)).format("%Y-%m-%d");
       println!("P {} {} {:.2} {}", formatted_timestamp, aticker.local, item.close, aticker.currency)
    }
}
