use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_parser = clap::value_parser!(String))]
    pub symbol: Vec<String>,

    #[arg(short, long = "days", default_value_t = 0)]
    pub days_back: usize,

    #[arg(short, long = "weeks", default_value_t = 0)]
    pub weeks_back: usize,

    #[arg(short, long = "months", default_value_t = 0)]
    pub months_back: usize,

    #[arg(short, long = "years", default_value_t = 0)]
    pub years_back: usize,
}
