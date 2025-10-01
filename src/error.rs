// Error handling untuk benchmark
use std::fmt;

#[derive(Debug)]
pub enum BenchmarkError {
    KeyGeneration(String),
    SignatureOperation(String),
    VerificationFailed(String),
    EncapsulationFailed(String),
    DecapsulationFailed(String),
    MeasurementError(String),
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BenchmarkError::KeyGeneration(msg) => write!(f, "Key generation failed: {}", msg),
            BenchmarkError::SignatureOperation(msg) => write!(f, "Signature operation failed: {}", msg),
            BenchmarkError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            BenchmarkError::EncapsulationFailed(msg) => write!(f, "Encapsulation failed: {}", msg),
            BenchmarkError::DecapsulationFailed(msg) => write!(f, "Decapsulation failed: {}", msg),
            BenchmarkError::MeasurementError(msg) => write!(f, "Measurement error: {}", msg),
        }
    }
}

impl std::error::Error for BenchmarkError {}

pub type Result<T> = std::result::Result<T, BenchmarkError>;
