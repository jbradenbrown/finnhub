//! Performance benchmarks for the Finnhub client.

use chrono::{Duration, Utc};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use finnhub::{
    models::stock::{CandleResolution, StatementFrequency, StatementType},
    FinnhubClient,
};

fn benchmark_client_creation(c: &mut Criterion) {
    c.bench_function("client creation", |b| {
        b.iter(|| {
            let _client = FinnhubClient::new(black_box("test-api-key"));
        });
    });
}

fn benchmark_stock_endpoints(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = FinnhubClient::new("test_api_key");

    c.bench_function("quote_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = black_box(client.stock().quote("AAPL")).await;
            })
        })
    });

    c.bench_function("company_profile_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = black_box(client.stock().company_profile("AAPL")).await;
            })
        })
    });

    c.bench_function("candles_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let to = Utc::now().timestamp();
                let from = (Utc::now() - Duration::days(7)).timestamp();
                let _ = black_box(client.stock().candles(
                    "AAPL",
                    CandleResolution::Daily,
                    from,
                    to,
                ))
                .await;
            })
        })
    });

    c.bench_function("metrics_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = black_box(client.stock().metrics("AAPL").await);
            })
        })
    });

    c.bench_function("financials_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = black_box(
                    client
                        .stock()
                        .financials(
                            "AAPL",
                            StatementType::IncomeStatement,
                            StatementFrequency::Annual,
                        )
                        .await,
                );
            })
        })
    });

    c.bench_function("historical_market_cap_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let to_date = Utc::now().format("%Y-%m-%d").to_string();
                let from_date = (Utc::now() - Duration::days(30))
                    .format("%Y-%m-%d")
                    .to_string();
                let _ = black_box(
                    client
                        .stock()
                        .historical_market_cap("AAPL", &from_date, &to_date),
                )
                .await;
            })
        })
    });
}

criterion_group!(
    benches,
    benchmark_client_creation,
    benchmark_stock_endpoints
);
criterion_main!(benches);
