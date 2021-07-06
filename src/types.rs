
use chrono::prelude::*;

#[derive(Default, Copy, Clone, Debug)]
pub struct Price {
    value: f32,
}
impl From<f32> for Price {
    fn from(value: f32) -> Self {
        Price{ value }
    }
}
impl From<f64> for Price {
    fn from(value: f64) -> Self {
        Price{ value: value as f32 }
    }
}
impl ToString for Price {

    fn to_string(&self) -> String {
        format!("${:.2}", self.value)
    }

}
impl From<Price> for f32 {
    fn from(p: Price) -> Self {
        p.value
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Percentage {
    value: f32,
}
impl From<f32> for Percentage {
    fn from(value: f32) -> Self {
        Percentage{ value: (value - 1_f32) * 100_f32 }
    }
}
impl From<f64> for Percentage {
    fn from(value: f64) -> Self {
        (value as f32).into()
    }
}
impl ToString for Percentage {

    fn to_string(&self) -> String {
        format!("{:.2}%", self.value)
    }

}

#[derive(Copy, Clone, Debug)]
pub struct Timestamp {
    datetime: DateTime<Utc>,
}
impl Default for Timestamp {

    fn default() -> Self {
        Timestamp{ datetime: Utc::now(), }
    }

}
impl ToString for Timestamp {

    fn to_string(&self) -> String {
        self.datetime.to_rfc3339()
    }

}

#[derive(Default, Clone, Debug)]
pub struct StockData {
    datetime: Timestamp,
    stock_symbol: String,
    close: Option<Price>,
}
impl StockData {

    pub fn new(stock_symbol: String, datetime: DateTime<Utc>) -> Self {
        StockData::new_now(stock_symbol).datetime(datetime)
    }
    pub fn new_now(stock_symbol: String) -> Self {
        StockData::default().stock_symbol(stock_symbol)
    }

    pub fn datetime(&self, datetime: DateTime<Utc>) -> Self {
        StockData { datetime: Timestamp { datetime }, ..self.clone() }
    }

    pub fn stock_symbol(&self, stock_symbol: String) -> Self {
        StockData { stock_symbol, ..self.clone() }
    }

    pub fn close(&self, close: Price) -> Self {
        StockData { close: Some(close), ..self.clone() }
    }
    pub fn close_value(&self) -> Price {
        match self.close {
            Some(p) => p,
            None => 0_f32.into(),
        }
    }

}
impl ToString for StockData {
    fn to_string(&self) -> String {
        format!("{},{},{}", 
                self.datetime.to_string(),
                self.stock_symbol,
                format(self.close),
            )
    }
}

pub fn format<Val: ToString>(value: Option<Val>) -> String {
    match value {
        Some(s) => s.to_string(),
        None => "-".to_string(),
    }    
}