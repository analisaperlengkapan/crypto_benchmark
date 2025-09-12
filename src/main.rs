mod signatures;
mod kem;

use crypto_benchmark::measure_resources;
use sysinfo::System;

fn main() {
    println!("Cryptographic Benchmarking Tool");
    println!("=================================");

    // Initialize system info
    let mut sys = System::new_all();

    // Benchmark Signatures
    println!("\nBenchmarking Signatures:");
    crypto_benchmark::signatures::benchmark_signatures(&mut sys);

    // Benchmark KEM
    println!("\nBenchmarking Key Encapsulation Mechanisms:");
    crypto_benchmark::kem::benchmark_kem(&mut sys);
}
