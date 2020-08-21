extern crate yahoo_finance;
extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};
use yahoo_finance::{history};


struct Ticker<'a>
{
    yahoo: &'a str,
    local: &'a str,
    currency: &'a str,
    active: bool,
}

fn main()
{
    let tickers: Vec<Ticker> = vec![
        Ticker {yahoo: "AD.AS", local: "ams_ad", currency: "EUR", active: true},
        Ticker {yahoo: "BESI.AS", local: "ams_besi", currency: "EUR", active: true},
        Ticker {yahoo: "BOKA.AS", local: "ams_boka", currency: "EUR", active: true},
        Ticker {yahoo: "EXM.BR", local: "ebr_exm", currency: "EUR", active: true},
        Ticker {yahoo: "EUR=X", local: "USD", currency: "EUR", active: true},
        Ticker {yahoo: "EURUSD=X", local: "EUR", currency: "USD", active: true}
    ];
    for (i, ticker) in tickers.iter().enumerate()
    {
        // TODO: year as parameter?
        process(ticker, 2008)
    }
}

fn process(aticker: &Ticker, ayear: i32)
{
    if !(aticker.active)
    {
        return;
    }
    retrieve_and_print(aticker, ayear);
}

fn retrieve_and_print(aticker: &Ticker, ayear: i32)
{
    let start: DateTime<Utc> = Utc.ymd(ayear, 1, 1).and_hms(0, 0, 0);
    let end: DateTime<Utc> = Utc.ymd(ayear, 12, 31).and_hms(0, 0, 0);
    let data = history::retrieve_range(aticker.yahoo, start, Some(end)).unwrap();

    // print the ledger price database line for each day we got a price from the api.
    for bar in &data
    {
       println!("P {} {} {:.2} {}", bar.timestamp.format("%Y-%m-%d"), aticker.local, bar.close, aticker.currency)
    }

}
