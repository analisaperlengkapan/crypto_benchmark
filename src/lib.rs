// Core modules (now include optimized implementations)
pub mod signatures;
pub mod kem;

// Infrastructure modules
pub mod error;
pub mod constants;
pub mod keys;
pub mod measurement;

// Legacy support - deprecated but kept for compatibility
use std::time::Instant;
use sysinfo::System;

#[deprecated(since = "0.2.0", note = "Use measurement::benchmark_operation instead")]
pub fn measure_resources<F>(sys: &mut System, f: F) -> (u64, u64, u64)
where
    F: FnOnce(),
{
    sys.refresh_all();
    let cpu_before = sys.global_cpu_usage() as u64;
    let mem_before = sys.used_memory();

    let start = Instant::now();
    f();
    let duration = start.elapsed().as_micros() as u64;

    sys.refresh_all();
    let cpu_after = sys.global_cpu_usage() as u64;
    let mem_after = sys.used_memory();

    let cpu_usage = cpu_after.saturating_sub(cpu_before);
    let mem_usage = mem_after.saturating_sub(mem_before);

    (duration, cpu_usage, mem_usage)
}

// Re-exports untuk convenience
pub use error::{BenchmarkError, Result};
pub use constants::*;
pub use keys::BenchmarkKeys;
pub use measurement::{BenchmarkResult, benchmark_operation};
