

use std::error::Error;
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

// Import response structs
use crate::exchange::binance::response::BinanceOrderbookResponse;


pub struct BinanceRestClient {
    api_key: String,
    api_secret: String,
    base_url: String,
    http_client: Client
}


impl BinanceRestClient {
    
    // Binance Rest client Constructor
    pub fn new(api_key:String, api_secret:String) -> Self{
        BinanceRestClient { 
            api_key: api_key,
            api_secret: api_secret,
            base_url: "https://fapi.binance.com".to_string(), 
            http_client:  Client::new()
        }
    }



    

    // Public Data Endpoints
    pub async fn get_orderbook(&self, symbol: &str, limit: Option<u32>) -> Result<BinanceOrderbookResponse, Box<dyn Error>> {
        /*
            Get orderbook data for a symbol
            https://binance-docs.github.io/apidocs/futures/en/#order-book
        */
        
        let mut url = format!("{}/fapi/v1/depth?symbol={}", self.base_url, symbol);
        
        // Add optional limit parameter if provided
        if let Some(limit) = limit {
            // Validate limit parameter according to Binance docs
            let valid_limits = [5, 10, 20, 50, 100, 500, 1000];
            if valid_limits.contains(&limit) {
                url.push_str(&format!("&limit={}", limit));
            } else {
                return Err("Invalid limit. Must be one of: 5, 10, 20, 50, 100, 500, 1000".into());
            }
        }
        
        // Query API Endpoint (no authentication required for public data)
        let response = self.http_client.get(&url).send().await?;
        let api_response = response.json::<BinanceOrderbookResponse>().await?;
        
        Ok(api_response)
    }

    



}