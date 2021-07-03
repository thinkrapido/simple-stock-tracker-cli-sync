
mod tests;
mod types;

use crate::types::*;
use clap::{Arg, App};
use chrono::prelude::*;
use chrono::Duration;
use yahoo_finance_api as yahoo;

fn main() {

    let matches = App::new("finance reader sync")
        .version("1.0")
        .author("Romeo Disca <romeo.disca@gmail.com")
        .about("finance reader sync")
        .arg(Arg::new("symbol")
            .about("the stock symbol to be read")
            .index(1)
            .required(true)
        )        
        .arg(Arg::new("days_back")
            .about("how many days should we look back")
            .short('d')
            .long("days")
            .default_value("0")
            .takes_value(true)
        )
        .arg(Arg::new("months_back")
            .about("how many months should we look back")
            .short('m')
            .long("months")
            .default_value("0")
            .takes_value(true)
        )
        .arg(Arg::new("years_back")
            .about("how many years should we look back")
            .short('y')
            .long("years")
            .default_value("0")
            .takes_value(true)
        )
        .get_matches()
        ;

    let today = Utc::today();
    let mut passed_date = today.clone();
    let mut symbol = "UNKNOWN";

    if let Some(s) = matches.value_of("symbol") {
        symbol = s;
    }
    if let Some(d) = matches.value_of("days_back") {
        passed_date = passed_date.checked_add_signed(Duration::days(-d.parse::<i64>().unwrap())).unwrap();
    }
    if let Some(d) = matches.value_of("months_back") {
        passed_date = add_months(&passed_date, -d.parse::<i32>().unwrap());
    }
    if let Some(d) = matches.value_of("years_back") {
        passed_date = add_years(&passed_date, -d.parse::<i32>().unwrap());
    }

    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history_interval(symbol, passed_date.and_hms(0,0,0), today.and_hms(0,0,0), "1d");
    let quotes = response.unwrap().quotes().unwrap();
    let quotes = quotes.into_iter().map(|quote| StockData::new(symbol.to_string(), Utc.timestamp(quote.timestamp as i64, 0)).close(quote.adjclose.into()));
    println!("\n\n{}'s quotes of the period: {} to {}", symbol, passed_date.format("%Y-%m-%d"), today.format("%Y-%m-%d"));
    for quote in quotes {
        println!("{:?}", quote);
    }

}

fn add_years(date: &Date<Utc>, years: i32) -> Date<Utc>{
    Utc.ymd(date.year() + years, date.month(), date.day())
}
fn add_months(date: &Date<Utc>, months: i32) -> Date<Utc>{
    let mut months = (date.month() as i32 + months) % 12;
    let mut years = months / 12;
    if months < 0 {
        months += 12;
        years -= 1;
    }
    Utc.ymd(date.year() + years, months as u32, date.day())
}
