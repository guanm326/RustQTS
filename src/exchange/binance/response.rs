use serde::{Deserialize, Serialize};

/*
Binance Order Book Response
https://binance-docs.github.io/apidocs/futures/en/#order-book
*/

#[derive(Deserialize, Debug)]
pub struct BinanceOrderbookResponse {
    /*
    https://binance-docs.github.io/apidocs/futures/en/#order-book
     */
    pub lastUpdateId: u64,
    pub E: u64,  // Message output time
    pub T: u64,     // Transaction time
    pub bids: Vec<[String; 2]>,    // Bids array - [price, qty]
    pub asks: Vec<[String; 2]>,    // Asks array - [price, qty]
}

/*
Binance 24hr Ticker Price Change Statistics
https://binance-docs.github.io/apidocs/futures/en/#24hr-ticker-price-change-statistics
*/

#[derive(Deserialize, Debug)]
pub struct BinanceTickerResponse {
    /*
    Single ticker response when symbol is provided
    https://binance-docs.github.io/apidocs/futures/en/#24hr-ticker-price-change-statistics
     */
    pub symbol: String,
    pub priceChange: String,
    pub priceChangePercent: String,
    pub weightedAvgPrice: String,
    pub lastPrice: String,
    pub lastQty: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub volume: String,
    pub quoteVolume: String,
    pub openTime: u64,
    pub closeTime: u64,
    pub firstId: u64,   // First tradeId
    pub lastId: u64,    // Last tradeId
    pub count: u64      // Trade count
}

#[derive(Deserialize, Debug)]
pub struct BinanceTickersResponse {
    /*
    Multiple tickers response when symbol is not provided
    https://binance-docs.github.io/apidocs/futures/en/#24hr-ticker-price-change-statistics
     */
    pub list: Vec<BinanceTickerItem>
}

#[derive(Deserialize, Debug)]
pub struct BinanceTickerItem {
    /*
    Individual ticker item in the list response
    https://binance-docs.github.io/apidocs/futures/en/#24hr-ticker-price-change-statistics
     */
    pub symbol: String,
    pub priceChange: String,
    pub priceChangePercent: String,
    pub weightedAvgPrice: String,
    pub lastPrice: String,
    pub lastQty: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub volume: String,
    pub quoteVolume: String,
    pub openTime: u64,
    pub closeTime: u64,
    pub firstId: u64,   // First tradeId
    pub lastId: u64,    // Last tradeId
    pub count: u64      // Trade count
}
