# Contributing to Crypto Benchmark

First off, thank you for considering contributing to Crypto Benchmark! 🎉

This document provides guidelines for contributing to this project. Following these guidelines helps maintain code quality and makes the contribution process smooth for everyone.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Adding New Algorithms](#adding-new-algorithms)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)

---

## 📜 Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow:

- **Be Respectful**: Treat everyone with respect and consideration
- **Be Collaborative**: Work together and help each other
- **Be Professional**: Keep discussions focused and constructive
- **Be Inclusive**: Welcome contributors of all backgrounds and skill levels

---

## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have:

```bash
# Rust 1.70 or later
rustc --version

# Cargo (comes with Rust)
cargo --version

# Git
git --version
```

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/crypto-benchmark.git
cd crypto-benchmark
```

3. Add upstream remote:

```bash
git remote add upstream https://github.com/ORIGINAL_OWNER/crypto-benchmark.git
```

---

## 🛠️ Development Setup

### Initial Setup

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo run --release
cargo bench
```

### IDE Setup (Recommended)

**VS Code:**
```bash
# Install Rust Analyzer extension
code --install-extension rust-lang.rust-analyzer
```

**Rust Analyzer Configuration** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
```

---

## 🤝 How to Contribute

### Types of Contributions

We welcome:

1. **Bug Fixes**: Fix issues in existing code
2. **New Algorithms**: Add support for new cryptographic algorithms
3. **Performance Improvements**: Optimize existing implementations
4. **Documentation**: Improve docs, examples, or comments
5. **Tests**: Add or improve test coverage
6. **CI/CD**: Improve build and deployment processes

### Contribution Workflow

1. **Create an Issue** (optional but recommended)
   - Describe what you want to do
   - Get feedback before starting work

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

3. **Make Changes**
   - Write code following our [Coding Standards](#coding-standards)
   - Add tests for new functionality
   - Update documentation as needed

4. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   cargo run --release
   cargo bench
   ```

5. **Commit Changes**
   - Follow [Commit Message Guidelines](#commit-message-guidelines)
   ```bash
   git add .
   git commit -m "feat: add support for SPHINCS+ algorithm"
   ```

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   - Go to GitHub and create a Pull Request
   - Fill out the PR template completely

---

## 💻 Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```bash
# Format code automatically
cargo fmt

# Check for common mistakes
cargo clippy -- -D warnings
```

### Code Quality Requirements

**All code must:**
- Pass `cargo test` (all tests pass)
- Pass `cargo clippy` (no warnings)
- Pass `cargo fmt --check` (properly formatted)
- Have appropriate documentation comments
- Include tests for new functionality

### Documentation Standards

**Function Documentation:**
```rust
/// Performs a cryptographic signature operation.
///
/// # Arguments
///
/// * `message` - The message to sign (as byte slice)
///
/// # Returns
///
/// * `Result<Signature, BenchmarkError>` - The signature or error
///
/// # Examples
///
/// ```
/// let sig = sign_message(b"Hello, world!")?;
/// ```
///
/// # Panics
///
/// This function does not panic under normal circumstances.
pub fn sign_message(message: &[u8]) -> Result<Signature, BenchmarkError> {
    // implementation
}
```

### Error Handling

**Always use `Result` types:**
```rust
// Good ✅
pub fn benchmark_operation() -> Result<BenchmarkResult, BenchmarkError> {
    // ...
}

// Bad ❌
pub fn benchmark_operation() -> BenchmarkResult {
    // ... might panic
}
```

### Performance Considerations

**For benchmark code:**
- Use `#[inline]` for hot paths
- Avoid allocations in tight loops
- Pre-generate keys when possible
- Use `black_box()` in Criterion benchmarks

**Example:**
```rust
use std::hint::black_box;

pub fn benchmark_fn(c: &mut Criterion) {
    let key = generate_key();
    c.bench_function("operation", |b| {
        b.iter(|| {
            operation(black_box(&key))
        })
    });
}
```

---

## 🧪 Testing Guidelines

### Unit Tests

**Location**: Same file as implementation, in `#[cfg(test)]` module

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let keys = BenchmarkKeys::generate().unwrap();
        assert!(keys.ed25519_signing.len() == 32);
    }

    #[test]
    fn test_signature_verification() {
        // Test implementation
    }
}
```

### Integration Tests

**Location**: `tests/` directory

```rust
// tests/integration_test.rs
use crypto_benchmark::*;

#[test]
fn test_full_benchmark_workflow() {
    let keys = BenchmarkKeys::generate().unwrap();
    signatures::benchmark_signatures_optimized(&keys);
    // Assertions
}
```

### Benchmark Tests

**Location**: `benches/` directory

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_new_algorithm(c: &mut Criterion) {
    c.bench_function("New Algorithm", |b| {
        b.iter(|| new_algorithm_operation())
    });
}

criterion_group!(benches, benchmark_new_algorithm);
criterion_main!(benches);
```

### Test Coverage

**Running tests:**
```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Doc tests
cargo test --doc
```

---

## 📝 Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks
- **ci**: CI/CD changes

### Examples

```bash
# Feature
git commit -m "feat(signatures): add SPHINCS+ support"

# Bug fix
git commit -m "fix(kem): correct Kyber decapsulation timing"

# Documentation
git commit -m "docs(readme): update benchmark results"

# Performance
git commit -m "perf(keys): optimize key generation with parallel execution"

# With body
git commit -m "feat(algorithms): add Falcon-1024 variant

Added support for Falcon-1024 in addition to Falcon-512.
Updated benchmarks and documentation accordingly.

Closes #123"
```

---

## 🔄 Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)
- [ ] Benchmarks still run correctly

### PR Template

When creating a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested the changes

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] No new warnings
- [ ] Benchmarks verified

## Screenshots (if applicable)
Add screenshots of benchmark results

## Related Issues
Closes #123
```

### Review Process

1. **Automated Checks**: CI/CD runs tests and checks
2. **Code Review**: Maintainers review your code
3. **Feedback**: Address review comments
4. **Approval**: At least one maintainer approval required
5. **Merge**: Maintainer merges the PR

### Review Timeline

- Initial review: Within 3-7 days
- Follow-up reviews: Within 2-3 days
- Complex PRs may take longer

---

## ➕ Adding New Algorithms

### Steps to Add a New Algorithm

#### 1. Add Dependency

**In `Cargo.toml`:**
```toml
[dependencies]
new-crypto-lib = "1.0"
```

#### 2. Create Helper Functions

**In `src/signatures.rs` or `src/kem.rs`:**
```rust
use new_crypto_lib::*;

/// Helper function for Criterion benchmarks
#[allow(dead_code)]
pub fn new_algorithm_sign(message: &[u8]) -> Vec<u8> {
    let (pk, sk) = keypair();
    sign(&sk, message)
}
```

#### 3. Add to BenchmarkKeys

**In `src/keys.rs`:**
```rust
pub struct BenchmarkKeys {
    // ... existing fields
    pub new_algorithm_keypair: (PublicKey, SecretKey),
}

impl BenchmarkKeys {
    pub fn generate() -> Result<Self, BenchmarkError> {
        // ... existing key generation
        
        let new_algorithm_keypair = new_crypto_lib::keypair();
        
        Ok(BenchmarkKeys {
            // ... existing fields
            new_algorithm_keypair,
        })
    }
}
```

#### 4. Add Benchmark Function

**In `src/signatures.rs`:**
```rust
pub fn benchmark_signatures_optimized(keys: &BenchmarkKeys) {
    // ... existing benchmarks
    
    // New Algorithm
    println!("\n--- New Algorithm ---");
    let sign_result = measurement::benchmark_operation(
        || {
            let sig = new_crypto_lib::sign(&keys.new_algorithm_keypair.1, MESSAGE);
            sig
        },
        MEASUREMENT_ITERATIONS,
    );
    println!("Sign:   {}", format_benchmark_result(&sign_result));
}
```

#### 5. Add Criterion Benchmark

**In `benches/crypto_bench.rs`:**
```rust
fn benchmark_signatures(c: &mut Criterion) {
    // ... existing benchmarks
    
    group.bench_function("New Algorithm Sign", |b| {
        b.iter(|| new_algorithm_sign(black_box(b"Hello, world!")))
    });
}
```

#### 6. Update Documentation

- Update README.md with new algorithm results
- Add algorithm details to documentation
- Update performance comparison tables

#### 7. Run Complete Test Suite

```bash
cargo test
cargo clippy
cargo fmt
cargo run --release
cargo bench
```

### Example: Adding SPHINCS+

See [Example PR #XXX](https://github.com/example) for a complete example of adding SPHINCS+.

---

## 🐛 Reporting Bugs

### Before Reporting

1. Check existing issues
2. Verify with latest version
3. Reproduce the bug

### Bug Report Template

```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Run `cargo bench`
2. Observe incorrect timing for algorithm X
3. Compare with expected result

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: Ubuntu 22.04
- Rust version: 1.70
- Cargo version: 1.70
- Project version: 0.2.0

## Additional Context
- Screenshots
- Log output
- Related issues
```

---

## 💡 Suggesting Enhancements

### Enhancement Template

```markdown
## Feature Description
Brief description of the enhancement

## Motivation
Why is this enhancement needed?

## Detailed Design
How would this work?

## Alternatives Considered
What other approaches did you consider?

## Additional Context
- Use cases
- Examples from other projects
- References
```

### Priority Levels

- **High**: Critical functionality or major improvements
- **Medium**: Nice-to-have features
- **Low**: Minor improvements or optimizations

---

## 📚 Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)

### Cryptography Resources

- [Crypto101](https://www.crypto101.io/)
- [Applied Cryptography](https://www.schneier.com/books/applied-cryptography/)
- [PQC Standardization](https://csrc.nist.gov/Projects/post-quantum-cryptography/post-quantum-cryptography-standardization)

---

## 🏆 Recognition

Contributors are recognized in:

- **README.md**: Contributors section
- **CHANGELOG.md**: Release notes
- **GitHub**: Contributor graph

Top contributors may receive:
- Maintainer status
- Special recognition in releases
- Invitation to design discussions

---

## 📞 Getting Help

**Questions?**
- Open a [GitHub Discussion](https://github.com/your-repo/discussions)
- Check existing [Issues](https://github.com/your-repo/issues)
- Read the [Documentation](https://docs.rs/crypto-benchmark)

**Need Help?**
- Join community chat (if available)
- Email maintainers (if applicable)
- Comment on related issues

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing! 🎉**

Your contributions help make cryptographic benchmarking better for everyone.
