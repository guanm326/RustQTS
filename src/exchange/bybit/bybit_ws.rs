use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use url::Url;
use std::error::Error;

use crate::exchange::bybit::response::{
    BybitWebSocketOrderbookResponse,
    BybitWebSocketSubscription,
    BybitWebSocketSubscriptionResponse,
    LocalOrderbook,
};

/// WebSocket stream types for different Bybit endpoints
#[derive(Debug, Clone)]
pub enum BybitStreamType {
    /// Spot trading stream
    Spot,
    /// Linear perpetuals (USDT, USDC)
    Linear,
    /// Inverse contracts
    Inverse,
    /// Spread trading
    Spread,
    /// Options trading
    Option,
}

impl BybitStreamType {
    /// Get the WebSocket URL for the stream type
    /// 
    /// # Arguments
    /// * `is_testnet` - Whether to use testnet or mainnet URLs
    /// 
    /// # Returns
    /// The WebSocket URL as a String
    pub fn get_url(&self, is_testnet: bool) -> String {
        let base_url = if is_testnet {
            "wss://stream-testnet.bybit.com/v5/public"
        } else {
            "wss://stream.bybit.com/v5/public"
        };

        match self {
            BybitStreamType::Spot => format!("{}/spot", base_url),
            BybitStreamType::Linear => format!("{}/linear", base_url),
            BybitStreamType::Inverse => format!("{}/inverse", base_url),
            BybitStreamType::Spread => format!("{}/spread", base_url),
            BybitStreamType::Option => format!("{}/option", base_url),
        }
    }
}

/// WebSocket connection configuration
#[derive(Debug, Clone)]
pub struct BybitWebSocketConfig {
    /// Stream type (Spot, Linear, etc.)
    pub stream_type: BybitStreamType,
    /// Whether to use testnet
    pub is_testnet: bool,
    /// Maximum reconnection attempts
    pub max_reconnect_attempts: u32,
    /// Reconnection delay in milliseconds
    pub reconnect_delay_ms: u64,
}

impl Default for BybitWebSocketConfig {
    fn default() -> Self {
        Self {
            stream_type: BybitStreamType::Linear,
            is_testnet: true, // Default to testnet for safety
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
        }
    }
}

/// WebSocket message types that can be received
#[derive(Debug, Clone)]
pub enum BybitWebSocketMessage {
    /// Orderbook data
    Orderbook(BybitWebSocketOrderbookResponse),
    /// Subscription confirmation
    Subscription(BybitWebSocketSubscriptionResponse),
    /// Ping message
    Ping,
    /// Pong message
    Pong,
    /// Error message
    Error(String),
}

/// Main WebSocket client for Bybit
pub struct BybitWebSocketClient {
    /// Configuration for the WebSocket connection
    config: BybitWebSocketConfig,
    /// Sender channel for sending messages to the WebSocket
    sender: Option<mpsc::UnboundedSender<BybitWebSocketSubscription>>,
    /// Receiver channel for receiving messages from the WebSocket
    receiver: Option<mpsc::UnboundedReceiver<BybitWebSocketMessage>>,
}

impl BybitWebSocketClient {
    /// Create a new WebSocket client with the given configuration
    /// 
    /// # Arguments
    /// * `config` - WebSocket configuration
    /// 
    /// # Returns
    /// A new BybitWebSocketClient instance
    pub fn new(config: BybitWebSocketConfig) -> Self {
        Self {
            config,
            sender: None,
            receiver: None,
        }
    }

    /// Connect to the Bybit WebSocket stream
    /// 
    /// This method establishes the WebSocket connection and sets up the message channels.
    /// It returns a sender for sending subscription requests and a receiver for receiving messages.
    /// 
    /// # Returns
    /// * `Result<(mpsc::UnboundedSender<BybitWebSocketSubscription>, mpsc::UnboundedReceiver<BybitWebSocketMessage>), Box<dyn Error>>`
    ///   - Sender channel for sending subscription requests
    ///   - Receiver channel for receiving WebSocket messages
    ///   - Error if connection fails
    pub async fn connect(&mut self) -> Result<
        (
            mpsc::UnboundedSender<BybitWebSocketSubscription>,
            mpsc::UnboundedReceiver<BybitWebSocketMessage>,
        ),
        Box<dyn Error>,
    > {
        // Get the WebSocket URL based on configuration
        let url = self.config.stream_type.get_url(self.config.is_testnet);
        let url = Url::parse(&url)?;

        // Create channels for communication
        let (tx, mut rx) = mpsc::unbounded_channel::<BybitWebSocketSubscription>();
        let (message_tx, message_rx) = mpsc::unbounded_channel::<BybitWebSocketMessage>();

        // Store the channels for later use
        self.sender = Some(tx.clone());
        // Don't store message_rx here since we need to return it

        // Connect to the WebSocket
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Spawn a task to handle the WebSocket connection
        let message_tx_clone = message_tx.clone();
        tokio::spawn(async move {
            // Handle incoming messages from the WebSocket
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Try to parse the message as JSON
                        if let Ok(parsed) = serde_json::from_str::<BybitWebSocketOrderbookResponse>(&text) {
                            let _ = message_tx_clone.send(BybitWebSocketMessage::Orderbook(parsed));
                        } else if let Ok(parsed) = serde_json::from_str::<BybitWebSocketSubscriptionResponse>(&text) {
                            let _ = message_tx_clone.send(BybitWebSocketMessage::Subscription(parsed));
                        } else {
                            // Handle ping/pong messages
                            if text.contains("ping") {
                                let _ = message_tx_clone.send(BybitWebSocketMessage::Ping);
                            } else if text.contains("pong") {
                                let _ = message_tx_clone.send(BybitWebSocketMessage::Pong);
                            } else {
                                let _ = message_tx_clone.send(BybitWebSocketMessage::Error(format!("Unknown message: {}", text)));
                            }
                        }
                    }
                    Ok(Message::Ping(_)) => {
                        let _ = message_tx_clone.send(BybitWebSocketMessage::Ping);
                    }
                    Ok(Message::Pong(_)) => {
                        let _ = message_tx_clone.send(BybitWebSocketMessage::Pong);
                    }
                    Ok(Message::Close(_)) => {
                        println!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        let _ = message_tx_clone.send(BybitWebSocketMessage::Error(format!("WebSocket error: {}", e)));
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Spawn a task to handle outgoing messages
        tokio::spawn(async move {
            while let Some(subscription) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&subscription) {
                    if let Err(e) = write.send(Message::Text(json)).await {
                        eprintln!("Failed to send subscription: {}", e);
                        break;
                    }
                }
            }
        });

        Ok((tx, message_rx))
    }

    /// Subscribe to orderbook data for a specific symbol and depth
    /// 
    /// # Arguments
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `depth` - Orderbook depth (1, 50, 200, 500, 1000)
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Error if subscription fails
    pub async fn subscribe_orderbook(
        &self,
        symbol: &str,
        depth: u32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(sender) = &self.sender {
            let topic = format!("orderbook.{}.{}", depth, symbol);
            let subscription = BybitWebSocketSubscription {
                op: "subscribe".to_string(),
                args: vec![topic],
            };
            sender.send(subscription)?;
            Ok(())
        } else {
            Err("WebSocket not connected".into())
        }
    }

    /// Unsubscribe from orderbook data for a specific symbol and depth
    /// 
    /// # Arguments
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `depth` - Orderbook depth (1, 50, 200, 500, 1000)
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Error if unsubscription fails
    pub async fn unsubscribe_orderbook(
        &self,
        symbol: &str,
        depth: u32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(sender) = &self.sender {
            let topic = format!("orderbook.{}.{}", depth, symbol);
            let subscription = BybitWebSocketSubscription {
                op: "unsubscribe".to_string(),
                args: vec![topic],
            };
            sender.send(subscription)?;
            Ok(())
        } else {
            Err("WebSocket not connected".into())
        }
    }
}

/// Helper function to create a simple orderbook subscription example
/// 
/// This function demonstrates how to connect to Bybit WebSocket and subscribe to orderbook data.
/// It's designed to be easy to understand for beginners.
/// 
/// # Arguments
/// * `symbol` - Trading symbol to subscribe to (e.g., "BTCUSDT")
/// * `depth` - Orderbook depth (1, 50, 200, 500, 1000)
/// * `is_testnet` - Whether to use testnet or mainnet
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Error if the example fails
pub async fn run_orderbook_example(
    symbol: &str,
    depth: u32,
    is_testnet: bool,
) -> Result<(), Box<dyn Error>> {
    println!("🚀 Starting Bybit WebSocket Orderbook Example");
    println!("Symbol: {}, Depth: {}, Testnet: {}", symbol, depth, is_testnet);

    // Create WebSocket configuration
    let config = BybitWebSocketConfig {
        stream_type: BybitStreamType::Linear, // Using Linear for USDT perpetuals
        is_testnet,
        ..Default::default()
    };

    // Create and connect the WebSocket client
    let mut client = BybitWebSocketClient::new(config);
    let (sender, mut receiver) = client.connect().await?;

    println!("✅ Connected to Bybit WebSocket");

    // Subscribe to orderbook data
    client.subscribe_orderbook(symbol, depth).await?;
    println!("📡 Subscribed to orderbook.{}.{}", depth, symbol);

    // Create local orderbook to track state
    let mut local_orderbook = LocalOrderbook::new(symbol.to_string());
    
    // Listen for messages
    let mut message_count = 0;
    while let Some(message) = receiver.recv().await {
        match message {
            BybitWebSocketMessage::Orderbook(orderbook) => {
                message_count += 1;
                println!("\n📊 Orderbook Update #{}", message_count);
                println!("   Symbol: {}", orderbook.data.s);
                println!("   Type: {}", orderbook.data_type);
                println!("   Update ID: {}", orderbook.data.u);
                println!("   Sequence: {}", orderbook.data.seq);
                
                // Handle snapshot vs delta updates
                match orderbook.data_type.as_str() {
                    "snapshot" => {
                        println!("🔄 Processing SNAPSHOT...");
                        local_orderbook.apply_snapshot(&orderbook.data);
                    }
                    "delta" => {
                        println!("📈 Processing DELTA...");
                        local_orderbook.apply_delta(&orderbook.data);
                    }
                    _ => {
                        println!("❓ Unknown message type: {}", orderbook.data_type);
                        continue;
                    }
                }
                
                // Print current orderbook state
                local_orderbook.print_summary();
                println!("   ---");
            }
            BybitWebSocketMessage::Subscription(sub) => {
                println!("📋 Subscription Response: {}", sub.ret_msg);
                if sub.success {
                    println!("✅ Successfully subscribed to: {:?}", sub.args);
                } else {
                    println!("❌ Subscription failed: {:?}", sub.args);
                }
            }
            BybitWebSocketMessage::Ping => {
                println!("🏓 Received ping");
            }
            BybitWebSocketMessage::Pong => {
                println!("🏓 Received pong");
            }
            BybitWebSocketMessage::Error(err) => {
                println!("❌ Error: {}", err);
            }
        }

        // Stop after receiving 10 orderbook updates for demo purposes
        //if message_count >= 10 {
        //    println!("🛑 Demo complete! Received {} orderbook updates", message_count);
        //    break;
        //}
    }

    Ok(())
}
