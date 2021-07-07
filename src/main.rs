
mod tests;
mod types;

use crate::types::*;
use clap::{Arg, App};
use chrono::prelude::*;
use chrono::Duration;
use yahoo_finance_api as yahoo;
use std::io::{self, Write};

fn main() -> io::Result<()>{

    let matches = App::new("finance reader sync")
        .version("1.0")
        .author("Romeo Disca <romeo.disca@gmail.com")
        .about("finance reader sync")
        .arg(Arg::new("symbol")
            .about("the stock symbol to be read")
            .min_values(1)
            .multiple_occurrences(true)
            .required(true)
        )        
        .arg(Arg::new("days_back")
            .about("how many days should we look back")
            .short('d')
            .long("days")
            .default_value("0")
            .takes_value(true)
        )
        .arg(Arg::new("weeks_back")
            .about("how many days should we look back")
            .short('w')
            .long("weeks")
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

    if let Some(d) = matches.value_of("days_back") {
        passed_date = passed_date.checked_add_signed(Duration::days(-d.parse::<i64>().unwrap())).unwrap();
    }
    if let Some(d) = matches.value_of("weeks_back") {
        passed_date = passed_date.checked_add_signed(Duration::days(-7 * d.parse::<i64>().unwrap())).unwrap();
    }
    if let Some(d) = matches.value_of("months_back") {
        passed_date = add_months(&passed_date, -d.parse::<i32>().unwrap());
    }
    if let Some(d) = matches.value_of("years_back") {
        passed_date = add_years(&passed_date, -d.parse::<i32>().unwrap());
    }

    let mut stdout = io::stdout();

    let provider = yahoo::YahooConnector::new();
    
    //stdout.write_all(&format!("\nquotes of the period: {} to {}\n\n", passed_date.format("%Y-%m-%d"), today.format("%Y-%m-%d")).into_bytes())?;
    stdout.write_all(&format!("period start,symbol,price,change %,min,max,30d avg\n").into_bytes())?;
    
    for symbol in matches.values_of("symbol").unwrap() {
        let response = provider.get_quote_history_interval(symbol, passed_date.and_hms(0,0,0), today.and_hms(0,0,0), "1d");
        let quotes = response.unwrap().quotes().unwrap();
        let quotes: Vec<_> = quotes.into_iter().map(|quote| StockData::new(symbol.to_string(), Utc.timestamp(quote.timestamp as i64, 0)).close(quote.adjclose.into())).collect();
        let min: Price = quotes.iter().fold(f32::MAX, |acc, x| acc.min(f32::from(x.close_value()))).into();
        let max: Price = quotes.iter().fold(0_f32, |acc, x| acc.max(f32::from(x.close_value()))).into();
        let change: Percentage = (f32::from(quotes[quotes.len() - 1].close_value()) / f32::from(quotes[0].close_value())).into();
        let sma_30: Option<Price> = match quotes.len() {
            len if len >= 30 => Some((quotes[quotes.len() - 30 ..].iter().fold(0_f32, |acc, x| acc + f32::from(x.close_value())) / 30_f32).into()),
            _ => None,
        };

        stdout.write_all(&format!("{},", passed_date.and_hms(0,0,0).to_rfc3339()).into_bytes())?;
        stdout.write_all(&format!("{},", symbol).into_bytes())?;
        stdout.write_all(&format!("{},", quotes[quotes.len() - 1].close_value().to_string()).into_bytes())?;
        stdout.write_all(&format!("{},", change.to_string()).into_bytes())?;
        stdout.write_all(&format!("{},", min.to_string()).into_bytes())?;
        stdout.write_all(&format!("{},", max.to_string()).into_bytes())?;
        stdout.write_all(&format!("{}", format(sma_30)).into_bytes())?;
        stdout.write_all(&format!("\n").into_bytes())?;
    }

    Ok(())
}

fn add_years(date: &Date<Utc>, years: i32) -> Date<Utc>{
    Utc.ymd(date.year() + years, date.month(), date.day())
}
fn add_months(date: &Date<Utc>, months: i32) -> Date<Utc>{
    let mut months = (date.month() as i32 + months) % 12;
    let mut years = months / 12;
    if months <= 0 {
        months += 12;
        years -= 1;
    }
    Utc.ymd(date.year() + years, months as u32, date.day())
}
