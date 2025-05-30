//! Basic WebSocket usage example.
//!
//! This example demonstrates the current WebSocket implementation.
//! Note: This is a minimal implementation and lacks production features.

#[cfg(feature = "websocket")]
use finnhub::websocket::{WebSocketClient, WebSocketMessage};
use std::time::Duration;
use tokio::time::timeout;

#[cfg(not(feature = "websocket"))]
fn main() {
    println!("WebSocket support requires the 'websocket' feature.");
    println!("Run with: cargo run --example websocket_basic --features websocket");
}

#[cfg(feature = "websocket")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY must be set");
    
    println!("=== Finnhub WebSocket Example ===\n");
    println!("Note: This is a basic implementation demonstration.\n");
    
    // Create WebSocket client
    let client = WebSocketClient::new(api_key);
    println!("Connecting to Finnhub WebSocket...");
    
    // Connect to the WebSocket
    let mut stream = client.connect().await?;
    println!("Connected successfully!\n");
    
    // Subscribe to some symbols
    let symbols = vec!["AAPL", "GOOGL", "MSFT"];
    for symbol in &symbols {
        println!("Subscribing to {}...", symbol);
        stream.subscribe(symbol).await?;
    }
    println!("\nWaiting for trade data (this may take a moment during market hours)...\n");
    
    // Process messages for 30 seconds
    let duration = Duration::from_secs(30);
    let start = std::time::Instant::now();
    
    loop {
        // Check if we've exceeded our time limit
        if start.elapsed() > duration {
            println!("\nTime limit reached. Unsubscribing...");
            break;
        }
        
        // Wait for next message with timeout
        match timeout(Duration::from_secs(5), stream.next()).await {
            Ok(Ok(Some(msg))) => {
                match msg {
                    WebSocketMessage::Trade { data } => {
                        for trade in data {
                            println!(
                                "[{}] Trade: {} @ ${:.2} vol: {:.0} conditions: {:?}",
                                chrono::Local::now().format("%H:%M:%S%.3f"),
                                trade.symbol,
                                trade.price,
                                trade.volume,
                                trade.conditions.as_ref().unwrap_or(&vec![])
                            );
                        }
                    }
                    WebSocketMessage::Ping => {
                        println!("[{}] Received ping", chrono::Local::now().format("%H:%M:%S"));
                    }
                    WebSocketMessage::Error { msg } => {
                        eprintln!("WebSocket error: {}", msg);
                    }
                }
            }
            Ok(Ok(None)) => {
                println!("WebSocket closed");
                break;
            }
            Ok(Err(e)) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            Err(_) => {
                println!("No data received in 5 seconds (normal outside market hours)");
            }
        }
    }
    
    // Unsubscribe before closing
    for symbol in &symbols {
        println!("Unsubscribing from {}...", symbol);
        stream.unsubscribe(symbol).await?;
    }
    
    println!("\nWebSocket example complete.");
    println!("\nNotes:");
    println!("- Trade data is only available during market hours");
    println!("- This implementation lacks reconnection logic");
    println!("- In production, you'd want proper error handling and recovery");
    
    Ok(())
}