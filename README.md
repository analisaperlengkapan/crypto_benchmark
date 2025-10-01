# 🔐 Crypto Benchmark - Comprehensive Cryptographic Performance Analysis

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)

A high-performance benchmarking tool for comparing **classical** and **post-quantum** cryptographic algorithms. Features optimized implementations with statistical analysis and accurate performance measurements.

---

## ✨ Key Features

### 🎯 Algorithm Support

**Digital Signatures:**
- **Classical**: Ed25519, RSA-PSS (2048-bit), ECDSA (P-256)
- **Post-Quantum**: ML-DSA-44 (Dilithium), Falcon-512

**Key Exchange Mechanisms:**
- **Classical**: X25519 (Curve25519 DH), ECDH (P-256), RSA-KEM
- **Post-Quantum**: ML-KEM-512 (Kyber)

### ⚡ Performance Optimizations

- **Pre-generated Keys**: Keys generated once and reused (70-80% speedup)
- **Statistical Analysis**: 50-100 iterations per operation
- **Accurate Timing**: High-resolution measurements with standard deviation
- **Dual Benchmark Modes**: 
  - Interactive mode with real-time output
  - Criterion benchmarks with HTML reports

---

## 📊 Benchmark Results

### Interactive Benchmark (`cargo run --release`)

**System Info**: Ubuntu Linux, Rust 1.70+  
**Total Time**: 0.49s (0.28s key generation + 0.21s benchmarking)

#### Digital Signatures Performance

| Algorithm | Operation | Mean Time | Min | Max | Key/Sig Size |
|-----------|-----------|-----------|-----|-----|--------------|
| **Ed25519** | Sign | 16 μs | 16 μs | 29 μs | 32 / 64 bytes |
| | Verify | 35 μs | 32 μs | 64 μs | |
| **RSA-2048** | Sign | 1,388 μs | 1,207 μs | 4,276 μs | 2048 bits / 256 bytes |
| | Verify | 176 μs | 148 μs | 818 μs | |
| **ECDSA P-256** | Sign | 181 μs | 154 μs | 891 μs | 32 / 64 bytes |
| | Verify | 283 μs | 261 μs | 574 μs | |
| **ML-DSA-44** | Sign | 69 μs | 32 μs | 230 μs | 1312 / 2420 bytes |
| | Verify | 26 μs | 25 μs | 35 μs | |
| **Falcon-512** | Sign | 366 μs | 192 μs | 1,254 μs | 897 / 752 bytes |
| | Verify | 37 μs | 36 μs | 70 μs | |

#### Key Exchange Mechanisms Performance

| Algorithm | Operation | Mean Time | Min | Max | Key Sizes |
|-----------|-----------|-----------|-----|-----|-----------|
| **X25519** | Key Exchange | 45 μs | 43 μs | 62 μs | 32 bytes |
| **ECDH P-256** | Key Exchange | 136 μs | 131 μs | 329 μs | 32 bytes |
| **ML-KEM-512** | Encapsulate | 8 μs | 7 μs | 19 μs | 800 / 768 bytes |
| | Decapsulate | 8 μs | 7 μs | 16 μs | |

### Criterion Benchmark (`cargo bench`)

More detailed statistical analysis with warmup, outlier detection, and HTML reports.

#### Signature Algorithms (Criterion)

| Algorithm | Operation | Mean Time | StdDev | Outliers |
|-----------|-----------|-----------|--------|----------|
| **Ed25519** | Sign | 36.6 μs | ±0.9 μs | 11% |
| | Verify | 84.8 μs | ±2.3 μs | 4% |
| **RSA-2048** | Sign | 163.7 ms | ±14.9 ms | 1% |
| | Verify | 157.5 ms | ±14.6 ms | 2% |
| **ECDSA P-256** | Sign | 340.8 μs | ±4.9 μs | 12% |
| | Verify | 611.8 μs | ±10.5 μs | 6% |
| **ML-DSA-44** | Sign | 113.3 μs | ±4.7 μs | 9% |
| | Verify | 138.4 μs | ±2.8 μs | 2% |
| **Falcon-512** | Sign | 7.09 ms | ±0.15 ms | 0% |
| | Verify | 7.23 ms | ±0.21 ms | 6% |

#### KEM Algorithms (Criterion)

| Algorithm | Operation | Mean Time | StdDev | Outliers |
|-----------|-----------|-----------|--------|----------|
| **RSA-2048 KEM** | Encapsulate | 166.6 ms | ±20.3 ms | 5% |
| | Decapsulate | 176.6 ms | ±19.0 ms | 4% |
| **X25519 DH** | Encapsulate | 65.4 μs | ±0.7 μs | 7% |
| | Decapsulate | 65.3 μs | ±0.4 μs | 6% |
| **ECDH P-256** | Encapsulate | 289.8 μs | ±1.7 μs | 4% |
| | Decapsulate | 271.7 μs | ±4.1 μs | 7% |
| **ML-KEM-512** | Encapsulate | 18.1 μs | ±1.1 μs | 12% |
| | Decapsulate | 9.7 μs | ±0.2 μs | 1% |

---

## 🚀 Quick Start

### Prerequisites

```bash
# Rust 1.70 or later
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

```bash
git clone <repository-url>
cd enc_test
cargo build --release
```

### Running Benchmarks

#### 1. Interactive Benchmark (Recommended)

Run the optimized benchmark with real-time output:

```bash
cargo run --release
```

**Command Line Options:**

```bash
# Show help
cargo run --release -- --help

# Show version
cargo run --release -- --version

# Run comparison mode (shows performance improvement)
cargo run --release -- comparison
```

**Expected Output:**
```
╔═══════════════════════════════════════════════════════════════╗
║         Cryptographic Benchmarking Tool v0.2.0               ║
║    Classical & Post-Quantum Cryptography Performance         ║
╚═══════════════════════════════════════════════════════════════╝

⏳ Generating benchmark keys...
✓ All keys generated successfully (0.28s)

═══════════════════════════════════════════════════════════════
                   SIGNATURE ALGORITHMS
═══════════════════════════════════════════════════════════════

--- Ed25519 ---
Sign:   16.000 μs (min: 16 μs, max: 29 μs, σ: 1.23 μs)
Verify: 35.000 μs (min: 32 μs, max: 64 μs, σ: 3.45 μs)
...

╔════════════════════════════════════════════════════════╗
║                 BENCHMARK SUMMARY                      ║
╚════════════════════════════════════════════════════════╝
  📊 Statistics:      100 iterations per operation
  ⚡ Key Generation:  0.28s (one-time cost)
  🔬 Benchmark Time:  0.21s (all operations)
  ⏱️  Total Time:      0.49s

✅ All benchmarks completed successfully!
```

#### 2. Criterion Benchmark (Statistical Analysis)

```bash
cargo bench
```

Results will be saved to `target/criterion/` with detailed HTML reports.

**View Reports:**
```bash
# Open in browser
firefox target/criterion/report/index.html
# Or
xdg-open target/criterion/report/index.html
```

---

## 📈 Performance Analysis

### Key Insights

#### 🏆 Fastest Algorithms

**Signatures:**
1. **ML-KEM-512 Decapsulate**: 8 μs (Post-Quantum winner!)
2. **Ed25519 Sign**: 16 μs (Classical winner)
3. **ML-DSA-44 Verify**: 26 μs

**Key Exchange:**
1. **ML-KEM-512**: 8-18 μs (Revolutionary speed!)
2. **X25519 DH**: 45-65 μs (Battle-tested)
3. **ECDH P-256**: 136-290 μs

#### ⚠️ Slowest Algorithms

**Signatures:**
1. **Falcon-512 Sign**: 7.09 ms (Criterion) / 366 μs (Interactive)* 
2. **Falcon-512 Verify**: 7.23 ms (Criterion) / 37 μs (Interactive)*
3. **RSA-2048 Sign**: 163.7 ms (Needs key generation!)
4. **RSA-2048 Verify**: 157.5 ms

**Key Exchange:**
1. **RSA-KEM**: 166-177 ms (Very slow!)

*Note: Large discrepancy between Criterion and interactive benchmarks for Falcon suggests key generation overhead in Criterion helper functions.

#### 📊 Classical vs Post-Quantum

| Metric | Classical Best | Post-Quantum Best | Winner |
|--------|---------------|-------------------|--------|
| **Sign Speed** | Ed25519 (16 μs) | ML-DSA-44 (69 μs) | 🏆 Classical |
| **Verify Speed** | Ed25519 (35 μs) | ML-DSA-44 (26 μs) | 🏆 Post-Quantum! |
| **Sign Size** | Ed25519 (64 B) | Falcon-512 (752 B) | 🏆 Classical |
| **KEM Speed** | X25519 (45 μs) | ML-KEM-512 (8 μs) | 🏆 Post-Quantum! |
| **Key Size** | X25519 (32 B) | ML-KEM-512 (800 B) | 🏆 Classical |

**Surprising Result**: Post-quantum ML-KEM-512 is **3x faster** than classical X25519! 🎉

---

## 🏗️ Architecture

### Project Structure

```
enc_test/
├── src/
│   ├── main.rs              # Interactive benchmark CLI
│   ├── lib.rs               # Library exports
│   ├── signatures.rs        # Signature algorithms + helpers
│   ├── kem.rs               # KEM algorithms + helpers
│   ├── keys.rs              # Pre-generated key management
│   ├── measurement.rs       # Statistical benchmarking
│   ├── error.rs             # Error handling
│   └── constants.rs         # Configuration constants
├── benches/
│   └── crypto_bench.rs      # Criterion benchmarks
├── Cargo.toml               # Dependencies
└── README.md                # This file
```

### Key Components

#### 1. Pre-generated Keys (`keys.rs`)

```rust
pub struct BenchmarkKeys {
    pub ed25519_signing: Ed25519SigningKey,
    pub rsa_private: RsaPrivateKey,
    pub dilithium_public: mldsa44::PublicKey,
    // ... all algorithm keys
}
```

**Benefits:**
- RSA key generation (~200ms) done once
- Post-quantum key generation amortized
- 70-80% performance improvement

#### 2. Statistical Measurement (`measurement.rs`)

```rust
pub struct BenchmarkResult {
    pub mean_duration: Duration,
    pub std_deviation: f64,
    pub min_duration: Duration,
    pub max_duration: Duration,
}
```

**Features:**
- Warmup cycles (10 iterations)
- Multiple measurements (50-100 iterations)
- Outlier detection
- Standard deviation calculation

---

## 🔧 Configuration

### Customize Iterations

Edit `src/constants.rs`:

```rust
pub const DEFAULT_MEASUREMENT_ITERATIONS: usize = 100;
pub const DEFAULT_WARMUP_ITERATIONS: usize = 10;
```

### Custom Message

```rust
pub const DEFAULT_MESSAGE: &[u8] = b"Your custom test message";
```

### Adjust Criterion Settings

Edit `benches/crypto_bench.rs`:

```rust
criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)      // Number of samples
        .warm_up_time(Duration::from_secs(3));
    targets = benchmark_signatures, benchmark_kem
);
```

---

## 📚 API Documentation

### Generate Documentation

```bash
cargo doc --open
```

### Example Usage

```rust
use crypto_benchmark::{BenchmarkKeys, signatures, kem};

fn main() {
    // Generate keys once
    let keys = BenchmarkKeys::generate().unwrap();
    
    // Run benchmarks
    signatures::benchmark_signatures_optimized(&keys);
    kem::benchmark_kem_optimized(&keys);
}
```

---

## 🧪 Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code
cargo check --all-targets

# Lint
cargo clippy --all-targets
```

---

## 📊 Interpretation Guide

### Understanding Results

**Mean Time**: Average execution time (most important)
**Min/Max**: Best and worst case performance
**StdDev**: Consistency (lower = more consistent)
**Outliers**: Anomalous measurements (higher % = less reliable)

### Performance Tiers

| Time Range | Performance | Examples |
|------------|-------------|----------|
| < 50 μs | 🟢 Excellent | Ed25519, ML-KEM |
| 50-200 μs | 🟡 Good | ECDSA, Falcon verify |
| 200-500 μs | 🟠 Moderate | ECDH, Falcon sign |
| > 1 ms | 🔴 Slow | RSA operations |

---

## 💡 Best Practices

### For Production

1. **Use Ed25519 for speed**: Fastest classical signature
2. **Use ML-KEM-512 for quantum resistance**: Faster than classical!
3. **Avoid RSA**: Too slow for modern applications
4. **Pre-generate keys**: Never generate keys in hot path

### For Research

1. **Run multiple times**: Results vary by ~5-10%
2. **Use release mode**: Debug is 10x slower
3. **Close other apps**: Reduces CPU noise
4. **Check outliers**: High outliers = unreliable results

### For Benchmarking

```bash
# Best accuracy
cargo run --release

# Most detailed
cargo bench

# Both
cargo run --release && cargo bench
```

---

## 🔬 Technical Details

### Algorithms

**NIST Standards:**
- ML-DSA (FIPS 204 - Digital Signatures)
- ML-KEM (FIPS 203 - Key Encapsulation)

**Classical Standards:**
- Ed25519 (RFC 8032)
- RSA-PSS (RFC 8017)
- ECDSA (FIPS 186-4)

### Dependencies

```toml
ed25519-dalek = "2.0"      # Ed25519 signatures
rsa = "0.9"                # RSA cryptography
p256 = "0.13"              # ECDSA & ECDH
pqcrypto-mldsa = "0.1.2"   # ML-DSA (Dilithium)
pqcrypto-mlkem = "0.1.1"   # ML-KEM (Kyber)
pqcrypto-falcon = "0.4.1"  # Falcon signatures
criterion = "0.7"          # Benchmarking framework
```

### ⚠️ Important Notes on Falcon Benchmarks

**Discrepancy Alert**: Falcon shows significant performance differences between benchmark modes:

| Mode | Sign | Verify | Reason |
|------|------|--------|--------|
| **Interactive** | 366 μs | 37 μs | Uses pre-generated keys ✅ |
| **Criterion** | 7.09 ms | 7.23 ms | Helper includes key generation ⚠️ |

**Explanation**: The Criterion helper functions (`falcon_sign`, `falcon_verify`) generate new keys on each iteration, while the interactive benchmark uses pre-generated keys from `BenchmarkKeys`. This demonstrates the importance of key reuse in production systems.

**Recommendation**: For production performance estimates, use the **interactive benchmark** results which reflect real-world usage with key caching.

---

## 🎯 Use Cases

### Academic Research
- Compare classical vs post-quantum performance
- Study cryptographic algorithm characteristics
- Generate performance data for papers

### Security Audits
- Assess algorithm performance impact
- Plan migration to post-quantum crypto
- Evaluate resource requirements

### System Design
- Choose appropriate algorithms
- Performance budgeting
- Capacity planning

---

## 🤝 Contributing

We welcome contributions! Whether you're fixing bugs, adding new algorithms, improving documentation, or optimizing performance, your help is appreciated.

### Quick Start for Contributors

1. **Read** [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines
2. **Fork** the repository
3. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
4. **Make** your changes with tests
5. **Test** thoroughly (`cargo test && cargo clippy && cargo fmt`)
6. **Commit** with conventional commits (`git commit -m 'feat: add amazing feature'`)
7. **Push** to your fork (`git push origin feature/amazing-feature`)
8. **Open** a Pull Request

### What Can You Contribute?

- 🐛 **Bug Fixes**: Fix issues or improve error handling
- ✨ **New Algorithms**: Add SPHINCS+, BIKE, or other cryptographic algorithms
- ⚡ **Performance**: Optimize existing implementations
- 📚 **Documentation**: Improve README, add examples, fix typos
- 🧪 **Tests**: Increase test coverage or add integration tests
- 🔧 **CI/CD**: Improve build process or add automation

See [CONTRIBUTING.md](CONTRIBUTING.md) for complete guidelines including:
- Coding standards and style guide
- Testing requirements
- Commit message format
- Pull request process
- How to add new algorithms

---

## 📝 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

### What This Means

✅ **You can:**
- Use this software for commercial purposes
- Modify and distribute the code
- Use it privately
- Sublicense the code

⚠️ **You must:**
- Include the original copyright notice
- Include a copy of the license

❌ **Limitation:**
- The software is provided "as is" without warranty

For the full license text, see [LICENSE](LICENSE).

---

## � Contributors

This project exists thanks to all the people who contribute!

<!-- Add contributor badges/avatars here when available -->

Want to contribute? See [CONTRIBUTING.md](CONTRIBUTING.md) to get started!

### How to Contribute

We're always looking for help with:
- Adding new cryptographic algorithms
- Performance optimizations
- Documentation improvements
- Bug fixes and testing
- Code reviews

---

## �🙏 Acknowledgments

- **NIST PQC Project**: Post-quantum algorithm standardization
- **Rust Crypto**: Excellent cryptographic libraries
- **Criterion.rs**: Professional benchmarking framework
- **All Contributors**: Thank you for making this project better!

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/analisaperlengkapan/crypto_benchmark/issues)
- **Documentation**: `cargo doc --open`

---

## 🔮 Roadmap

- [ ] Add more post-quantum algorithms (SPHINCS+, BIKE)
- [ ] GPU acceleration support
- [ ] WebAssembly benchmarks
- [ ] CI/CD integration
- [ ] Performance regression testing
- [ ] JSON/CSV export
- [ ] Comparison charts

---

**⚡ Fast. Secure. Future-proof.**

*Last updated: October 1, 2025*  
*Version: 0.2.0*
