
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
