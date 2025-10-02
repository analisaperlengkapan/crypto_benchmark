# Testing Guide

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Types

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Benchmark tests
cargo test --benches

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_ed25519_sign_and_verify
```

## Test Structure

### Unit Tests

Located in `src/` files with `#[cfg(test)]` modules:

- **`src/signatures.rs`**: Test Ed25519 signing and verification
- **`src/kem.rs`**: Test Kyber encapsulation and decapsulation

### Integration Tests

Located in `tests/integration_test.rs`:

- `test_benchmark_keys_generation`: Verifies key generation works
- `test_signatures_benchmark_runs`: Ensures signature benchmarks complete
- `test_kem_benchmark_runs`: Ensures KEM benchmarks complete

### Benchmark Tests

Located in `benches/crypto_bench.rs`:

- Tests all Criterion benchmarks can run
- Validates algorithm implementations

## Test Coverage

### Current Coverage

✅ **Unit Tests** (2 tests)
- Ed25519 sign/verify
- Kyber encapsulate/decapsulate

✅ **Integration Tests** (3 tests)
- Key generation
- Signature benchmarks
- KEM benchmarks

✅ **Benchmark Tests** (18 benchmarks)
- All signature algorithms
- All KEM algorithms

## Writing New Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BenchmarkKeys;

    #[test]
    fn test_your_feature() {
        let keys = BenchmarkKeys::generate().unwrap();
        // Your test code
        assert_eq!(expected, actual);
    }
}
```

### Integration Test Template

```rust
use crypto_benchmark::{signatures, kem, BenchmarkKeys};

#[test]
fn test_new_feature() {
    let keys = BenchmarkKeys::generate().expect("Key generation failed");
    // Test your feature
}
```

## Test Results

### Last Test Run

```
running 2 tests (unit)
test kem::tests::test_kyber_encapsulate_decapsulate ... ok
test signatures::tests::test_ed25519_sign_and_verify ... ok

running 3 tests (integration)
test test_benchmark_keys_generation ... ok
test test_kem_benchmark_runs ... ok
test test_signatures_benchmark_runs ... ok

running 18 tests (benchmarks)
All benchmark tests ... ok

Total: 23 tests passed ✅
```

## Continuous Integration

When adding tests:

1. Ensure all tests pass: `cargo test`
2. Check no warnings: `cargo clippy`
3. Format code: `cargo fmt`
4. Verify benchmarks: `cargo bench`

## Common Issues

### Slow Tests

Key generation takes ~50 seconds. This is expected for cryptographic key generation.

### Flaky Tests

Benchmark tests may vary slightly between runs due to system load. This is normal.

### Memory Issues

If tests fail due to memory, reduce `BENCH_ITERATIONS` in test files.

## Notes

- Unit tests use pre-generated keys from `BenchmarkKeys`
- Integration tests verify end-to-end functionality
- Benchmark tests validate algorithm implementations
- All tests should complete without panics

## Future Improvements

- [ ] Add more unit tests for each algorithm
- [ ] Add property-based tests
- [ ] Add performance regression tests
- [ ] Add fuzzing tests
- [ ] Increase test coverage to 80%+
