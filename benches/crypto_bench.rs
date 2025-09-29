use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use crypto_benchmark::signatures::*;
use crypto_benchmark::kem::*;
use pqcrypto_mlkem::mlkem512;

fn benchmark_signatures(c: &mut Criterion) {
    let mut group = c.benchmark_group("Signatures");

    group.bench_function("Ed25519 Sign", |b| b.iter(|| ed25519_sign(black_box(b"Hello, world!"))));
    group.bench_function("Ed25519 Verify", |b| b.iter(|| ed25519_verify(black_box(b"Hello, world!"))));

    group.bench_function("RSA Sign", |b| b.iter(|| rsa_sign(black_box(b"Hello, world!"))));
    group.bench_function("RSA Verify", |b| b.iter(|| rsa_verify(black_box(b"Hello, world!"))));

    group.bench_function("ECDSA Sign", |b| b.iter(|| ecdsa_sign(black_box(b"Hello, world!"))));
    group.bench_function("ECDSA Verify", |b| b.iter(|| ecdsa_verify(black_box(b"Hello, world!"))));

    group.bench_function("Dilithium Sign", |b| b.iter(|| dilithium_sign(black_box(b"Hello, world!"))));
    group.bench_function("Dilithium Verify", |b| b.iter(|| dilithium_verify(black_box(b"Hello, world!"))));

    group.finish();
}

fn benchmark_kem(c: &mut Criterion) {
    let mut group = c.benchmark_group("KEM");

    group.bench_function("RSA KEM Encapsulate", |b| b.iter(|| rsa_kem_encapsulate()));
    group.bench_function("RSA KEM Decapsulate", |b| b.iter(|| rsa_kem_decapsulate()));

    group.bench_function("DH Encapsulate", |b| b.iter(|| dh_encapsulate()));
    group.bench_function("DH Decapsulate", |b| b.iter(|| dh_decapsulate()));

    group.bench_function("ECDH Encapsulate", |b| b.iter(|| ecdh_encapsulate()));
    group.bench_function("ECDH Decapsulate", |b| b.iter(|| ecdh_decapsulate()));

    group.bench_function("Kyber Encapsulate", |b| b.iter(|| kyber_encapsulate()));

    group.bench_function("Kyber Decapsulate", |b| {
        let (_, sk) = mlkem512::keypair();
        let (_, ciphertext) = kyber_encapsulate();
        b.iter(|| kyber_decapsulate(ciphertext, sk))
    });

    group.finish();
}

criterion_group!(benches, benchmark_signatures, benchmark_kem);
criterion_main!(benches);
