extern crate yahoo_finance;
extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};
use yahoo_finance::{history};

fn main()
{
    /*
     * ams_ahold,AD.AS
     * ams_besi,BESI.AS
     * ams_boka,BOKA.AS
     * EUR,EUR=X
     * USD,EURUSD=X
     */
    // TODO: make sure the 2008 does not have to be copy/pasted
    // Also make a translation for the ticker symbols
    // How to get the EUR values? Get the EURO price too? EUR=X
    retrieve_and_print("BESI.AS", 2008);
    retrieve_and_print("EXM.BR", 2008);
    retrieve_and_print("EUR=X", 2008);
    retrieve_and_print("EURUSD=X", 2008);
}

fn retrieve_and_print(asymbol: &str, ayear: i32)
{
    let start: DateTime<Utc> = Utc.ymd(ayear, 1, 1).and_hms(0, 0, 0);
    let end: DateTime<Utc> = Utc.ymd(ayear, 12, 31).and_hms(0, 0, 0);
    // let end = Utc::now();
    let data = history::retrieve_range(asymbol, start, Some(end)).unwrap();

    // print the date and closing price for each day we have data
    for bar in &data
    {
       println!("P {} {} {:.2} USD", bar.timestamp.format("%Y-%m-%d"), asymbol, bar.close)
    }

}
