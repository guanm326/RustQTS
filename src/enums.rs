

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StrategyTypes {
    #[serde(rename = "SPREAD")]
    Spread,
    #[serde(rename = "EE")]
    EE,
    #[serde(rename = "PEE")]
    PEE,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Exchanges {
    #[serde(rename = "bybit")]
    Bybit,
    #[serde(rename = "binance")]
    Binance,
    #[serde(rename = "okx")]
    Okx,
    #[serde(rename = "bitget")]
    Bitget,
    #[serde(rename = "hyperliquid")]
    Hyperliquid
}



#[derive(Debug, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "PERP")]
    Perp,
    #[serde(rename = "LINEAR")]
    Linear,
    #[serde(rename = "INVERSE")]
    Inverse,
    #[serde(rename = "FUTURES")]
    Futures,
    #[serde(rename = "OPTIONS")]
    Options
}
