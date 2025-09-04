
// Import Class Objects
mod exchange { 
    pub mod bybit{
        pub mod response;
        pub mod bybit_rest;
    }
}
mod data_structure;

use crate::exchange::bybit::response::{
    BybitOrderbookResponse, 
    BybitTickersResponse, 
    BybitAccInfoResponse,
    BybitWalletBalanceResponse

};
use crate::exchange::bybit::bybit_rest::BybitRestClient;
use data_structure::APIKey;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::format,fs};
use std::error::Error;
use tokio::time::{sleep, Duration}; // Import tokio for async runtime and sleep function




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rust QTS");
    println!("=================================\n");

    let json_content = fs::read_to_string(r"C:\Users\micha\dev\RustQTS\config\bybit_main.json")?;
    let config: APIKey = serde_json::from_str(&json_content)?;



    let mut by_rest_client: BybitRestClient = BybitRestClient::new(config.api_key,config.api_secret);


    let acc_info_response: BybitWalletBalanceResponse = by_rest_client.get_account_info(None,None).await?;

    //let acc_type_response: BybitAccInfoResponse = by_rest_client.get_account_type().await?;

    //let ob_response: BybitOrderbookResponse = by_rest_client.get_orderbook("linear","BTCUSDT").await?;
    //let tk_response: BybitTickersResponse = by_rest_client.get_tickers("linear", None,None, None).await?;

    println!("Acc Info: {:?}", acc_info_response.result);

    //println!("Tickers: {:?}", tk_response.result);

    //println!("Orderbook: {:?}", ob_response.result);

    Ok(())
}


