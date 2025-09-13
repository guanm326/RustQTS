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

/*
Binance Position Information V3 (USER_DATA)
https://binance-docs.github.io/apidocs/futures/en/#position-information-v3-user_data
*/

#[derive(Deserialize, Debug)]
pub struct BinancePositionResponse {
    /*
    Position information response
    https://binance-docs.github.io/apidocs/futures/en/#position-information-v3-user_data
     */
    pub list: Vec<BinancePositionItem>
}

#[derive(Deserialize, Debug)]
pub struct BinancePositionItem {
    /*
    Individual position item
    https://binance-docs.github.io/apidocs/futures/en/#position-information-v3-user_data
     */
    pub symbol: String,
    pub positionSide: String,               // position side (BOTH, LONG, SHORT)
    pub positionAmt: String,                // position amount
    pub entryPrice: String,                 // entry price
    pub breakEvenPrice: String,             // break even price
    pub markPrice: String,                  // mark price
    pub unRealizedProfit: String,           // unrealized profit
    pub liquidationPrice: String,           // liquidation price
    pub isolatedMargin: String,             // isolated margin
    pub notional: String,                   // notional value
    pub marginAsset: String,                // margin asset
    pub isolatedWallet: String,             // isolated wallet
    pub initialMargin: String,              // initial margin required with current mark price
    pub maintMargin: String,                // maintenance margin required
    pub positionInitialMargin: String,      // initial margin required for positions with current mark price
    pub openOrderInitialMargin: String,     // initial margin required for open orders with current mark price
    pub adl: u64,                          // auto deleveraging indicator
    pub bidNotional: String,                // bids notional, ignore
    pub askNotional: String,                // ask notional, ignore
    pub updateTime: u64                    // update time
}

/*
Binance WebSocket API Response Structures
https://binance-docs.github.io/apidocs/futures/en/#websocket-api-general-info
*/

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceWebSocketResponse {
    /*
    Generic WebSocket response structure
    https://binance-docs.github.io/apidocs/futures/en/#websocket-api-request-format
     */
    pub id: Option<String>,
    pub status: u16,
    pub result: Option<serde_json::Value>,
    pub error: Option<BinanceWebSocketError>,
    pub rateLimits: Option<Vec<BinanceRateLimit>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceWebSocketError {
    /*
    WebSocket error response
    https://binance-docs.github.io/apidocs/futures/en/#websocket-api-request-format
     */
    pub code: i32,
    pub msg: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceRateLimit {
    /*
    Rate limit information
    https://binance-docs.github.io/apidocs/futures/en/#websocket-api-rate-limits
     */
    pub rateLimitType: String,
    pub interval: String,
    pub intervalNum: u32,
    pub limit: u32,
    pub count: u32,
}
