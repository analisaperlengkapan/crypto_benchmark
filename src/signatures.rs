// Optimized signatures module dengan pre-generated keys
use crate::keys::BenchmarkKeys;
use crate::measurement::benchmark_operation;
use crate::constants::DEFAULT_MESSAGE;
use ed25519_dalek::Signer;
use rsa::Pss;
use sha2::{Sha256, Digest};
use pqcrypto_mldsa::mldsa44;
use pqcrypto_falcon::falcon512;

const BENCH_ITERATIONS: usize = 100;

pub fn benchmark_signatures_optimized(keys: &BenchmarkKeys) {
    println!("\n=== OPTIMIZED SIGNATURES BENCHMARK ===\n");
    
    // Ed25519
    println!("Ed25519 Signature:");
    benchmark_ed25519_optimized(keys);

    // RSA
    println!("\nRSA Signature:");
    benchmark_rsa_optimized(keys);

    // ECDSA
    println!("\nECDSA Signature:");
    benchmark_ecdsa_optimized(keys);

    // Dilithium (ML-DSA)
    println!("\nDilithium (ML-DSA-44) Signature:");
    benchmark_dilithium_optimized(keys);

    // Falcon
    println!("\nFalcon-512 Signature:");
    benchmark_falcon_optimized(keys);
}

fn benchmark_ed25519_optimized(keys: &BenchmarkKeys) {
    let message = DEFAULT_MESSAGE;
    
    // Benchmark signing
    let sign_result = benchmark_operation(
        || keys.ed25519_signing.sign(message),
        BENCH_ITERATIONS
    );
    
    // Pre-generate signature untuk verification benchmark
    let signature = keys.ed25519_signing.sign(message);
    
    // Benchmark verification
    let verify_result = benchmark_operation(
        || {
            use ed25519_dalek::Verifier;
            keys.ed25519_verifying.verify(message, &signature).expect("Verification failed")
        },
        BENCH_ITERATIONS
    );
    
    println!("  Key Size: 32 bytes");
    println!("  Signature Size: {} bytes", signature.to_bytes().len());
    sign_result.print("Sign Performance:");
    verify_result.print("Verify Performance:");
}

fn benchmark_rsa_optimized(keys: &BenchmarkKeys) {
    let message = DEFAULT_MESSAGE;
    
    // Pre-compute hash
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed = hasher.finalize();
    
    // Benchmark signing
    let sign_result = benchmark_operation(
        || {
            let padding = Pss::new::<Sha256>();
            keys.rsa_private.sign_with_rng(&mut rand::rngs::OsRng, padding, &hashed)
                .expect("RSA signing failed")
        },
        50  // RSA is slower, fewer iterations
    );
    
    // Pre-generate signature
    let padding = Pss::new::<Sha256>();
    let signature = keys.rsa_private.sign_with_rng(&mut rand::rngs::OsRng, padding, &hashed)
        .expect("RSA signing failed");
    
    // Benchmark verification
    let verify_result = benchmark_operation(
        || {
            let padding = Pss::new::<Sha256>();
            keys.rsa_public.verify(padding, &hashed, &signature)
                .expect("RSA verification failed")
        },
        BENCH_ITERATIONS
    );
    
    println!("  Key Size: 2048 bits");
    println!("  Signature Size: {} bytes", signature.len());
    sign_result.print("Sign Performance:");
    verify_result.print("Verify Performance:");
}

fn benchmark_ecdsa_optimized(keys: &BenchmarkKeys) {
    let message = DEFAULT_MESSAGE;
    
    // Benchmark signing
    let sign_result = benchmark_operation(
        || {
            let signature: p256::ecdsa::Signature = keys.ecdsa_signing.sign(message);
            signature
        },
        BENCH_ITERATIONS
    );
    
    // Pre-generate signature
    let signature: p256::ecdsa::Signature = keys.ecdsa_signing.sign(message);
    
    // Benchmark verification
    let verify_result = benchmark_operation(
        || {
            use p256::ecdsa::signature::Verifier;
            keys.ecdsa_verifying.verify(message, &signature)
                .expect("ECDSA verification failed")
        },
        BENCH_ITERATIONS
    );
    
    println!("  Key Size: 32 bytes");
    println!("  Signature Size: {} bytes", signature.to_vec().len());
    sign_result.print("Sign Performance:");
    verify_result.print("Verify Performance:");
}

fn benchmark_dilithium_optimized(keys: &BenchmarkKeys) {
    let message = DEFAULT_MESSAGE;
    
    // Benchmark signing
    let sign_result = benchmark_operation(
        || mldsa44::sign(message, &keys.dilithium_secret),
        50  // PQC is slower
    );
    
    // Pre-generate signature
    let signature = mldsa44::sign(message, &keys.dilithium_secret);
    
    // Benchmark verification
    let verify_result = benchmark_operation(
        || {
            mldsa44::open(&signature, &keys.dilithium_public)
                .expect("Dilithium verification failed")
        },
        BENCH_ITERATIONS
    );
    
    println!("  Public Key Size: {} bytes", mldsa44::public_key_bytes());
    println!("  Secret Key Size: {} bytes", mldsa44::secret_key_bytes());
    println!("  Signature Size: {} bytes", mldsa44::signature_bytes());
    sign_result.print("Sign Performance:");
    verify_result.print("Verify Performance:");
}

fn benchmark_falcon_optimized(keys: &BenchmarkKeys) {
    let message = DEFAULT_MESSAGE;
    
    // Benchmark signing
    let sign_result = benchmark_operation(
        || falcon512::sign(message, &keys.falcon_secret),
        50  // PQC is slower
    );
    
    // Pre-generate signature
    let signature = falcon512::sign(message, &keys.falcon_secret);
    
    // Benchmark verification
    let verify_result = benchmark_operation(
        || {
            falcon512::open(&signature, &keys.falcon_public)
                .expect("Falcon verification failed")
        },
        BENCH_ITERATIONS
    );
    
    println!("  Public Key Size: {} bytes", falcon512::public_key_bytes());
    println!("  Secret Key Size: {} bytes", falcon512::secret_key_bytes());
    println!("  Signature Size: {} bytes", falcon512::signature_bytes());
    sign_result.print("Sign Performance:");
    verify_result.print("Verify Performance:");
}

// Helper functions for Criterion benchmarks
use ed25519_dalek::Signature as Ed25519Signature;
use p256::ecdsa::SigningKey as P256SigningKey;
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::{RsaPrivateKey, RsaPublicKey};

#[allow(dead_code)]
pub fn ed25519_sign(message: &[u8]) -> Ed25519Signature {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_bytes);
    signing_key.sign(message)
}

#[allow(dead_code)]
pub fn ed25519_verify(message: &[u8]) {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(message);
    use ed25519_dalek::Verifier;
    verifying_key.verify(message, &signature).unwrap();
}

#[allow(dead_code)]
pub fn rsa_sign(message: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let padding = Pss::new::<Sha256>();
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed = hasher.finalize();
    private_key.sign_with_rng(&mut rng, padding, &hashed).unwrap()
}

#[allow(dead_code)]
pub fn rsa_verify(message: &[u8]) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let padding = Pss::new::<Sha256>();
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed = hasher.finalize();
    let signature = private_key.sign_with_rng(&mut rng, Pss::new::<Sha256>(), &hashed).unwrap();
    public_key.verify(padding, &hashed, &signature).unwrap();
}

#[allow(dead_code)]
pub fn ecdsa_sign(message: &[u8]) -> p256::ecdsa::Signature {
    let mut rng = OsRng;
    let signing_key = P256SigningKey::random(&mut rng);
    signing_key.sign(message)
}

#[allow(dead_code)]
pub fn ecdsa_verify(message: &[u8]) {
    let mut rng = OsRng;
    let signing_key = P256SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let signature: p256::ecdsa::Signature = signing_key.sign(message);
    use p256::ecdsa::signature::Verifier;
    verifying_key.verify(message, &signature).unwrap();
}

#[allow(dead_code)]
pub fn dilithium_sign(message: &[u8]) -> pqcrypto_mldsa::mldsa44::SignedMessage {
    let (_, sk) = mldsa44::keypair();
    mldsa44::sign(message, &sk)
}

#[allow(dead_code)]
pub fn dilithium_verify(message: &[u8]) {
    let (pk, sk) = mldsa44::keypair();
    let signature = mldsa44::sign(message, &sk);
    mldsa44::open(&signature, &pk).unwrap();
}

#[allow(dead_code)]
pub fn falcon_sign(message: &[u8]) -> pqcrypto_falcon::falcon512::SignedMessage {
    let (_, sk) = falcon512::keypair();
    falcon512::sign(message, &sk)
}

#[allow(dead_code)]
pub fn falcon_verify(message: &[u8]) {
    let (pk, sk) = falcon512::keypair();
    let signature = falcon512::sign(message, &sk);
    falcon512::open(&signature, &pk).unwrap();
}
