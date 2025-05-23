//! Performance benchmarks for the Finnhub client.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use finnhub::FinnhubClient;

fn benchmark_client_creation(c: &mut Criterion) {
    c.bench_function("client creation", |b| {
        b.iter(|| {
            let _client = FinnhubClient::new(black_box("test-api-key"));
        });
    });
}

criterion_group!(benches, benchmark_client_creation);
criterion_main!(benches);