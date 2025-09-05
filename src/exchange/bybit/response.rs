
use serde::{Deserialize, Serialize};
use serde_json::Value;


/*
Bybit Order Book
*/

#[derive(Deserialize, Debug)]
pub struct BybitOrderbookResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/market/orderbook
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitOrderbookResult,
    pub retExtInfo:Value,
    pub time: i64
    
}

#[derive(Deserialize, Debug)]
pub struct BybitOrderbookResult {
    /*
    https://bybit-exchange.github.io/docs/v5/market/orderbook
     */
    pub s: String,
    pub a: Vec<(String,String)>,
    pub b: Vec<(String,String)>,
    pub ts:i64,
    pub u: i64,
    pub seq:i64,
    pub cts: i64
}



/*
Bybit Tickers

*/


#[derive(Deserialize, Debug)]
pub struct BybitTickersResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/market/tickers
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitTickersResult,
    pub retExtInfo: Value,
    pub time: i64
}

#[derive(Deserialize, Debug)]
pub struct BybitTickersResult {
    /*
    https://bybit-exchange.github.io/docs/v5/market/tickers
     */
    pub category: String,
    pub list: Vec<BybitTickerItem>
}

#[derive(Deserialize, Debug)]
pub struct BybitTickerItem {
    /*
    https://bybit-exchange.github.io/docs/v5/market/tickers
     */
    pub symbol: String,
    pub lastPrice: String,
    pub indexPrice: String,
    pub markPrice: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub basis: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
    pub ask1Size: String,
    pub bid1Price: String,
    pub ask1Price: String,
    pub bid1Size: String,
    pub preOpenPrice: String,
    pub preQty: String,
    pub curPreListingPhase: String
}


/*
Bybit Account Info

*/


#[derive(Deserialize, Debug)]
pub struct BybitAccInfoResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/account/account-info
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitAccInfoResult
}

#[derive(Deserialize, Debug)]
pub struct BybitAccInfoResult {
    /*
    https://bybit-exchange.github.io/docs/v5/account/account-info
     */

    pub marginMode: String,
    pub updatedTime: String,
    pub unifiedMarginStatus: i32,
    pub dcpStatus: String,
    pub timeWindow: i32,
    pub smpGroup: i32,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String
}




/*
Bybit Wallet Balance

*/

#[derive(Deserialize, Debug)]
pub struct BybitWalletBalanceResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/account/wallet-balance
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitWalletBalanceResult,
    pub retExtInfo: Value,
    pub time: i64
}

#[derive(Deserialize, Debug)]
pub struct BybitWalletBalanceResult {
    /*
    https://bybit-exchange.github.io/docs/v5/account/wallet-balance
     */
    pub list: Vec<BybitWalletBalanceItem>
}

#[derive(Deserialize, Debug)]
pub struct BybitWalletBalanceItem {
    /*
    https://bybit-exchange.github.io/docs/v5/account/wallet-balance
     */
    pub totalEquity: String,
    pub accountIMRate: String,
    pub accountIMRateByMp: String,
    pub totalMarginBalance: String,
    pub totalInitialMargin: String,
    pub totalInitialMarginByMp: String,
    pub accountType: String,
    pub totalAvailableBalance: String,
    pub accountMMRate: String,
    pub accountMMRateByMp: String,
    pub totalPerpUPL: String,
    pub totalWalletBalance: String,
    pub accountLTV: String,
    pub totalMaintenanceMargin: String,
    pub totalMaintenanceMarginByMp: String,
    pub coin: Vec<BybitCoinInfo>
}

#[derive(Deserialize, Debug)]
pub struct BybitCoinInfo {
    /*
    https://bybit-exchange.github.io/docs/v5/account/wallet-balance
     */
    pub availableToBorrow: String,
    pub bonus: String,
    pub accruedInterest: String,
    pub availableToWithdraw: String,
    pub totalOrderIM: String,
    pub equity: String,
    pub totalPositionMM: String,
    pub usdValue: String,
    pub spotHedgingQty: String,
    pub unrealisedPnl: String,
    pub collateralSwitch: bool,
    pub borrowAmount: String,
    pub totalPositionIM: String,
    pub walletBalance: String,
    pub cumRealisedPnl: String,
    pub locked: String,
    pub marginCollateral: bool,
    pub coin: String
}


/*
Bybit Position Info
*/

#[derive(Deserialize, Debug)]
pub struct BybitPositionResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/position/list
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitPositionResult,
    pub retExtInfo: Value,
    pub time: i64
}

#[derive(Deserialize, Debug)]
pub struct BybitPositionResult {
    /*
    https://bybit-exchange.github.io/docs/v5/position/list
     */
    pub list: Vec<BybitPositionItem>,
    pub nextPageCursor: String,
    pub category: String,    
}

#[derive(Deserialize, Debug)]
pub struct BybitPositionItem {
    /*
    https://bybit-exchange.github.io/docs/v5/position/list
     */
    pub positionIdx: i32,
    pub riskId: i32,
    pub riskLimitValue: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub avgPrice: String,
    pub positionValue: String,
    pub tradeMode: i32,
    pub positionStatus: String,
    pub autoAddMargin: i32,
    pub adlRankIndicator: i32,
    pub leverage: String,
    pub positionBalance: String,
    pub markPrice: String,
    pub liqPrice: String,
    pub bustPrice: String,
    pub positionMM: String,
    pub positionMMByMp: String,
    pub positionIM: String,
    pub positionIMByMp: String,
    pub tpslMode: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub trailingStop: String,
    pub unrealisedPnl: String,
    pub curRealisedPnl: String,
    pub cumRealisedPnl: String,
    pub seq: i64,
    pub isReduceOnly: bool,
    pub mmrSysUpdateTime: Option<String>,
    pub leverageSysUpdatedTime: String,
    pub sessionAvgPrice: String,
    pub createdTime: String,
    pub updatedTime: String
}


/*
Bybit Open Orders
*/

#[derive(Deserialize, Debug)]
pub struct BybitOpenOrdersResponse {
    /*
    https://bybit-exchange.github.io/docs/v5/order/open-orders
     */
    pub retCode: i32,
    pub retMsg: String,
    pub result: BybitOpenOrdersResult,
    pub retExtInfo: Value,
    pub time: i64
}

#[derive(Deserialize, Debug)]
pub struct BybitOpenOrdersResult {
    /*
    https://bybit-exchange.github.io/docs/v5/order/open-orders
     */
    pub category: String,
    pub nextPageCursor: String,
    pub list: Vec<BybitOpenOrderItem>
}

#[derive(Deserialize, Debug)]
pub struct BybitOpenOrderItem {
    /*
    https://bybit-exchange.github.io/docs/v5/order/open-orders
     */
    pub orderId: String,
    pub orderLinkId: String,
    pub blockTradeId: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: String,
    pub isLeverage: String,
    pub positionIdx: i32,
    pub orderStatus: String,
    pub createType: Option<String>,
    pub cancelType: String,
    pub rejectReason: String,
    pub avgPrice: String,
    pub leavesQty: String,
    pub leavesValue: String,
    pub cumExecQty: String,
    pub cumExecValue: String,
    pub cumExecFee: String,
    pub timeInForce: String,
    pub orderType: String,
    pub stopOrderType: String,
    pub orderIv: String,
    pub marketUnit: String,
    pub triggerPrice: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub tpslMode: String,
    pub ocoTriggerBy: Option<String>,
    pub tpLimitPrice: String,
    pub slLimitPrice: String,
    pub tpTriggerBy: String,
    pub slTriggerBy: String,
    pub triggerDirection: i32,
    pub triggerBy: String,
    pub lastPriceOnCreated: String,
    pub basePrice: Option<String>,
    pub reduceOnly: bool,
    pub closeOnTrigger: bool,
    pub placeType: String,
    pub smpType: String,
    pub smpGroup: i32,
    pub smpOrderId: String,
    pub createdTime: String,
    pub updatedTime: String
}

/*
Bybit WebSocket Orderbook Response
*/

#[derive(Deserialize, Debug, Clone)]
pub struct BybitWebSocketOrderbookResponse {
    /*
    WebSocket orderbook response structure matching Bybit's format
    https://bybit-exchange.github.io/docs/v5/ws/public/orderbook
    */
    pub topic: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub ts: u64,
    pub data: BybitWebSocketOrderbookData,
    pub cts: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BybitWebSocketOrderbookData {
    /*
    Orderbook data structure for WebSocket responses
    */
    pub s: String,  // Symbol name
    pub b: Vec<[String; 2]>,  // Bids array - [price, size]
    pub a: Vec<[String; 2]>,  // Asks array - [price, size]
    pub u: u64,     // Update ID
    pub seq: u64,   // Cross sequence
}

/*
Bybit WebSocket Generic Response
For handling different types of WebSocket messages
*/

#[derive(Deserialize, Debug, Clone)]
pub struct BybitWebSocketResponse {
    /*
    Generic WebSocket response structure
    */
    pub topic: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub ts: u64,
    pub data: serde_json::Value,  // Generic data field for different message types
    pub cts: Option<u64>,
}

/*
Bybit WebSocket Subscription Request
*/

#[derive(Serialize, Debug, Clone)]
pub struct BybitWebSocketSubscription {
    /*
    WebSocket subscription request structure
    */
    pub op: String,  // Operation: "subscribe" or "unsubscribe"
    pub args: Vec<String>,  // List of topics to subscribe to
}

/*
Bybit WebSocket Subscription Response
*/

#[derive(Deserialize, Debug, Clone)]
pub struct BybitWebSocketSubscriptionResponse {
    /*
    WebSocket subscription response structure
    */
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub op: String,
    pub args: Vec<String>,
}

/*
Bybit Orderbook Management
*/

/// Represents a single price level in the orderbook
#[derive(Debug, Clone, PartialEq)]
pub struct OrderbookLevel {
    pub price: f64,
    pub size: f64,
}

/// Local orderbook state for managing snapshots and deltas
#[derive(Debug, Clone)]
pub struct LocalOrderbook {
    pub symbol: String,
    pub bids: Vec<OrderbookLevel>,  // Sorted by price descending (highest first)
    pub asks: Vec<OrderbookLevel>,  // Sorted by price ascending (lowest first)
    pub last_update_id: u64,
    pub last_sequence: u64,
    pub last_timestamp: u64,
}

impl LocalOrderbook {
    /// Create a new empty orderbook
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: Vec::new(),
            asks: Vec::new(),
            last_update_id: 0,
            last_sequence: 0,
            last_timestamp: 0,
        }
    }

    /// Apply a snapshot update (completely replace the orderbook)
    pub fn apply_snapshot(&mut self, snapshot: &BybitWebSocketOrderbookData) {
        println!("🔄 Applying snapshot for {}", self.symbol);
        
        // Clear existing data
        self.bids.clear();
        self.asks.clear();
        
        // Parse and add bids (convert strings to f64)
        for bid in &snapshot.b {
            if let (Ok(price), Ok(size)) = (bid[0].parse::<f64>(), bid[1].parse::<f64>()) {
                if size > 0.0 {  // Only add non-zero sizes
                    self.bids.push(OrderbookLevel { price, size });
                }
            }
        }
        
        // Parse and add asks (convert strings to f64)
        for ask in &snapshot.a {
            if let (Ok(price), Ok(size)) = (ask[0].parse::<f64>(), ask[1].parse::<f64>()) {
                if size > 0.0 {  // Only add non-zero sizes
                    self.asks.push(OrderbookLevel { price, size });
                }
            }
        }
        
        // Sort bids by price descending (highest first)
        self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        
        // Sort asks by price ascending (lowest first)
        self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Update metadata
        self.last_update_id = snapshot.u;
        self.last_sequence = snapshot.seq;
        self.last_timestamp = snapshot.u; // Using update ID as timestamp
        
        println!("✅ Snapshot applied: {} bids, {} asks", self.bids.len(), self.asks.len());
    }

    /// Apply a delta update (modify existing orderbook)
    pub fn apply_delta(&mut self, delta: &BybitWebSocketOrderbookData) {
        println!("📈 Applying delta for {}", self.symbol);
        
        // Apply bid updates
        for bid in &delta.b {
            if let (Ok(price), Ok(size)) = (bid[0].parse::<f64>(), bid[1].parse::<f64>()) {
                self.update_bid_level(price, size);
            }
        }
        
        // Apply ask updates
        for ask in &delta.a {
            if let (Ok(price), Ok(size)) = (ask[0].parse::<f64>(), ask[1].parse::<f64>()) {
                self.update_ask_level(price, size);
            }
        }
        
        // Update metadata
        self.last_update_id = delta.u;
        self.last_sequence = delta.seq;
        self.last_timestamp = delta.u;
        
        println!("✅ Delta applied: {} bids, {} asks", self.bids.len(), self.asks.len());
    }

    /// Update a bid level (price, size)
    fn update_bid_level(&mut self, price: f64, size: f64) {
        if size == 0.0 {
            // Remove the level
            self.bids.retain(|level| level.price != price);
        } else {
            // Find existing level or insert new one
            if let Some(pos) = self.bids.iter().position(|level| level.price == price) {
                self.bids[pos].size = size;
            } else {
                self.bids.push(OrderbookLevel { price, size });
                // Re-sort to maintain descending order
                self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        }
    }

    /// Update an ask level (price, size)
    fn update_ask_level(&mut self, price: f64, size: f64) {
        if size == 0.0 {
            // Remove the level
            self.asks.retain(|level| level.price != price);
        } else {
            // Find existing level or insert new one
            if let Some(pos) = self.asks.iter().position(|level| level.price == price) {
                self.asks[pos].size = size;
            } else {
                self.asks.push(OrderbookLevel { price, size });
                // Re-sort to maintain ascending order
                self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }
    }

    /// Get the best bid (highest bid price)
    pub fn best_bid(&self) -> Option<&OrderbookLevel> {
        self.bids.first()
    }

    /// Get the best ask (lowest ask price)
    pub fn best_ask(&self) -> Option<&OrderbookLevel> {
        self.asks.first()
    }

    /// Get the spread (ask - bid)
    pub fn spread(&self) -> Option<f64> {
        if let (Some(bid), Some(ask)) = (self.best_bid(), self.best_ask()) {
            Some(ask.price - bid.price)
        } else {
            None
        }
    }

    /// Print current orderbook state
    pub fn print_summary(&self) {
        println!("📊 Orderbook Summary for {}", self.symbol);
        println!("   Update ID: {}, Sequence: {}", self.last_update_id, self.last_sequence);
        
        if let Some(bid) = self.best_bid() {
            println!("   Best Bid: {:.2} @ {:.6}", bid.price, bid.size);
        }
        if let Some(ask) = self.best_ask() {
            println!("   Best Ask: {:.2} @ {:.6}", ask.price, ask.size);
        }
        if let Some(spread) = self.spread() {
            println!("   Spread: {:.2}", spread);
        }
        println!("   Total Levels: {} bids, {} asks", self.bids.len(), self.asks.len());
    }
}
