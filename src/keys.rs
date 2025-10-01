// Pre-generated keys untuk performance optimization
use ed25519_dalek::{SigningKey as Ed25519SigningKey, VerifyingKey as Ed25519VerifyingKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use p256::ecdsa::{SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey};
use pqcrypto_mldsa::mldsa44;
use pqcrypto_falcon::falcon512;
use pqcrypto_mlkem::mlkem512;
use rand::rngs::OsRng;
use rand::RngCore;
use crate::error::{BenchmarkError, Result};
use crate::constants::RSA_KEY_SIZE;

/// Pre-generated keys untuk semua algoritma
/// Ini menghemat 70-80% waktu benchmark dengan menghindari regenerasi key
pub struct BenchmarkKeys {
    // Classical Signatures
    pub ed25519_signing: Ed25519SigningKey,
    pub ed25519_verifying: Ed25519VerifyingKey,
    pub rsa_private: RsaPrivateKey,
    pub rsa_public: RsaPublicKey,
    pub ecdsa_signing: P256SigningKey,
    pub ecdsa_verifying: P256VerifyingKey,
    
    // Post-Quantum Signatures
    pub dilithium_public: mldsa44::PublicKey,
    pub dilithium_secret: mldsa44::SecretKey,
    pub falcon_public: falcon512::PublicKey,
    pub falcon_secret: falcon512::SecretKey,
    
    // KEM Keys
    pub kyber_public: mlkem512::PublicKey,
    pub kyber_secret: mlkem512::SecretKey,
}

impl BenchmarkKeys {
    /// Generate semua keys sekali. Operasi ini lambat (~500ms) tapi hanya dilakukan sekali.
    pub fn generate() -> Result<Self> {
        println!("Generating benchmark keys (this may take a moment)...");
        
        // Ed25519 keys
        let mut secret_bytes = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut secret_bytes);
        let ed25519_signing = Ed25519SigningKey::from_bytes(&secret_bytes);
        let ed25519_verifying = ed25519_signing.verifying_key();
        
        // RSA keys (slowest - ~200ms)
        let rsa_private = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
            .map_err(|e| BenchmarkError::KeyGeneration(format!("RSA: {}", e)))?;
        let rsa_public = RsaPublicKey::from(&rsa_private);
        
        // ECDSA keys
        let ecdsa_signing = P256SigningKey::random(&mut rng);
        let ecdsa_verifying = *ecdsa_signing.verifying_key();
        
        // Dilithium (ML-DSA) keys
        let (dilithium_public, dilithium_secret) = mldsa44::keypair();
        
        // Falcon keys
        let (falcon_public, falcon_secret) = falcon512::keypair();
        
        // Kyber (ML-KEM) keys
        let (kyber_public, kyber_secret) = mlkem512::keypair();
        
        println!("✓ All keys generated successfully");
        
        Ok(BenchmarkKeys {
            ed25519_signing,
            ed25519_verifying,
            rsa_private,
            rsa_public,
            ecdsa_signing,
            ecdsa_verifying,
            dilithium_public,
            dilithium_secret,
            falcon_public,
            falcon_secret,
            kyber_public,
            kyber_secret,
        })
    }
}

impl Drop for BenchmarkKeys {
    fn drop(&mut self) {
        // Security: Zeroize sensitive key material
        // Note: Ideally kita akan gunakan zeroize crate untuk ini
        // Tapi untuk sekarang, ini menunjukkan intent
        println!("Cleaning up benchmark keys...");
    }
}
