use chrono::{DateTime, TimeZone, Utc};
use chrono::prelude::{Datelike};
use yahoo_finance::{history, Bar};
use docopt::Docopt;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Commodityprice

Usage:
    commodityprice [--year=<year>]
    commodityprice (-h | --help)
    commodityprice --version

Options:
    [--year=<year>]   Get commodity prices for the given year. If no year is given, the current year is used.
    -h --help  Show this screen.
    --version  Show version.
";

struct Ticker<'a>
{
    yahoo: &'a str,
    local: &'a str,
    currency: &'a str,
    active: bool,
}

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version")
    {
        println!("Ledgerexport-tax v{}", VERSION);
        std::process::exit(0);
    }

    let current_year: i32 = Utc::now().year();
    let year = match args.get_str("--year").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => current_year,
    };


    let tickers: Vec<Ticker> = vec![
        Ticker {yahoo: "AD.AS", local: "ams_ad", currency: "EUR", active: false},
        Ticker {yahoo: "BESI.AS", local: "ams_besi", currency: "EUR", active: true},
        Ticker {yahoo: "BOKA.AS", local: "ams_boka", currency: "EUR", active: true},
        Ticker {yahoo: "CRXL.AS", local: "ams_crxl", currency: "EUR", active: false},
        Ticker {yahoo: "DRAK.AS", local: "ams_drak", currency: "EUR", active: false},
        Ticker {yahoo: "FUR.AS", local: "ams_fur", currency: "EUR", active: false},
        Ticker {yahoo: "SBMO.AS", local: "ams_sbm", currency: "EUR", active: false},
        Ticker {yahoo: "SR.AS", local: "ams_sr", currency: "EUR", active: false},
        Ticker {yahoo: "ABO.BR", local: "ebr_abo", currency: "EUR", active: false},
        Ticker {yahoo: "COFB.BR", local: "ebr_cofb", currency: "EUR", active: true},
        Ticker {yahoo: "DEVG.BR", local: "ebr_devg", currency: "EUR", active: false},
        Ticker {yahoo: "DEXB.BR", local: "ebr_dexb", currency: "EUR", active: false},
        Ticker {yahoo: "ENIN.BR", local: "ebr_enin", currency: "EUR", active: false},
        Ticker {yahoo: "EURN.BR", local: "ebr_eurn", currency: "EUR", active: false},
        Ticker {yahoo: "EXM.BR", local: "ebr_exm", currency: "EUR", active: true},
        Ticker {yahoo: "NESTS.BR", local: "ebr_nests", currency: "EUR", active: false},
        Ticker {yahoo: "RHII.BR", local: "ebr_rhii", currency: "EUR", active: false},
        Ticker {yahoo: "SOLB.BR", local: "ebr_solb", currency: "EUR", active: false},
        Ticker {yahoo: "TESB.BR", local: "ebr_tess", currency: "EUR", active: false},
        Ticker {yahoo: "THEB.BR", local: "ebr_theb", currency: "EUR", active: false},
        Ticker {yahoo: "TNET.BR", local: "ebr_tnet", currency: "EUR", active: false},
        Ticker {yahoo: "CA.PA", local: "epa_car", currency: "EUR", active: false},
        Ticker {yahoo: "ENGI.PA", local: "epa_gsz", currency: "EUR", active: true},
        Ticker {yahoo: "?", local: "etr_fme", currency: "EUR", active: false},
        Ticker {yahoo: "EUR=X", local: "USD", currency: "EUR", active: true},
        Ticker {yahoo: "EURUSD=X", local: "EUR", currency: "USD", active: true},
        Ticker {yahoo: "ADA-EUR", local: "ADA", currency: "EUR", active: true},
        Ticker {yahoo: "BTC-EUR", local: "BTC", currency: "EUR", active: true},
        Ticker {yahoo: "XRP-EUR", local: "XRP", currency: "EUR", active: true},
        Ticker {yahoo: "PHAU.AS", local: "etfs_phau", currency: "EUR", active: true},
        /* Note: PHAG.MI is incorrect, it is from AS, but no ticker symbol available. It follows the same price though. */
        Ticker {yahoo: "PHAG.MI", local: "etfs_psil", currency: "EUR", active: true},
    ];
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
    let start: DateTime<Utc> = Utc.ymd(ayear, 1, 1).and_hms(0, 0, 0);
    let end: DateTime<Utc> = Utc.ymd(ayear, 12, 31).and_hms(0, 0, 0);
    //let data = history::retrieve_range(aticker.yahoo, start, Some(end)).unwrap();
    match history::retrieve_range(aticker.yahoo, start, Some(end))
    {
        Ok(t) => print(aticker, t),
        Err(_) => () // Ignore exceptions
    };
}

fn print(aticker: &Ticker, adata: Vec<Bar>)
{
    // print the ledger price database line for each day we got a price from the api.
    for bar in &adata
    {
       println!("P {} {} {:.2} {}", bar.timestamp.format("%Y-%m-%d"), aticker.local, bar.close, aticker.currency)
    }

}
