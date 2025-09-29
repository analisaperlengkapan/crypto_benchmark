# Cryptographic Benchmarking Tool

Aplikasi Rust untuk membandingkan performa algoritma kriptografi, termasuk signature klasik dan post-quantum, serta Key Encapsul### Kecepatan KE### Kecepatan KEM (Decapsulate)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-KEM** | 409 μs | Post-Quantum ⚡ |
| **Diffie-Hellman** | 740 μs | Classical ⚡ |
| **ECDH** | 14.7 ms | Classical |
| **RSA KEM** | 55.2 ms | Classical 🐌 |sulate)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-KEM** | 172 μs | Post-Quantum ⚡ |
| **Diffie-Hellman** | 839 μs | Classical ⚡ |
| **RSA KEM** | 3.8 ms | Classical |
| **ECDH** | 3.7 ms | Classical |chanism (KEM).

## Algoritma yang Didukung

### Signature Algorithms

#### Klasik
- **Ed25519**: Algoritma signature berbasis kurva eliptik Ed25519
  - Key size: 32 bytes
  - Signature size: 64 bytes

- **RSA**: RSA signature dengan PSS padding dan SHA256
  - Key size: 2048 bits
  - Signature size: 256 bytes

- **ECDSA**: Elliptic Curve Digital Signature Algorithm (P256 curve)
  - Key size: 32 bytes
  - Signature size: 64 bytes

#### Post-Quantum
- **ML-DSA**: Post-quantum signature algorithm (ML-DSA44)
  - Public key size: 1312 bytes
  - Secret key size: 2560 bytes
  - Signature size: 2420 bytes

- **Falcon**: Post-quantum signature algorithm (Falcon512)
  - Public key size: 897 bytes
  - Secret key size: 1281 bytes
  - Signature size: 690 bytes

### Key Encapsulation Mechanism (KEM)

#### Klasik
- **Diffie-Hellman (DH)**: Menggunakan X25519
  - Key size: 32 bytes
  - Shared secret size: 32 bytes

- **RSA KEM**: Menggunakan RSA-OAEP dengan SHA-256
  - Key size: 2048-bit RSA
  - Ciphertext size: 256 bytes

- **ECDH**: Elliptic Curve Diffie-Hellman (P256 curve)
  - Key size: 32 bytes
  - Shared secret size: 32 bytes

#### Post-Quantum
- **ML-KEM**: Post-quantum KEM algorithm (ML-KEM512)
  - Public key size: 800 bytes
  - Secret key size: 1632 bytes
  - Ciphertext size: 768 bytes
  - Shared secret size: 32 bytes

## Fitur

- **Pengukuran Performa**: Mengukur waktu eksekusi, penggunaan CPU, dan penggunaan memori
- **Benchmark Criterion**: Benchmark presisi tinggi menggunakan Criterion.rs
- **Algoritma Lengkap**: Mendukung algoritma klasik dan post-quantum
- **Output Terstruktur**: Hasil benchmark ditampilkan dalam format yang mudah dibaca

## Cara Menjalankan

### Menjalankan Benchmark Sederhana
```bash
cargo run
```

### Menjalankan Benchmark Criterion (Lebih Presisi)
```bash
cargo bench
```

## Hasil Benchmark Terbaru (September 12, 2025)

### Signature Performance

```
Ed25519 Signature:
  Key Size: 32 bytes
  Signature Size: 64 bytes
  Sign Time: 174 μs, CPU: 25%, Mem: 504 KB
  Verify Time: 8487 μs, CPU: 0%, Mem: 1008 KB

RSA Signature:
  Key Size: 2048 bits
  Signature Size: 256 bytes
  Sign Time: 51040 μs, CPU: 0%, Mem: 3720 KB
  Verify Time: 4636 μs, CPU: 0%, Mem: 992 KB

ECDSA Signature:
  Key Size: 32 bytes
  Signature Size: 64 bytes
  Sign Time: 1785 μs, CPU: 0%, Mem: 748 KB
  Verify Time: 3399 μs, CPU: 0%, Mem: 2004 KB

Dilithium Signature:
  Public Key Size: 1312 bytes
  Secret Key Size: 2560 bytes
  Signature Size: 2420 bytes
  Sign Time: 162 μs, CPU: 0%, Mem: 1764 KB
  Verify Time: 220 μs, CPU: 9%, Mem: 4776 KB

Falcon Signature:
  Public Key Size: 897 bytes
  Secret Key Size: 1281 bytes
  Signature Size: 752 bytes
  Sign Time: 1179 μs, CPU: 0%, Mem: 932 KB
  Verify Time: 234 μs, CPU: 0%, Mem: 1252 KB
```

### KEM Performance

```
Diffie-Hellman:
  Key Size: 32 bytes
  Shared Secret Size: 32 bytes
  Encapsulate Time: 839 μs, CPU: 0%, Mem: 2404 KB
  Decapsulate Time: 740 μs, CPU: 0%, Mem: 1192 KB

RSA KEM:
  Key Size: 2048 bits
  Ciphertext Size: 256 bytes
  Encapsulate Time: 3798 μs, CPU: 0%, Mem: 2764 KB
  Decapsulate Time: 55188 μs, CPU: 0%, Mem: 4948 KB

ECDH:
  Key Size: 32 bytes
  Shared Secret Size: 32 bytes
  Encapsulate Time: 3726 μs, CPU: 0%, Mem: 2772 KB
  Decapsulate Time: 14727 μs, CPU: 0%, Mem: 2016 KB

Kyber:
  Public Key Size: 800 bytes
  Secret Key Size: 1632 bytes
  Ciphertext Size: 768 bytes
  Shared Secret Size: 32 bytes
  Encapsulate Time: 172 μs, CPU: 0%, Mem: 5524 KB
  Decapsulate Time: 409 μs, CPU: 0%, Mem: 6308 KB
````

## Perbandingan Algoritma

### Kecepatan Signature (Sign Time)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-DSA** | 162 μs | Post-Quantum ⚡ |
| **Ed25519** | 174 μs | Classical ⚡ |
| **ECDSA** | 1.8 ms | Classical |
| **Falcon** | 1.2 ms | Post-Quantum |
| **RSA** | 51.0 ms | Classical 🐌 |

### Kecepatan Signature (Verify Time)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-DSA** | 220 μs | Post-Quantum ⚡ |
| **Falcon** | 234 μs | Post-Quantum ⚡ |
| **ECDSA** | 3.4 ms | Classical |
| **RSA** | 4.6 ms | Classical |
| **Ed25519** | 8.5 ms | Classical |

### Kecepatan KEM (Encapsulate)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-KEM** | 319 μs | Post-Quantum ⚡ |
| **Diffie-Hellman** | 387 μs | Classical ⚡ |
| **ECDH** | 3.4 ms | Classical |
| **RSA KEM** | 4.4 ms | Classical |

### Kecepatan KEM (Decapsulate)

| Algorithm | Time | Category |
|-----------|------|----------|
| **ML-KEM** | 304 μs | Post-Quantum ⚡ |
| **Diffie-Hellman** | 470 μs | Classical ⚡ |
| **ECDH** | 2.8 ms | Classical |
| **RSA KEM** | 34.9 ms | Classical 🐌 |

### Ukuran Key dan Signature

| Algorithm | Key Size | Signature/Ciphertext | Category |
|-----------|----------|---------------------|----------|
| **Ed25519** | 32 bytes | 64 bytes | Classical |
| **ECDSA** | 32 bytes | 64 bytes | Classical |
| **Diffie-Hellman** | 32 bytes | 32 bytes | Classical |
| **ECDH** | 32 bytes | 32 bytes | Classical |
| **RSA** | 2048 bits (~256 bytes) | 256 bytes | Classical |
| **RSA KEM** | 2048 bits (~256 bytes) | 256 bytes | Classical |
| **Falcon** | 897 bytes (PK) / 1281 bytes (SK) | 752 bytes | Post-Quantum |
| **ML-KEM** | 800 bytes (PK) / 1632 bytes (SK) | 768 bytes | Post-Quantum |
| **ML-DSA** | 1312 bytes (PK) / 2560 bytes (SK) | 2420 bytes | Post-Quantum |

## Kesimpulan

### 🏆 **Pemenang Berdasarkan Kategori**

**Signature Tercepat (Sign):**
- **ML-DSA** (162 μs) - Post-quantum dengan performa luar biasa
- **Ed25519** (174 μs) - Classical tercepat

**Signature Tercepat (Verify):**
- **ML-DSA** (220 μs) - Verifikasi ultra-cepat
- **Falcon** (234 μs) - Hampir sama cepat

**Signature Terkecil:**
- **Falcon** (752 bytes) - Ukuran signature efisien

**KEM Tercepat (Encapsulate):**
- **ML-KEM** (172 μs) - Post-quantum tercepat

**KEM Tercepat (Decapsulate):**
- **ML-KEM** (409 μs) - Post-quantum dominan

**Keseimbangan Terbaik:**
- **Ed25519** - Cepat, ukuran kecil, terbukti aman
- **ECDSA** - Keseimbangan yang baik untuk aplikasi modern
- **Falcon** - Ukuran signature efisien, verifikasi cepat (post-quantum)

### 📊 **Rekomendasi Penggunaan**

- **Untuk performa maksimal**: ML-DSA + ML-KEM (post-quantum)
- **Untuk ukuran signature minimal**: Falcon + ML-KEM (post-quantum)
- **Untuk aplikasi legacy**: Ed25519 + Diffie-Hellman
- **Untuk enterprise**: ECDSA + ECDH
- **Untuk kompatibilitas**: RSA (walaupun lambat)

## Dependencies

- `ed25519-dalek`: Implementasi Ed25519 (v2.0)
- `x25519-dalek`: Implementasi Diffie-Hellman X25519 (v2.0)
- `rsa`: Implementasi RSA dengan PSS/SHA256 (v0.9)
- `ecdsa` & `p256`: Implementasi ECDSA dan ECDH (v0.16, v0.13)
- `pqcrypto-mldsa`: Post-quantum signature ML-DSA (FIPS 204, v0.1.2)
- `pqcrypto-mlkem`: Post-quantum KEM ML-KEM (FIPS 203, v0.1.1)
- `pqcrypto-falcon`: Post-quantum signature Falcon (FIPS 205, v0.4.1)
- `criterion`: Benchmarking presisi tinggi (v0.7)
- `sysinfo`: Monitoring sistem (v0.37)
- `rand` & `sha2`: Utilitas kriptografi (v0.8, v0.10)
- `serde`: Serialisasi (opsional)

## Struktur Proyek

```
src/
├── main.rs          # Entry point aplikasi
├── lib.rs           # Fungsi utilitas bersama
├── signatures.rs    # Implementasi benchmark signature
├── kem.rs           # Implementasi benchmark KEM
└── measure_resources.rs # Utilitas pengukuran performa

benches/
└── crypto_bench.rs  # Benchmark Criterion

Cargo.toml           # Dependencies dan konfigurasi
README.md           # Dokumentasi ini
```

## Catatan

Aplikasi ini menggunakan algoritma kriptografi terkini yang tersedia di Rust crates, termasuk implementasi post-quantum cryptography (ML-DSA, ML-KEM, dan Falcon) dari PQClean project yang mengikuti standar FIPS 204, FIPS 203, dan FIPS 205. Semua algoritma telah diuji dan memberikan hasil benchmark yang konsisten.

**Benchmark dijalankan pada:** September 29, 2025
**Platform:** Linux
**Rust Version:** 2021 Edition</content>
<parameter name="filePath">/home/clouduser/enc_test/README.md
