extern crate yahoo_finance;
extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};
use yahoo_finance::{history, Bar};


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
        Ticker {yahoo: "CRXL.AS", local: "ams_crxl", currency: "EUR", active: false},
        Ticker {yahoo: "DRAK.AS", local: "ams_drak", currency: "EUR", active: false},
        Ticker {yahoo: "FUR.AS", local: "ams_fur", currency: "EUR", active: true},
        Ticker {yahoo: "SBMO.AS", local: "ams_sbm", currency: "EUR", active: true},
        Ticker {yahoo: "SR.AS", local: "ams_sr", currency: "EUR", active: false},
        Ticker {yahoo: "ABO.BR", local: "ebr_abo", currency: "EUR", active: true},
        Ticker {yahoo: "COFB.BR", local: "ebr_cofb", currency: "EUR", active: true},
        Ticker {yahoo: "DEVG.BR", local: "ebr_devg", currency: "EUR", active: false},
        Ticker {yahoo: "DEXB.BR", local: "ebr_dexb", currency: "EUR", active: false},
        Ticker {yahoo: "ENIN.BR", local: "ebr_enin", currency: "EUR", active: false},
        Ticker {yahoo: "EURN.BR", local: "ebr_eurn", currency: "EUR", active: true},
        Ticker {yahoo: "EXM.BR", local: "ebr_exm", currency: "EUR", active: true},
        Ticker {yahoo: "NESTS.BR", local: "ebr_nests", currency: "EUR", active: false},
        Ticker {yahoo: "RHII.BR", local: "ebr_rhii", currency: "EUR", active: false},
        Ticker {yahoo: "SOLB.BR", local: "ebr_solb", currency: "EUR", active: true},
        Ticker {yahoo: "TESB.BR", local: "ebr_tess", currency: "EUR", active: true},
        Ticker {yahoo: "THEB.BR", local: "ebr_theb", currency: "EUR", active: false},
        Ticker {yahoo: "TNET.BR", local: "ebr_tnet", currency: "EUR", active: true},
        Ticker {yahoo: "CA.PA", local: "epa_car", currency: "EUR", active: true},
        Ticker {yahoo: "ENGI.PA", local: "epa_gsz", currency: "EUR", active: true},
        Ticker {yahoo: "?", local: "etr_fme", currency: "EUR", active: false},
        Ticker {yahoo: "EUR=X", local: "USD", currency: "EUR", active: true},
        Ticker {yahoo: "EURUSD=X", local: "EUR", currency: "USD", active: true},
        /* Note: crypto panics for years it didn't exist yet. */
        Ticker {yahoo: "ADA-EUR", local: "ADA", currency: "EUR", active: true},
        Ticker {yahoo: "BTC-EUR", local: "BTC", currency: "EUR", active: true},
        Ticker {yahoo: "XRP-EUR", local: "XRP", currency: "EUR", active: true}
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
        Err(e) => () // Ignore exceptions
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
