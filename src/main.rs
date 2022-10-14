use std::io::{self, Write};

use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;

use crate::args::Args;
use crate::types::*;

mod args;
mod tests;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let today = Utc::today();
    let mut passed_date = Some(today);

    if let (d, Some(pd)) = (args.days_back, passed_date) {
        passed_date = pd.checked_add_signed(Duration::days(-(d as i64)));
    }
    if let (d, Some(pd)) = (args.weeks_back, passed_date) {
        passed_date = pd.checked_add_signed(Duration::days(-7 * d as i64));
    }
    if let (d, Some(pd)) = (args.months_back, passed_date) {
        passed_date = Some(add_months(&pd, -(d as i32)));
    }
    if let (d, Some(pd)) = (args.years_back, passed_date) {
        passed_date = Some(add_years(&pd, -(d as i32)));
    }

    let mut stdout = io::stdout();

    let provider = yahoo::YahooConnector::new();

    //stdout.write_all(&format!("\nquotes of the period: {} to {}\n\n", passed_date.format("%Y-%m-%d"), today.format("%Y-%m-%d")).into_bytes())?;
    write!(stdout, "period start,symbol,price,change %,min,max,30d avg")?;
    writeln!(stdout)?;

    for symbol in args.symbol {
        if let Some(pd) = passed_date {
            let response = provider
                .get_quote_history_interval(
                    &symbol,
                    pd.and_hms(0, 0, 0),
                    today.and_hms(0, 0, 0),
                    "1d",
                )
                .await?;
            let quotes = response.quotes()?;
            let quotes: Vec<StockData> = quotes
                .into_iter()
                .map(|quote: Quote| {
                    StockData::new(symbol.to_string(), Utc.timestamp(quote.timestamp as i64, 0))
                        .close(quote.adjclose.into())
                })
                .collect();
            let min: Price = quotes
                .iter()
                .fold(f32::MAX, |acc, x| acc.min(f32::from(x.close_value())))
                .into();
            let max: Price = quotes
                .iter()
                .fold(0_f32, |acc, x| acc.max(f32::from(x.close_value())))
                .into();
            let change: Percentage = (f32::from(quotes[quotes.len() - 1].close_value())
                / f32::from(quotes[0].close_value()))
            .into();

            write!(stdout, "{},", pd.and_hms(0, 0, 0).to_rfc3339())?;
            write!(stdout, "{}", symbol)?;
            write!(
                stdout,
                "{},",
                quotes[quotes.len() - 1].close_value().to_string()
            )?;
            write!(stdout, "{},", change.to_string())?;
            write!(stdout, "{},", min.to_string())?;
            write!(stdout, "{}", max.to_string())?;
            {
                let len = quotes.len();
                if len >= 30 {
                    let sma_30 = quotes[len - 30..]
                        .iter()
                        .fold(0_f32, |acc, x| acc + f32::from(x.close_value()) / 30_f32);
                    write!(stdout, ",{}", sma_30)?;
                }
            }
            writeln!(stdout)?;
        }
    }

    Ok(())
}

fn add_years(date: &Date<Utc>, years: i32) -> Date<Utc> {
    Utc.ymd(date.year() + years, date.month(), date.day())
}
fn add_months(date: &Date<Utc>, months: i32) -> Date<Utc> {
    let mut months = (date.month() as i32 + months) % 12;
    let mut years = months / 12;
    if months <= 0 {
        months += 12;
        years -= 1;
    }
    Utc.ymd(date.year() + years, months as u32, date.day())
}
