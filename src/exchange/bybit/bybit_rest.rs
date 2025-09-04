
// Import Class Objects
use crate::exchange::bybit::response::{
    BybitOrderbookResponse, 
    BybitTickersResponse, 
    BybitAccInfoResponse,
    BybitWalletBalanceResponse
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
        let account_type = account_type.unwrap_or("INVERSE");

        let mut url = format!("{}/v5/account/wallet-balance?accountType={}", self.base_url, account_type);

        // Add optional coin parameter if provided
        if let Some(coin) = coin {
            url.push_str(&format!("&coin={}", coin));
        }

        let payload = serde_json::json!({});
        
        let timestamp = chrono::Utc::now().timestamp_millis();
        let recv_window = "5000";
        let query_string: String =  format!("{}{}{}", timestamp, self.api_key, recv_window);

        let mut headers = self.get_bybit_auth_headers(&query_string,timestamp,recv_window)?;

        let response = self.http_client.get(&url).headers(headers).send().await?;


        // Debug: Print the raw response text
        let response_text = response.text().await?;
        println!("Raw API response: {}", response_text);
        
        // Try to parse as JSON to see the structure
        let json_value: serde_json::Value = serde_json::from_str(&response_text)?;
        println!("Parsed JSON: {}", serde_json::to_string_pretty(&json_value)?);
        
        // Then try to deserialize
        let api_response: BybitWalletBalanceResponse = serde_json::from_str(&response_text)?;
        

        //let api_response: BybitWalletBalanceResponse = response.json::<BybitWalletBalanceResponse>().await?;

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

