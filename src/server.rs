use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use crypto_benchmark::BenchmarkKeys;
use crypto_benchmark::models::BenchmarkReport;
use crypto_benchmark::{signatures, kem};
use std::time::Instant;

// State shared across requests
struct AppState {
    keys: Arc<BenchmarkKeys>,
    // Cache the last result to avoid running benchmarks on every refresh if needed
    // For now, we'll run them every time or maybe cache them?
    // Let's cache them to avoid DDOSing the CPU
    cached_report: Mutex<Option<BenchmarkReport>>,
}

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("⏳ Generating keys for server...");
    let keys = BenchmarkKeys::generate()?;
    println!("✓ Keys generated. Starting server...");

    let state = Arc::new(AppState {
        keys: Arc::new(keys),
        cached_report: Mutex::new(None),
    });

    let app = Router::new()
        .route("/api/benchmarks", post(run_benchmarks))
        .route("/api/benchmarks/cached", get(get_cached_benchmarks))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🌍 Server running at http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn run_benchmarks(State(state): State<Arc<AppState>>) -> Json<BenchmarkReport> {
    // This endpoint triggers a fresh run
    let _start_keygen = Instant::now(); // Mock time, since keys are pre-gen
    // In reality, keygen time is 0 per request because we reuse them
    // But for the report, we might want to show the original keygen time if we stored it
    // For now let's just measure the benchmark time.

    let start_bench = Instant::now();

    // We clone the Arc<Keys> to pass to the blocking functions?
    // The benchmark functions take &BenchmarkKeys.

    // Since benchmarking is CPU intensive, we should probably run it in a blocking task
    // to avoid blocking the async runtime.
    let keys_clone = state.keys.clone();

    let report = tokio::task::spawn_blocking(move || {
        let sig_metrics = signatures::benchmark_signatures_optimized(&keys_clone);
        let kem_metrics = kem::benchmark_kem_optimized(&keys_clone);

        BenchmarkReport {
            signatures: sig_metrics,
            kem: kem_metrics,
            keygen_time_secs: 0.0, // Cached
            total_time_secs: 0.0, // Will be calculated
        }
    }).await.unwrap();

    let mut report = report;
    report.total_time_secs = start_bench.elapsed().as_secs_f64();

    // Update cache
    let mut cache = state.cached_report.lock().await;
    *cache = Some(report.clone());

    Json(report)
}

async fn get_cached_benchmarks(State(state): State<Arc<AppState>>) -> Json<Option<BenchmarkReport>> {
    let cache = state.cached_report.lock().await;
    Json(cache.clone())
}
