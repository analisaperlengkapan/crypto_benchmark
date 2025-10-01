// Improved measurement system
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub mean_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub std_deviation: f64,
    pub iterations: usize,
}

impl BenchmarkResult {
    pub fn print(&self, label: &str) {
        println!("  {}", label);
        println!("    Mean:   {:>10.2} μs", self.mean_duration.as_micros() as f64);
        println!("    Min:    {:>10.2} μs", self.min_duration.as_micros() as f64);
        println!("    Max:    {:>10.2} μs", self.max_duration.as_micros() as f64);
        println!("    StdDev: {:>10.2} μs", self.std_deviation);
        println!("    Iterations: {}", self.iterations);
    }
    
    pub fn mean_micros(&self) -> u64 {
        self.mean_duration.as_micros() as u64
    }
}

/// Accurate micro-benchmark dengan statistical analysis
/// Lebih reliable daripada single-run measurement
pub fn benchmark_operation<F, R>(mut f: F, iterations: usize) -> BenchmarkResult
where
    F: FnMut() -> R,
{
    let mut durations = Vec::with_capacity(iterations);
    
    // Warmup - penting untuk cache warming
    for _ in 0..10.min(iterations / 10) {
        let _ = f();
    }
    
    // Actual measurements
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = f();
        let duration = start.elapsed();
        durations.push(duration);
    }
    
    // Calculate statistics
    let total: Duration = durations.iter().sum();
    let mean = total / iterations as u32;
    
    let min = *durations.iter().min().unwrap();
    let max = *durations.iter().max().unwrap();
    
    // Standard deviation
    let mean_micros = mean.as_micros() as f64;
    let variance: f64 = durations
        .iter()
        .map(|d| {
            let diff = d.as_micros() as f64 - mean_micros;
            diff * diff
        })
        .sum::<f64>()
        / iterations as f64;
    let std_deviation = variance.sqrt();
    
    BenchmarkResult {
        mean_duration: mean,
        min_duration: min,
        max_duration: max,
        std_deviation,
        iterations,
    }
}

/// Quick benchmark untuk operasi yang sangat cepat
pub fn quick_benchmark<F, R>(f: F) -> Duration
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let _ = f();
    start.elapsed()
}
