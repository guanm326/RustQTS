use serde::{Deserialize, Serialize};


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
