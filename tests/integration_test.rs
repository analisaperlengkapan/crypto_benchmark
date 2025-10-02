use crypto_benchmark::{signatures, kem, BenchmarkKeys};

#[test]
fn test_benchmark_keys_generation() {
    let keys = BenchmarkKeys::generate().expect("Key generation failed");
    // Ed25519 key should be 32 bytes
    assert_eq!(keys.ed25519_signing.as_bytes().len(), 32);
}

#[test]
fn test_signatures_benchmark_runs() {
    let keys = BenchmarkKeys::generate().expect("Key generation failed");
    signatures::benchmark_signatures_optimized(&keys);
    // No panic means success
}

#[test]
fn test_kem_benchmark_runs() {
    let keys = BenchmarkKeys::generate().expect("Key generation failed");
    kem::benchmark_kem_optimized(&keys);
    // No panic means success
}
