//! Rate limiting implementation for the Finnhub API.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Rate limiter using token bucket algorithm.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterInner>>,
}

struct RateLimiterInner {
    /// Maximum tokens in the bucket.
    capacity: u32,
    /// Current number of tokens.
    tokens: u32,
    /// Tokens refilled per second.
    refill_rate: u32,
    /// Last time tokens were refilled.
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter with specified capacity and refill rate.
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimiterInner {
                capacity,
                tokens: capacity,
                refill_rate,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Create a rate limiter for Finnhub's default limits (30 requests/second).
    pub fn finnhub_default() -> Self {
        Self::new(30, 30)
    }
    
    /// Create a rate limiter for Finnhub with 15-second averaging window.
    /// This allows 450 requests per 15 seconds (30 req/s * 15s).
    pub fn finnhub_15s_window() -> Self {
        // 450 tokens capacity, refill at 30 tokens/second
        Self::new(450, 30)
    }

    /// Acquire a token, waiting if necessary.
    pub async fn acquire(&self) -> Result<(), crate::Error> {
        loop {
            let mut limiter = self.inner.lock().await;

            // Refill tokens based on elapsed time
            let now = Instant::now();
            let elapsed = now.duration_since(limiter.last_refill);
            let tokens_to_add = (elapsed.as_secs_f64() * f64::from(limiter.refill_rate)) as u32;

            if tokens_to_add > 0 {
                limiter.tokens = (limiter.tokens + tokens_to_add).min(limiter.capacity);
                limiter.last_refill = now;
            }

            // Try to acquire a token
            if limiter.tokens > 0 {
                limiter.tokens -= 1;
                return Ok(());
            }

            // Calculate wait time
            let tokens_needed = 1;
            let wait_time =
                Duration::from_secs_f64(f64::from(tokens_needed) / f64::from(limiter.refill_rate));

            drop(limiter); // Release lock while waiting
            sleep(wait_time).await;
        }
    }

    /// Try to acquire a token without waiting.
    pub async fn try_acquire(&self) -> Result<(), crate::Error> {
        let mut limiter = self.inner.lock().await;

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(limiter.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * f64::from(limiter.refill_rate)) as u32;

        if tokens_to_add > 0 {
            limiter.tokens = (limiter.tokens + tokens_to_add).min(limiter.capacity);
            limiter.last_refill = now;
        }

        // Try to acquire a token
        if limiter.tokens > 0 {
            limiter.tokens -= 1;
            Ok(())
        } else {
            let retry_after = (1.0 / f64::from(limiter.refill_rate)).ceil() as u64;
            Err(crate::Error::RateLimitExceeded { retry_after })
        }
    }

    /// Get the current number of available tokens.
    pub async fn available_tokens(&self) -> u32 {
        let mut limiter = self.inner.lock().await;

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(limiter.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * f64::from(limiter.refill_rate)) as u32;

        if tokens_to_add > 0 {
            limiter.tokens = (limiter.tokens + tokens_to_add).min(limiter.capacity);
            limiter.last_refill = now;
        }

        limiter.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(2, 2);

        // Should be able to acquire 2 tokens immediately
        assert!(limiter.try_acquire().await.is_ok());
        assert!(limiter.try_acquire().await.is_ok());

        // Third attempt should fail
        assert!(limiter.try_acquire().await.is_err());

        // Wait for refill
        sleep(Duration::from_millis(600)).await;

        // Should be able to acquire again
        assert!(limiter.try_acquire().await.is_ok());
    }
}
