



use serde::{Deserialize, Serialize};
use crate::enums::StrategyTypes;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrategyParams{
    pub strategy_name: String,
    pub strategy_type: StrategyTypes
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HedgeParams{
    pub hedge_ratio: f64,        // in usd dollar terms, for every $1 quote, how much should you short
    pub price_ratio: f64,        // price ratio parameter
    pub hedge_mode: u32,          // 0=no hedging, 1=active hedging, 2=submits limit order but doesn't do anything
    pub hedge_delay: f64,        // in seconds
    pub max_slippage: f64,       // max allowable cross
    pub hedge_offset: f64,       // used when hedge_mode=2
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuoteParams{
    pub bid_on: bool,                    // Whether to turn on bid side only
    pub ask_on: bool,                     // Whether to turn on ask side only
    pub min_spread: f64,                  // 50% to 70% of AT
    pub bid_offset: f64,                  // means how much bid offset we are from ratio spread
    pub ask_offset: f64,                  // ask offset from ratio spread
    pub vol_spread_mult: f64,             // Multiple of vol to add to spread
    pub relist_interval: f64,             // aim to be less than clip interval, so that it wont overlap with other orders
    pub max_position: f64,                // this is in coin qty, and its in addition to the clip size quantity +levels
    pub clip_size: f64,                   // Initial Size of inner order
    pub quote_levels: u32,                // number of quote levels
    pub clip_step_size: f64,              // step size for clips
    pub clip_interval: f64,               // on the last level fill, target 1.5ATR or more
    pub poll_interval_seconds: u32,       // polling interval in seconds
    pub order_refresh_rate_ms: u32,       // how long before we can update order
    pub fill_refresh_delay: u32,          // how long to pause after a fill before we re-submit
    pub margin_ratio_threshold: f64,     // when margin ratio exceeds this level, we will actively quote to reduce
    pub margin_check_interval: u32,       // in milliseconds, 1 min = 60000ms
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunMode{
    pub debug: bool,                      // if this is true, we will not place any orders in market
    pub risk_management_mode: bool,      // if we have this on, we actively monitor margins and cannot increase position
}



