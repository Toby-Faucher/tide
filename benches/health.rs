use criterion::{Criterion, criterion_group, criterion_main};

fn bench_health_serialization(c: &mut Criterion) {
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct HealthResponse {
        status: &'static str,
    }

    c.bench_function("health_serialize", |b| {
        b.iter(|| serde_json::to_string(&HealthResponse { status: "ok" }).unwrap())
    });
}

criterion_group!(benches, bench_health_serialization);
criterion_main!(benches);
