use serde::{Deserialize, Serialize};
use crate::enums::{InstrumentType, Exchanges};


pub struct QTSOrderBook{
    pub symbol: String,
    pub asks: Vec<(String,String)>,
    pub bids: Vec<(String,String)>,
    pub time:i64,
}

#[derive(Deserialize)]
pub struct APIKey {
    pub api_name: String,
    pub api_key: String,
    pub api_secret: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symbol{
    pub symbol: String,
    pub exchange: Exchanges,
    pub instrument_type: InstrumentType,
    pub min_tick: f64,
    pub qty_decimal: u32,
}


