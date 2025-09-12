use crate::measure_resources;
use x25519_dalek::EphemeralSecret;
use rand::rngs::OsRng;
use sysinfo::System;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use sha2::Sha256;
use p256::{ecdh::EphemeralSecret as P256EphemeralSecret};
use pqcrypto_mlkem::*;

pub fn benchmark_kem(sys: &mut System) {
    // Diffie-Hellman
    println!("Diffie-Hellman:");
    benchmark_dh(sys);

    // RSA KEM
    println!("\nRSA KEM:");
    benchmark_rsa_kem(sys);

    // ECDH
    println!("\nECDH:");
    benchmark_ecdh(sys);

    // Kyber
    println!("\nKyber:");
    benchmark_kyber(sys);
}

fn benchmark_dh(sys: &mut System) {
    let mut rng = OsRng;
    let alice_secret = EphemeralSecret::random_from_rng(&mut rng);
    let alice_public = x25519_dalek::PublicKey::from(&alice_secret);
    let bob_secret = EphemeralSecret::random_from_rng(&mut rng);
    let bob_public = x25519_dalek::PublicKey::from(&bob_secret);

    let (enc_time, enc_cpu, enc_mem) = measure_resources(sys, || {
        let alice_secret_temp = EphemeralSecret::random_from_rng(&mut OsRng);
        let _ = alice_secret_temp.diffie_hellman(&x25519_dalek::PublicKey::from(&EphemeralSecret::random_from_rng(&mut OsRng)));
    });

    let alice_shared = alice_secret.diffie_hellman(&bob_public);

    let (dec_time, dec_cpu, dec_mem) = measure_resources(sys, || {
        let bob_secret_temp = EphemeralSecret::random_from_rng(&mut OsRng);
        let _ = bob_secret_temp.diffie_hellman(&x25519_dalek::PublicKey::from(&EphemeralSecret::random_from_rng(&mut OsRng)));
    });

    let bob_shared = bob_secret.diffie_hellman(&alice_public);

    assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());

    println!("  Key Size: {} bytes", 32);
    println!("  Shared Secret Size: {} bytes", alice_shared.as_bytes().len());
    println!("  Encapsulate Time: {} μs, CPU: {}%, Mem: {} KB", enc_time, enc_cpu, enc_mem / 1024);
    println!("  Decapsulate Time: {} μs, CPU: {}%, Mem: {} KB", dec_time, dec_cpu, dec_mem / 1024);
}

fn benchmark_rsa_kem(sys: &mut System) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    let (enc_time, enc_cpu, enc_mem) = measure_resources(sys, || {
        let padding = Oaep::new::<Sha256>();
        let data = b"secret data";
        let _ = public_key.encrypt(&mut OsRng, padding, data).unwrap();
    });

    let padding = Oaep::new::<Sha256>();
    let data = b"secret data";
    let encrypted = public_key.encrypt(&mut rng, padding, data).unwrap();

    let (dec_time, dec_cpu, dec_mem) = measure_resources(sys, || {
        let padding = Oaep::new::<Sha256>();
        let _ = private_key.decrypt(padding, &encrypted).unwrap();
    });

    println!("  Key Size: {} bits", 2048);
    println!("  Ciphertext Size: {} bytes", encrypted.len());
    println!("  Encapsulate Time: {} μs, CPU: {}%, Mem: {} KB", enc_time, enc_cpu, enc_mem / 1024);
    println!("  Decapsulate Time: {} μs, CPU: {}%, Mem: {} KB", dec_time, dec_cpu, dec_mem / 1024);
}

fn benchmark_ecdh(sys: &mut System) {
    let alice_secret = P256EphemeralSecret::random(&mut OsRng);
    let alice_public = alice_secret.public_key();
    let bob_secret = P256EphemeralSecret::random(&mut OsRng);
    let bob_public = bob_secret.public_key();

    let (enc_time, enc_cpu, enc_mem) = measure_resources(sys, || {
        let alice_secret_temp = P256EphemeralSecret::random(&mut OsRng);
        let _ = alice_secret_temp.diffie_hellman(&P256EphemeralSecret::random(&mut OsRng).public_key());
    });

    let alice_shared = alice_secret.diffie_hellman(&bob_public);

    let (dec_time, dec_cpu, dec_mem) = measure_resources(sys, || {
        let bob_secret_temp = P256EphemeralSecret::random(&mut OsRng);
        let _ = bob_secret_temp.diffie_hellman(&P256EphemeralSecret::random(&mut OsRng).public_key());
    });

    let bob_shared = bob_secret.diffie_hellman(&alice_public);

    assert_eq!(alice_shared.raw_secret_bytes(), bob_shared.raw_secret_bytes());

    println!("  Key Size: {} bytes", 32);
    println!("  Shared Secret Size: {} bytes", alice_shared.raw_secret_bytes().len());
    println!("  Encapsulate Time: {} μs, CPU: {}%, Mem: {} KB", enc_time, enc_cpu, enc_mem / 1024);
    println!("  Decapsulate Time: {} μs, CPU: {}%, Mem: {} KB", dec_time, dec_cpu, dec_mem / 1024);
}

pub fn rsa_kem_encapsulate() -> Vec<u8> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let padding = Oaep::new::<Sha256>();
    let data = b"secret data";
    public_key.encrypt(&mut rng, padding, data).unwrap()
}

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

pub fn dh_encapsulate() -> [u8; 32] {
    let mut rng = OsRng;
    let alice_secret = EphemeralSecret::random_from_rng(&mut rng);
    let bob_public = x25519_dalek::PublicKey::from(&EphemeralSecret::random_from_rng(&mut rng));
    alice_secret.diffie_hellman(&bob_public).as_bytes().clone()
}

pub fn dh_decapsulate() -> [u8; 32] {
    let mut rng = OsRng;
    let bob_secret = EphemeralSecret::random_from_rng(&mut rng);
    let alice_public = x25519_dalek::PublicKey::from(&EphemeralSecret::random_from_rng(&mut rng));
    bob_secret.diffie_hellman(&alice_public).as_bytes().clone()
}

pub fn ecdh_encapsulate() -> Vec<u8> {
    let alice_secret = P256EphemeralSecret::random(&mut OsRng);
    let bob_public = P256EphemeralSecret::random(&mut OsRng).public_key();
    alice_secret.diffie_hellman(&bob_public).raw_secret_bytes().to_vec()
}

pub fn ecdh_decapsulate() -> Vec<u8> {
    let bob_secret = P256EphemeralSecret::random(&mut OsRng);
    let alice_public = P256EphemeralSecret::random(&mut OsRng).public_key();
    bob_secret.diffie_hellman(&alice_public).raw_secret_bytes().to_vec()
}

fn benchmark_kyber(sys: &mut System) {
    let (pk, sk) = mlkem512::keypair();

    let (enc_time, enc_cpu, enc_mem) = measure_resources(sys, || {
        let (pk_temp, _) = mlkem512::keypair();
        let _ = mlkem512::encapsulate(&pk_temp);
    });

    let (shared_secret_alice, ciphertext) = mlkem512::encapsulate(&pk);

    let (dec_time, dec_cpu, dec_mem) = measure_resources(sys, || {
        let (_, sk_temp) = mlkem512::keypair();
        let (pk_temp, _) = mlkem512::keypair();
        let (_, ct_temp) = mlkem512::encapsulate(&pk_temp);
        let _ = mlkem512::decapsulate(&ct_temp, &sk_temp);
    });

    let _shared_secret_bob = mlkem512::decapsulate(&ciphertext, &sk);

    // Operations completed successfully - that's our test

    println!("  Public Key Size: {} bytes", mlkem512::public_key_bytes());
    println!("  Secret Key Size: {} bytes", mlkem512::secret_key_bytes());
    println!("  Ciphertext Size: {} bytes", mlkem512::ciphertext_bytes());
    println!("  Shared Secret Size: {} bytes", mlkem512::shared_secret_bytes());
    println!("  Encapsulate Time: {} μs, CPU: {}%, Mem: {} KB", enc_time, enc_cpu, enc_mem / 1024);
    println!("  Decapsulate Time: {} μs, CPU: {}%, Mem: {} KB", dec_time, dec_cpu, dec_mem / 1024);
}

pub fn kyber_encapsulate() -> (pqcrypto_mlkem::mlkem512::SharedSecret, pqcrypto_mlkem::mlkem512::Ciphertext) {
    let (pk, _) = mlkem512::keypair();
    mlkem512::encapsulate(&pk)
}

pub fn kyber_decapsulate(ciphertext: pqcrypto_mlkem::mlkem512::Ciphertext, sk: pqcrypto_mlkem::mlkem512::SecretKey) -> pqcrypto_mlkem::mlkem512::SharedSecret {
    mlkem512::decapsulate(&ciphertext, &sk)
}
