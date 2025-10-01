// Constants untuk benchmark configuration
pub const DEFAULT_MESSAGE: &[u8] = b"Hello, world! This is a test message for cryptographic benchmarking.";

// Key sizes
pub const ED25519_KEY_SIZE: usize = 32;
pub const RSA_KEY_SIZE: usize = 2048;
pub const ECDSA_KEY_SIZE: usize = 32;

// Benchmark configuration
pub const DEFAULT_WARMUP_ITERATIONS: usize = 10;
pub const DEFAULT_MEASUREMENT_ITERATIONS: usize = 100;
pub const MIN_MEASUREMENT_TIME_MS: u64 = 100;

// Resource measurement
pub const SYSTEM_REFRESH_INTERVAL_MS: u64 = 100;
pub const CPU_SAMPLE_COUNT: usize = 5;
