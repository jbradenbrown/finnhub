use finnhub::{ClientConfig, FinnhubClient, RateLimitStrategy};
use std::time::Instant;

#[tokio::test]
#[ignore = "requires API key"]
async fn test_rate_limiting_behavior() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("FINNHUB_API_KEY")
        .expect("FINNHUB_API_KEY must be set");
    
    println!("\n=== Rate Limiting Test ===\n");
    
    // Create multiple clients with the 15-second window strategy
    let clients: Vec<FinnhubClient> = (0..5)
        .map(|i| {
            let mut config = ClientConfig::default();
            config.rate_limit_strategy = RateLimitStrategy::FifteenSecondWindow;
            println!("Created client {}", i);
            FinnhubClient::with_config(&api_key, config)
        })
        .collect();
    
    println!("\nMaking 10 requests from each of 5 clients (50 total requests)...\n");
    
    let start = Instant::now();
    let mut tasks = vec![];
    
    for (client_idx, client) in clients.into_iter().enumerate() {
        let task = tokio::spawn(async move {
            let mut success = 0;
            let mut rate_limited = 0;
            let mut other_errors = 0;
            
            for i in 0..10 {
                match client.stock().quote("AAPL").await {
                    Ok(_) => {
                        success += 1;
                        println!("Client {} request {} succeeded", client_idx, i);
                    }
                    Err(e) => {
                        let error_str = e.to_string();
                        if error_str.contains("429") || error_str.contains("rate limit") {
                            rate_limited += 1;
                            println!("Client {} request {} rate limited: {}", client_idx, i, e);
                        } else {
                            other_errors += 1;
                            println!("Client {} request {} failed: {}", client_idx, i, e);
                        }
                    }
                }
            }
            
            (client_idx, success, rate_limited, other_errors)
        });
        
        tasks.push(task);
    }
    
    // Wait for all tasks to complete
    let results: Vec<_> = futures::future::join_all(tasks).await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();
    
    let elapsed = start.elapsed();
    
    println!("\n=== Results ===");
    println!("Total time: {:?}", elapsed);
    
    let mut total_success = 0;
    let mut total_rate_limited = 0;
    let mut total_errors = 0;
    
    for (client_idx, success, rate_limited, errors) in results {
        println!(
            "Client {}: {} success, {} rate limited, {} errors",
            client_idx, success, rate_limited, errors
        );
        total_success += success;
        total_rate_limited += rate_limited;
        total_errors += errors;
    }
    
    println!("\nTotal: {} success, {} rate limited, {} errors", 
             total_success, total_rate_limited, total_errors);
    
    if total_rate_limited > 0 {
        println!("\n⚠️  Rate limiting occurred! Multiple clients with individual rate limiters can exceed API limits.");
        println!("Consider using --test-threads=N to limit parallel test execution.");
    } else {
        println!("\n✅ No rate limiting detected. The 15-second window strategy may be helping.");
    }
}