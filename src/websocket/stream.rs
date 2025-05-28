//! WebSocket streaming implementation.

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::error::Result;

const WEBSOCKET_URL: &str = "wss://ws.finnhub.io";

/// WebSocket message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WebSocketMessage {
    /// Trade data.
    Trade {
        /// Trade data.
        data: Vec<TradeData>,
    },
    /// Ping message.
    Ping,
    /// Error message.
    Error {
        /// Error message.
        msg: String,
    },
}

/// Trade data from WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Price.
    #[serde(rename = "p")]
    pub price: f64,
    /// Timestamp.
    #[serde(rename = "t")]
    pub timestamp: i64,
    /// Volume.
    #[serde(rename = "v")]
    pub volume: f64,
    /// Trade conditions.
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<String>>,
}

/// WebSocket subscription request.
#[derive(Debug, Serialize)]
struct SubscribeRequest {
    #[serde(rename = "type")]
    request_type: String,
    symbol: String,
}

/// WebSocket client for real-time data.
pub struct WebSocketClient {
    api_key: String,
}

impl WebSocketClient {
    /// Create a new WebSocket client.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Connect to the WebSocket API.
    pub async fn connect(&self) -> Result<WebSocketStream> {
        let url = Url::parse(&format!("{}?token={}", WEBSOCKET_URL, self.api_key))?;

        let (ws_stream, _) = connect_async(url.as_str()).await?;

        Ok(WebSocketStream { inner: ws_stream })
    }
}

/// Active WebSocket stream.
pub struct WebSocketStream {
    inner: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl WebSocketStream {
    /// Subscribe to a symbol.
    pub async fn subscribe(&mut self, symbol: &str) -> Result<()> {
        let request = SubscribeRequest {
            request_type: "subscribe".to_string(),
            symbol: symbol.to_string(),
        };

        let message = Message::Text(serde_json::to_string(&request)?);
        self.inner.send(message).await?;

        Ok(())
    }

    /// Unsubscribe from a symbol.
    pub async fn unsubscribe(&mut self, symbol: &str) -> Result<()> {
        let request = SubscribeRequest {
            request_type: "unsubscribe".to_string(),
            symbol: symbol.to_string(),
        };

        let message = Message::Text(serde_json::to_string(&request)?);
        self.inner.send(message).await?;

        Ok(())
    }

    /// Receive the next message from the stream.
    pub async fn next(&mut self) -> Result<Option<WebSocketMessage>> {
        match self.inner.next().await {
            Some(Ok(Message::Text(text))) => {
                let message: WebSocketMessage = serde_json::from_str(&text)?;
                Ok(Some(message))
            }
            Some(Ok(Message::Close(_))) => Ok(None),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
            _ => Ok(None), // Ignore other message types
        }
    }
}
