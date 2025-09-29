use crate::measure_resources;
use ed25519_dalek::{SigningKey as Ed25519SigningKey, Signature as Ed25519Signature, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey, Pss};
use sha2::{Sha256, Digest};
use p256::ecdsa::{SigningKey as P256SigningKey};
use pqcrypto_mldsa::*;
use pqcrypto_falcon::*;
use rand::rngs::OsRng;
use rand::RngCore;
use sysinfo::System;

const MESSAGE: &[u8] = b"Hello, world!";

pub fn benchmark_signatures(sys: &mut System) {
    // Ed25519
    println!("Ed25519 Signature:");
    benchmark_ed25519(sys);

    // RSA
    println!("\nRSA Signature:");
    benchmark_rsa(sys);

    // ECDSA
    println!("\nECDSA Signature:");
    benchmark_ecdsa(sys);

    // Dilithium
    println!("\nDilithium Signature:");
    benchmark_dilithium(sys);

    // Falcon
    println!("\nFalcon Signature:");
    benchmark_falcon(sys);
}

fn benchmark_ed25519(sys: &mut System) {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = Ed25519SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    let (sign_time, sign_cpu, sign_mem) = measure_resources(sys, || {
        let _ = signing_key.sign(MESSAGE);
    });

    let signature: Ed25519Signature = signing_key.sign(MESSAGE);

    let (verify_time, verify_cpu, verify_mem) = measure_resources(sys, || {
        verifying_key.verify(MESSAGE, &signature).unwrap();
    });

    println!("  Key Size: {} bytes", 32);
    println!("  Signature Size: {} bytes", signature.to_bytes().len());
    println!("  Sign Time: {} μs, CPU: {}%, Mem: {} KB", sign_time, sign_cpu, sign_mem / 1024);
    println!("  Verify Time: {} μs, CPU: {}%, Mem: {} KB", verify_time, verify_cpu, verify_mem / 1024);
}

fn benchmark_rsa(sys: &mut System) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    let (sign_time, sign_cpu, sign_mem) = measure_resources(sys, || {
        let padding = Pss::new::<Sha256>();
        let mut hasher = Sha256::new();
        hasher.update(MESSAGE);
        let hashed = hasher.finalize();
        let _ = private_key.sign_with_rng(&mut OsRng, padding, &hashed).unwrap();
    });

    let padding = Pss::new::<Sha256>();
    let mut hasher = Sha256::new();
    hasher.update(MESSAGE);
    let hashed = hasher.finalize();
    let signature = private_key.sign_with_rng(&mut rng, padding, &hashed).unwrap();

    let (verify_time, verify_cpu, verify_mem) = measure_resources(sys, || {
        let padding = Pss::new::<Sha256>();
        let mut hasher = Sha256::new();
        hasher.update(MESSAGE);
        let hashed = hasher.finalize();
        public_key.verify(padding, &hashed, &signature).unwrap();
    });

    println!("  Key Size: {} bits", 2048);
    println!("  Signature Size: {} bytes", signature.len());
    println!("  Sign Time: {} μs, CPU: {}%, Mem: {} KB", sign_time, sign_cpu, sign_mem / 1024);
    println!("  Verify Time: {} μs, CPU: {}%, Mem: {} KB", verify_time, verify_cpu, verify_mem / 1024);
}

fn benchmark_ecdsa(sys: &mut System) {
    let mut rng = OsRng;
    let signing_key = P256SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let (sign_time, sign_cpu, sign_mem) = measure_resources(sys, || {
        let _: p256::ecdsa::Signature = signing_key.sign(MESSAGE);
    });

    let signature: p256::ecdsa::Signature = signing_key.sign(MESSAGE);

    let (verify_time, verify_cpu, verify_mem) = measure_resources(sys, || {
        verifying_key.verify(MESSAGE, &signature).unwrap();
    });

    println!("  Key Size: {} bytes", 32);
    println!("  Signature Size: {} bytes", signature.to_vec().len());
    println!("  Sign Time: {} μs, CPU: {}%, Mem: {} KB", sign_time, sign_cpu, sign_mem / 1024);
    println!("  Verify Time: {} μs, CPU: {}%, Mem: {} KB", verify_time, verify_cpu, verify_mem / 1024);
}

fn benchmark_dilithium(sys: &mut System) {
    let (pk, sk) = mldsa44::keypair();
    let message = MESSAGE;

    let (sign_time, sign_cpu, sign_mem) = measure_resources(sys, || {
        let _ = mldsa44::sign(message, &sk);
    });

    let signature = mldsa44::sign(message, &sk);

    let (verify_time, verify_cpu, verify_mem) = measure_resources(sys, || {
        mldsa44::open(&signature, &pk).unwrap();
    });

    println!("  Public Key Size: {} bytes", mldsa44::public_key_bytes());
    println!("  Secret Key Size: {} bytes", mldsa44::secret_key_bytes());
    println!("  Signature Size: {} bytes", mldsa44::signature_bytes());
    println!("  Sign Time: {} μs, CPU: {}%, Mem: {} KB", sign_time, sign_cpu, sign_mem / 1024);
    println!("  Verify Time: {} μs, CPU: {}%, Mem: {} KB", verify_time, verify_cpu, verify_mem / 1024);
}

fn benchmark_falcon(sys: &mut System) {
    let (pk, sk) = falcon512::keypair();
    let message = MESSAGE;

    let (sign_time, sign_cpu, sign_mem) = measure_resources(sys, || {
        let _ = falcon512::sign(message, &sk);
    });

    let signature = falcon512::sign(message, &sk);

    let (verify_time, verify_cpu, verify_mem) = measure_resources(sys, || {
        falcon512::open(&signature, &pk).unwrap();
    });

    println!("  Public Key Size: {} bytes", falcon512::public_key_bytes());
    println!("  Secret Key Size: {} bytes", falcon512::secret_key_bytes());
    println!("  Signature Size: {} bytes", falcon512::signature_bytes());
    println!("  Sign Time: {} μs, CPU: {}%, Mem: {} KB", sign_time, sign_cpu, sign_mem / 1024);
    println!("  Verify Time: {} μs, CPU: {}%, Mem: {} KB", verify_time, verify_cpu, verify_mem / 1024);
}

pub fn ed25519_sign(message: &[u8]) -> Ed25519Signature {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = Ed25519SigningKey::from_bytes(&secret_bytes);
    signing_key.sign(message)
}

pub fn ed25519_verify(message: &[u8]) {
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let signing_key = Ed25519SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(message);
    verifying_key.verify(message, &signature).unwrap();
}

pub fn rsa_sign(message: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let padding = Pss::new::<Sha256>();
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed = hasher.finalize();
    private_key.sign_with_rng(&mut rng, padding, &hashed).unwrap()
}

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

pub fn ecdsa_sign(message: &[u8]) -> p256::ecdsa::Signature {
    let mut rng = OsRng;
    let signing_key = P256SigningKey::random(&mut rng);
    signing_key.sign(message)
}

pub fn ecdsa_verify(message: &[u8]) {
    let mut rng = OsRng;
    let signing_key = P256SigningKey::random(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let signature: p256::ecdsa::Signature = signing_key.sign(message);
    verifying_key.verify(message, &signature).unwrap();
}

pub fn dilithium_sign(message: &[u8]) -> pqcrypto_mldsa::mldsa44::SignedMessage {
    let (_, sk) = mldsa44::keypair();
    mldsa44::sign(message, &sk)
}

pub fn dilithium_verify(message: &[u8]) {
    let (pk, sk) = mldsa44::keypair();
    let signature = mldsa44::sign(message, &sk);
    mldsa44::open(&signature, &pk).unwrap();
}

pub fn falcon_sign(message: &[u8]) -> pqcrypto_falcon::falcon512::SignedMessage {
    let (_, sk) = falcon512::keypair();
    falcon512::sign(message, &sk)
}

pub fn falcon_verify(message: &[u8]) {
    let (pk, sk) = falcon512::keypair();
    let signature = falcon512::sign(message, &sk);
    falcon512::open(&signature, &pk).unwrap();
}
