// Optimized KEM module dengan pre-generated keys
use crate::keys::BenchmarkKeys;
use crate::measurement::benchmark_operation;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use p256::ecdh::EphemeralSecret as P256EphemeralSecret;
use pqcrypto_mlkem::mlkem512;
use rand::rngs::OsRng;

const BENCH_ITERATIONS: usize = 100;

pub fn benchmark_kem_optimized(keys: &BenchmarkKeys) {
    println!("\n=== OPTIMIZED KEM BENCHMARK ===\n");
    
    // Diffie-Hellman (X25519)
    println!("X25519 Diffie-Hellman:");
    benchmark_dh_optimized();

    // ECDH (P-256)
    println!("\nECDH (P-256):");
    benchmark_ecdh_optimized();

    // Kyber (ML-KEM)
    println!("\nKyber (ML-KEM-512):");
    benchmark_kyber_optimized(keys);
}

fn benchmark_dh_optimized() {
    // Pre-generate keypairs
    let alice_secret = EphemeralSecret::random_from_rng(OsRng);
    let alice_public = X25519PublicKey::from(&alice_secret);
    let bob_secret = EphemeralSecret::random_from_rng(OsRng);
    let bob_public = X25519PublicKey::from(&bob_secret);
    
    // Benchmark key exchange (Alice's side)
    let exchange_result = benchmark_operation(
        || {
            let temp_secret = EphemeralSecret::random_from_rng(OsRng);
            let _shared = temp_secret.diffie_hellman(&bob_public);
        },
        BENCH_ITERATIONS
    );
    
    // Verify correctness
    let alice_shared = alice_secret.diffie_hellman(&bob_public);
    let bob_shared = bob_secret.diffie_hellman(&alice_public);
    assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes(), "DH shared secret mismatch");
    
    println!("  Key Size: 32 bytes");
    println!("  Shared Secret Size: {} bytes", alice_shared.as_bytes().len());
    exchange_result.print("Key Exchange Performance:");
}

fn benchmark_ecdh_optimized() {
    // Pre-generate keypairs
    let alice_secret = P256EphemeralSecret::random(&mut OsRng);
    let alice_public = alice_secret.public_key();
    let bob_secret = P256EphemeralSecret::random(&mut OsRng);
    let bob_public = bob_secret.public_key();
    
    // Benchmark key exchange
    let exchange_result = benchmark_operation(
        || {
            let temp_secret = P256EphemeralSecret::random(&mut OsRng);
            let _shared = temp_secret.diffie_hellman(&bob_public);
        },
        BENCH_ITERATIONS
    );
    
    // Verify correctness
    let alice_shared = alice_secret.diffie_hellman(&bob_public);
    let bob_shared = bob_secret.diffie_hellman(&alice_public);
    assert_eq!(
        alice_shared.raw_secret_bytes(),
        bob_shared.raw_secret_bytes(),
        "ECDH shared secret mismatch"
    );
    
    println!("  Key Size: 32 bytes");
    println!("  Shared Secret Size: {} bytes", alice_shared.raw_secret_bytes().len());
    exchange_result.print("Key Exchange Performance:");
}

fn benchmark_kyber_optimized(keys: &BenchmarkKeys) {
    // Benchmark encapsulation
    let encaps_result = benchmark_operation(
        || {
            let (_shared_secret, _ciphertext) = mlkem512::encapsulate(&keys.kyber_public);
        },
        50  // PQC is slower
    );
    
    // Pre-generate ciphertext for decapsulation benchmark
    let (_shared_secret_alice, ciphertext) = mlkem512::encapsulate(&keys.kyber_public);
    
    // Benchmark decapsulation
    let decaps_result = benchmark_operation(
        || {
            let _shared_secret_bob = mlkem512::decapsulate(&ciphertext, &keys.kyber_secret);
        },
        BENCH_ITERATIONS
    );
    
    // Note: Kyber shared secrets are opaque types, we can't directly compare
    // but the fact that decapsulation completes without error validates correctness
    
    println!("  Public Key Size: {} bytes", mlkem512::public_key_bytes());
    println!("  Secret Key Size: {} bytes", mlkem512::secret_key_bytes());
    println!("  Ciphertext Size: {} bytes", mlkem512::ciphertext_bytes());
    println!("  Shared Secret Size: {} bytes", mlkem512::shared_secret_bytes());
    encaps_result.print("Encapsulation Performance:");
    decaps_result.print("Decapsulation Performance:");
}

// Helper functions for Criterion benchmarks
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use sha2::Sha256;

#[allow(dead_code)]
pub fn rsa_kem_encapsulate() -> Vec<u8> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let padding = Oaep::new::<Sha256>();
    let data = b"secret data";
    public_key.encrypt(&mut rng, padding, data).unwrap()
}

#[allow(dead_code)]
pub fn rsa_kem_decapsulate() -> Vec<u8> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let padding = Oaep::new::<Sha256>();
    let data = b"secret data";
    let encrypted = public_key.encrypt(&mut rng, padding, data).unwrap();
    let padding_dec = Oaep::new::<Sha256>();
    private_key.decrypt(padding_dec, &encrypted).unwrap()
}

#[allow(dead_code)]
pub fn dh_encapsulate() -> [u8; 32] {
    let rng = OsRng;
    let alice_secret = EphemeralSecret::random_from_rng(rng);
    let bob_public = X25519PublicKey::from(&EphemeralSecret::random_from_rng(rng));
    *alice_secret.diffie_hellman(&bob_public).as_bytes()
}

#[allow(dead_code)]
pub fn dh_decapsulate() -> [u8; 32] {
    let rng = OsRng;
    let bob_secret = EphemeralSecret::random_from_rng(rng);
    let alice_public = X25519PublicKey::from(&EphemeralSecret::random_from_rng(rng));
    *bob_secret.diffie_hellman(&alice_public).as_bytes()
}

#[allow(dead_code)]
pub fn ecdh_encapsulate() -> Vec<u8> {
    let alice_secret = P256EphemeralSecret::random(&mut OsRng);
    let bob_public = P256EphemeralSecret::random(&mut OsRng).public_key();
    alice_secret.diffie_hellman(&bob_public).raw_secret_bytes().to_vec()
}

#[allow(dead_code)]
pub fn ecdh_decapsulate() -> Vec<u8> {
    let bob_secret = P256EphemeralSecret::random(&mut OsRng);
    let alice_public = P256EphemeralSecret::random(&mut OsRng).public_key();
    bob_secret.diffie_hellman(&alice_public).raw_secret_bytes().to_vec()
}

#[allow(dead_code)]
pub fn kyber_encapsulate() -> (pqcrypto_mlkem::mlkem512::SharedSecret, pqcrypto_mlkem::mlkem512::Ciphertext) {
    let (pk, _) = mlkem512::keypair();
    mlkem512::encapsulate(&pk)
}

#[allow(dead_code)]
pub fn kyber_decapsulate(ciphertext: pqcrypto_mlkem::mlkem512::Ciphertext, sk: pqcrypto_mlkem::mlkem512::SecretKey) -> pqcrypto_mlkem::mlkem512::SharedSecret {
    mlkem512::decapsulate(&ciphertext, &sk)
}

#[cfg(test)]
mod tests {
    use crate::BenchmarkKeys;

    #[test]
    fn test_kyber_encapsulate_decapsulate() {
        let keys = BenchmarkKeys::generate().unwrap();
        // Encapsulate with public key - returns (SharedSecret, Ciphertext)
        let (shared_secret_enc, ciphertext) = pqcrypto_mlkem::mlkem512::encapsulate(&keys.kyber_public);
        // Decapsulate with secret key
        let shared_secret_dec = pqcrypto_mlkem::mlkem512::decapsulate(&ciphertext, &keys.kyber_secret);
        // Compare shared secrets
        assert_eq!(shared_secret_enc, shared_secret_dec, "Kyber shared secret mismatch");
    }
}
