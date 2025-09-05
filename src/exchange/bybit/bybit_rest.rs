
// Import Class Objects
use crate::exchange::bybit::response::{
    BybitOrderbookResponse, 
    BybitTickersResponse, 
    BybitAccInfoResponse,
    BybitWalletBalanceResponse,
    BybitPositionResponse,
    BybitOpenOrdersResponse
};

use std::error::Error;
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;


pub struct BybitRestClient {
    api_key: String,
    api_secret: String,
    base_url: String,
    http_client: Client
}


impl BybitRestClient {
    
    // Bybit Rest client Constructor
    pub fn new(api_key:String, api_secret:String) -> Self{
        BybitRestClient { 
            api_key: api_key,
            api_secret: api_secret,
            base_url: "https://api.bybit.com".to_string(), 
            http_client:  Client::new()
        }
    }

    fn generate_bybit_signature2<T:Serialize>(&self, payload: &T,timestamp: i64,recv_window: &str) -> Result<String, Box<dyn Error>>{


        let payload_json = serde_json::to_string(payload)?;
        //let param_str = format!("{}{}{}{}", timestamp, self.api_key, recv_window, payload_json);

        let param_str = if payload_json == "{}" {
            format!("{}{}{}", timestamp, self.api_key, recv_window)
        } else {
            format!("{}{}{}{}", timestamp, self.api_key, recv_window, payload_json)
        };

        // Create HMAC-SHA256 instance with the API secret
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(param_str.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(signature)
    }

    fn generate_bybit_signature(&self, query_string: &str) -> Result<String, Box<dyn Error>>{

        // Create HMAC-SHA256 instance with the API secret
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(query_string.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(signature)
    }


    pub fn get_bybit_auth_headers(&self, signature: &str, timestamp: i64, recv_window: &str) -> Result<HeaderMap, Box<dyn Error>> {

        // Construct headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature)?);
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&self.api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(recv_window)?);
        
        Ok(headers)
    }



    // Private Data Endpoints
    pub async fn get_account_type(&self) -> Result<BybitAccInfoResponse,Box<dyn Error>>{
        /*
            Bybits API calls the Account info, but internally we will handle this as 
            account type
         */

        let url = format!("{}/v5/account/info", self.base_url);

        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";
        let query_string: String =  format!("{}{}{}", timestamp, self.api_key, recv_window);


        let signature = self.generate_bybit_signature(&query_string)?;
        let mut headers = self.get_bybit_auth_headers(&signature, timestamp, recv_window)?;
        
        let response = self.http_client.get(&url).headers(headers).send().await?;
        let api_response: BybitAccInfoResponse = response.json::<BybitAccInfoResponse>().await?;

        Ok(api_response)
    }


    pub async fn get_account_info(&self, account_type: Option<&str>, coin: Option<&str>) -> Result<BybitWalletBalanceResponse,Box<dyn Error>>{
        /*
            Bybits API calls wallet balance, but internally we will handle this as 
            account info
        */
        let account_type = account_type.unwrap_or("UNIFIED");

        let mut url = format!("{}/v5/account/wallet-balance?accountType={}", self.base_url, account_type);

        let mut query_params = format!("accountType={}", account_type);


        // Add optional coin parameter if provided
        if let Some(coin) = coin {
            url.push_str(&format!("&coin={}", coin));
            query_params.push_str(&format!("&coin={}", coin));
        }
        
        //println!("url String {}",url);
        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";

        let signature_string = format!("{}{}{}{}", timestamp, self.api_key, recv_window, query_params);
        //println!("signature_string: {}", signature_string);

        let signature = self.generate_bybit_signature(&signature_string)?;
        let mut headers = self.get_bybit_auth_headers(&signature, timestamp, recv_window)?;

        let response = self.http_client.get(&url).headers(headers).send().await?;


        // Debug: Print the raw response text
        //let response_text = response.text().await?;
        //println!("Raw API response: {}", response_text);
        
        // Try to parse as JSON to see the structure
        //let json_value: serde_json::Value = serde_json::from_str(&response_text)?;
        //println!("Parsed JSON: {}", serde_json::to_string_pretty(&json_value)?);
        
        // Then try to deserialize
        //let api_response: BybitWalletBalanceResponse = serde_json::from_str(&response_text)?;
        

        let api_response: BybitWalletBalanceResponse = response.json::<BybitWalletBalanceResponse>().await?;

        Ok(api_response)
    }


    pub async fn get_positions(&self, category: &str, symbol: Option<&str>, base_coin: Option<&str>, settle_coin: Option<&str>, limit: Option<i32>, cursor: Option<&str>) -> Result<BybitPositionResponse, Box<dyn Error>> {
        if symbol.is_none() && settle_coin.is_none() {
            return Err("Either symbol or settle_coin must be provided".into());
        }
        
        let endpoint = "/v5/position/list";
        
        // Build query parameters
        let mut params = vec![("category".to_string(), category.to_string())];
        
        if let Some(symbol) = symbol {
            params.push(("symbol".to_string(), symbol.to_string()));
        }
        if let Some(base_coin) = base_coin {
            params.push(("baseCoin".to_string(), base_coin.to_string()));
        }
        if let Some(settle_coin) = settle_coin {
            params.push(("settleCoin".to_string(), settle_coin.to_string()));
        }
        if let Some(limit) = limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor".to_string(), cursor.to_string()));
        }
        
        // Generate timestamp and signature
        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";
        
        // Create query string for signature
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        let sign_string = format!("{}{}{}{}", timestamp, self.api_key, recv_window, query_string);
        let signature = self.generate_bybit_signature(&sign_string)?;
        
        // Build URL with query parameters
        let url = format!("{}{}?{}", self.base_url, endpoint, query_string);
        
        // Create headers
        let headers = self.get_bybit_auth_headers(&signature, timestamp, recv_window)?;
        
        // Make the request
        let response = self
            .http_client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
        


        // Debug: Print the raw response text
        //let response_text = response.text().await?;
        //println!("Raw API response: {}", response_text);
        
        // Try to parse as JSON to see the structure
        //let json_value: serde_json::Value = serde_json::from_str(&response_text)?;
        //println!("Parsed JSON: {}", serde_json::to_string_pretty(&json_value)?);
        
        // Then try to deserialize
        //let api_response: BybitPositionResponse = serde_json::from_str(&response_text)?;
        

        
        let api_response: BybitPositionResponse = response.json::<BybitPositionResponse>().await?;
        
        Ok(api_response)
    }

    pub async fn get_open_orders(
        &self, 
        category: &str, 
        symbol: Option<&str>, 
        base_coin: Option<&str>, 
        settle_coin: Option<&str>, 
        order_id: Option<&str>, 
        order_link_id: Option<&str>, 
        open_only: Option<i32>, 
        order_filter: Option<&str>, 
        limit: Option<i32>, 
        cursor: Option<&str>
    ) -> Result<BybitOpenOrdersResponse, Box<dyn Error>> {
        
        // Validate required parameters based on category
        match category {
            "linear" => {
                if symbol.is_none() && base_coin.is_none() && settle_coin.is_none() {
                    return Err("For linear category, either symbol, baseCoin, or settleCoin must be provided".into());
                }
            },
            "inverse" => {
                if symbol.is_none() && base_coin.is_none() && settle_coin.is_none() {
                    return Err("For inverse category, either symbol, baseCoin, or settleCoin must be provided".into());
                }
            },
            "spot" => {
                if symbol.is_none() && base_coin.is_none() {
                    return Err("For spot category, either symbol or baseCoin must be provided".into());
                }
            },
            "option" => {
                // Option category doesn't require any specific parameters
            },
            _ => {
                return Err("Invalid category. Must be one of: linear, inverse, spot, option".into());
            }
        }
        
        let endpoint = "/v5/order/realtime";
        
        // Build query parameters
        let mut params = vec![("category".to_string(), category.to_string())];
        
        if let Some(symbol) = symbol {
            params.push(("symbol".to_string(), symbol.to_string()));
        }
        if let Some(base_coin) = base_coin {
            params.push(("baseCoin".to_string(), base_coin.to_string()));
        }
        if let Some(settle_coin) = settle_coin {
            params.push(("settleCoin".to_string(), settle_coin.to_string()));
        }
        if let Some(order_id) = order_id {
            params.push(("orderId".to_string(), order_id.to_string()));
        }
        if let Some(order_link_id) = order_link_id {
            params.push(("orderLinkId".to_string(), order_link_id.to_string()));
        }
        if let Some(open_only) = open_only {
            params.push(("openOnly".to_string(), open_only.to_string()));
        }
        if let Some(order_filter) = order_filter {
            params.push(("orderFilter".to_string(), order_filter.to_string()));
        }
        if let Some(limit) = limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor".to_string(), cursor.to_string()));
        }
        
        // Generate timestamp and signature
        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";
        
        // Create query string for signature
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        let sign_string = format!("{}{}{}{}", timestamp, self.api_key, recv_window, query_string);
        let signature = self.generate_bybit_signature(&sign_string)?;
        
        // Build URL with query parameters
        let url = format!("{}{}?{}", self.base_url, endpoint, query_string);
        
        // Create headers
        let headers = self.get_bybit_auth_headers(&signature, timestamp, recv_window)?;
        
        // Make the request
        let response = self
            .http_client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
        
        let api_response: BybitOpenOrdersResponse = response.json::<BybitOpenOrdersResponse>().await?;
        
        Ok(api_response)
    }
    // Public Data Endpoints
    pub async fn get_orderbook(&self, category: &str, symbol: &str) -> Result<BybitOrderbookResponse,Box<dyn Error>>{

        let url= format!(
        "{}/v5/market/orderbook?category={}&symbol={}",
            self.base_url, category, symbol

        );  

        // Query API Endpoint
        let response = self.http_client.get(&url).send().await?;
        let api_response = response.json::<BybitOrderbookResponse>().await?;

        
        Ok(api_response)

    }

    pub async fn get_tickers(&self, category: &str, symbol: Option<&str>, base_coin: Option<&str>, exp_date: Option<&str>) -> Result<BybitTickersResponse, Box<dyn Error>> {
        let mut url = format!(
            "{}/v5/market/tickers?category={}",
            self.base_url, category
        );

        // Add optional parameters if provided
        if let Some(symbol) = symbol {
            url.push_str(&format!("&symbol={}", symbol));
        }
        if let Some(base_coin) = base_coin {
            url.push_str(&format!("&baseCoin={}", base_coin));
        }
        if let Some(exp_date) = exp_date {
            url.push_str(&format!("&expDate={}", exp_date));
        }

        // Query API Endpoint
        let response = self.http_client.get(&url).send().await?;
        let api_response = response.json::<BybitTickersResponse>().await?;

        Ok(api_response)
    }



}

