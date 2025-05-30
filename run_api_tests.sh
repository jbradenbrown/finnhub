#!/bin/bash
# Run all API integration tests
# 
# The tests use dotenv to load FINNHUB_API_KEY from .env file
# Create a .env file with: FINNHUB_API_KEY=your_api_key_here
#
# Usage: ./run_api_tests.sh

set -e

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "Error: .env file not found"
    echo "Create a .env file with:"
    echo "  FINNHUB_API_KEY=your_api_key_here"
    exit 1
fi

# Check if API key is in .env
if ! grep -q "FINNHUB_API_KEY=" .env; then
    echo "Error: FINNHUB_API_KEY not found in .env file"
    echo "Add to .env file:"
    echo "  FINNHUB_API_KEY=your_api_key_here"
    exit 1
fi

echo "=== Running Finnhub API Integration Tests ==="
echo "Using API key from .env file"
echo ""

# Run all ignored tests with output
echo "Running tests with rate limiting (sequential to avoid hitting limits)..."
cargo test --all-features -- --ignored --test-threads=1 --nocapture

echo ""
echo "=== Test Summary ==="
echo "To run specific endpoint tests:"
echo "  cargo test test_forex --features websocket -- --ignored --nocapture"
echo "  cargo test test_stock --features websocket -- --ignored --nocapture"
echo ""
echo "To run tests in parallel (may hit rate limits):"
echo "  cargo test --all-features -- --ignored --nocapture"