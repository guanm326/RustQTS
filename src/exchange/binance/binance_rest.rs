

use std::error::Error;
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

// Import response structs
use crate::exchange::binance::response::{
    BinanceOrderbookResponse, 
    BinanceTickerResponse, 
    BinanceTickersResponse, 
    BinanceTickerItem,
    BinancePositionResponse, 
    BinancePositionItem
};


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

    fn generate_binance_signature(&self, query_string: &str) -> Result<String, Box<dyn Error>> {
        /*
            Generate HMAC SHA256 signature for Binance API
            Based on: https://binance-docs.github.io/apidocs/futures/en/#signed-endpoint-examples-for-post-fapi-v1-order-hmac-keys
        */
        
        // Create HMAC-SHA256 instance with the API secret
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(query_string.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(signature)
    }

    pub fn get_binance_auth_headers(&self) -> Result<HeaderMap, Box<dyn Error>> {
        /*
            Generate authentication headers for Binance API
        */
        
        let mut headers = HeaderMap::new();
        headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&self.api_key)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
        
        Ok(headers)
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

    pub async fn get_tickers(&self, symbol: Option<&str>) -> Result<BinanceTickersResponse, Box<dyn Error>> {
        /*
            Get 24hr ticker price change statistics
            https://binance-docs.github.io/apidocs/futures/en/#24hr-ticker-price-change-statistics
        */
        
        let mut url = format!("{}/fapi/v1/ticker/24hr", self.base_url);
        
        // Add optional symbol parameter if provided
        if let Some(symbol) = symbol {
            url.push_str(&format!("?symbol={}", symbol));
        }
        
        // Query API Endpoint (no authentication required for public data)
        let response = self.http_client.get(&url).send().await?;
        
        // Handle the response - Binance returns either a single object or array
        let response_text = response.text().await?;
        
        // Try to parse as array first (multiple tickers)
        if let Ok(tickers_array) = serde_json::from_str::<Vec<BinanceTickerResponse>>(&response_text) {
            Ok(BinanceTickersResponse {
                list: tickers_array.into_iter().map(|ticker| BinanceTickerItem {
                    symbol: ticker.symbol,
                    priceChange: ticker.priceChange,
                    priceChangePercent: ticker.priceChangePercent,
                    weightedAvgPrice: ticker.weightedAvgPrice,
                    lastPrice: ticker.lastPrice,
                    lastQty: ticker.lastQty,
                    openPrice: ticker.openPrice,
                    highPrice: ticker.highPrice,
                    lowPrice: ticker.lowPrice,
                    volume: ticker.volume,
                    quoteVolume: ticker.quoteVolume,
                    openTime: ticker.openTime,
                    closeTime: ticker.closeTime,
                    firstId: ticker.firstId,
                    lastId: ticker.lastId,
                    count: ticker.count,
                }).collect()
            })
        } else {
            // Try to parse as single object (single ticker)
            let single_ticker: BinanceTickerResponse = serde_json::from_str(&response_text)?;
            Ok(BinanceTickersResponse {
                list: vec![BinanceTickerItem {
                    symbol: single_ticker.symbol,
                    priceChange: single_ticker.priceChange,
                    priceChangePercent: single_ticker.priceChangePercent,
                    weightedAvgPrice: single_ticker.weightedAvgPrice,
                    lastPrice: single_ticker.lastPrice,
                    lastQty: single_ticker.lastQty,
                    openPrice: single_ticker.openPrice,
                    highPrice: single_ticker.highPrice,
                    lowPrice: single_ticker.lowPrice,
                    volume: single_ticker.volume,
                    quoteVolume: single_ticker.quoteVolume,
                    openTime: single_ticker.openTime,
                    closeTime: single_ticker.closeTime,
                    firstId: single_ticker.firstId,
                    lastId: single_ticker.lastId,
                    count: single_ticker.count,
                }]
            })
        }
    }

    // Private Data Endpoints
    pub async fn get_positions(&self, symbol: Option<&str>) -> Result<BinancePositionResponse, Box<dyn Error>> {
        /*
            Get current position information
            https://binance-docs.github.io/apidocs/futures/en/#position-information-v3-user_data
        */
        
        let mut url = format!("{}/fapi/v3/positionRisk", self.base_url);
        
        // Generate timestamp and recvWindow
        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";
        
        // Build query parameters
        let mut query_params = vec![
            ("timestamp".to_string(), timestamp.to_string()),
            ("recvWindow".to_string(), recv_window.to_string()),
        ];
        
        // Add optional symbol parameter if provided
        if let Some(symbol) = symbol {
            query_params.push(("symbol".to_string(), symbol.to_string()));
        }
        
        // Create query string for signature
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        // Generate signature
        let signature = self.generate_binance_signature(&query_string)?;
        
        // Build final URL with signature
        let final_url = format!("{}?{}&signature={}", url, query_string, signature);
        
        // Create headers
        let headers = self.get_binance_auth_headers()?;
        
        // Make the request
        let response = self
            .http_client
            .get(&final_url)
            .headers(headers)
            .send()
            .await?;
        


        
        // Parse response - Binance returns an array directly, not wrapped in a result object
        let response_text = response.text().await?;
        println!("Raw API response: {}", response_text);
        let positions: Vec<BinancePositionItem> = serde_json::from_str(&response_text)?;
        
        Ok(BinancePositionResponse {
            list: positions
        })
    }

}