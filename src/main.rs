
// Import Class Objects
mod exchange;
mod data_structure;

use crate::exchange::bybit::response::{
    BybitOrderbookResponse, 
    BybitTickersResponse, 
    BybitAccInfoResponse,
    BybitWalletBalanceResponse,
    BybitPositionResponse,
    BybitOpenOrdersResponse

};
use crate::exchange::bybit::bybit_rest::BybitRestClient;
use crate::exchange::bybit::bybit_ws::run_orderbook_example;
use data_structure::APIKey;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::format,fs};
use std::error::Error;
use tokio::time::{sleep, Duration}; // Import tokio for async runtime and sleep function




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Rust QTS");
    println!("=================================\n");

    //let json_content = fs::read_to_string(r"C:\Users\micha\dev\RustQTS\config\bybit_main.json")?;
    let json_content = fs::read_to_string(r"/Users/michaelguan326/Documents/dev/rust/RustQTS/config/bybit_main.json")?;
    let config: APIKey = serde_json::from_str(&json_content)?;



    //let mut by_rest_client: BybitRestClient = BybitRestClient::new(config.api_key,config.api_secret);
    //let ord_response: BybitOpenOrdersResponse = by_rest_client.get_open_orders("linear",Some("TAUSDT"),None,None,None,None,None,None,None,None).await?;


    //let pos_response: BybitPositionResponse = by_rest_client.get_positions("linear",None,None,Some("USDT"),None,None).await?;

    //let acc_info_response: BybitWalletBalanceResponse = by_rest_client.get_account_info(None,None).await?;
    //let acc_type_response: BybitAccInfoResponse = by_rest_client.get_account_type().await?;
    //let ob_response: BybitOrderbookResponse = by_rest_client.get_orderbook("linear","BTCUSDT").await?;
    //let tk_response: BybitTickersResponse = by_rest_client.get_tickers("linear", None,None, None).await?;

    //println!("Position Info: {:?}", ord_response.result);
    //println!("Position Info: {:?}", pos_response.result);

    //println!("Acc Info: {:?}", acc_info_response.result);
    //println!("Tickers: {:?}", tk_response.result);
    //println!("Orderbook: {:?}", ob_response.result);

    println!("\n🌐 Testing WebSocket Connection...");
    println!("=================================");
    
    // Test WebSocket orderbook connection
    // This will connect to Bybit testnet and subscribe to BTCUSDT orderbook
    // Change the parameters as needed:
    // - Symbol: "BTCUSDT", "ETHUSDT", etc.
    // - Depth: 1, 50, 200, 500, 1000
    // - Testnet: true for testnet, false for mainnet
    run_orderbook_example("BTCUSDT", 1, false).await?;

    Ok(())
}


