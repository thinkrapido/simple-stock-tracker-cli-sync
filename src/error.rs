
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::num::ParseIntError;
use yahoo_finance_api as yahoo;

pub type SimpleStockTrackerResult = Result<(), SimpleStockTrackerError>;

#[derive(Debug)]
pub enum SimpleStockTrackerError {
    IoError(Error),
    ParseError(ParseIntError),
    YahooError(yahoo::YahooError),
}

impl std::error::Error for SimpleStockTrackerError {}

impl Display for SimpleStockTrackerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::IoError(e) => return e.fmt(f),
            Self::ParseError(e) => return e.fmt(f),
            Self::YahooError(e) => return e.fmt(f),
        }
     }
}

impl From<Error> for SimpleStockTrackerError {
    fn from(e: Error) -> Self { SimpleStockTrackerError::IoError(e) }
}

impl From<ParseIntError> for SimpleStockTrackerError {
    fn from(e: ParseIntError) -> Self { SimpleStockTrackerError::ParseError(e) }
}

impl From<yahoo::YahooError> for SimpleStockTrackerError {
    fn from(e: yahoo::YahooError) -> Self { SimpleStockTrackerError::YahooError(e) }
}

