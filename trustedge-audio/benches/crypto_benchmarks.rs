//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
// GitHub: https://github.com/TrustEdge-Labs/trustedge
//

//! Performance benchmarks for TrustEdge cryptographic operations
//!
//! This benchmark suite measures the performance of core crypto operations
//! including AES-GCM encryption, Ed25519 signatures, and universal backend overhead.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;
use trustedge_audio::{
    BackendPreferences, CryptoOperation, HashAlgorithm, UniversalBackendRegistry,
};

// Test data sizes for throughput benchmarks
const SIZES: &[usize] = &[1024, 4096, 16384, 65536, 262144, 1048576]; // 1KB to 1MB

/// Generate test data of specified size
fn generate_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Benchmark AES-256-GCM encryption throughput
fn bench_aes_gcm_encryption(c: &mut Criterion) {
    use aes_gcm::{
        aead::{Aead, AeadCore, OsRng},
        Aes256Gcm, KeyInit,
    };
    use rand::RngCore;

    let mut group = c.benchmark_group("aes_gcm_encryption");

    // Generate a consistent key for benchmarking
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    for &size in SIZES {
        let data = generate_test_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("encrypt", size), &data, |b, data| {
            b.iter(|| {
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
                let _ciphertext = cipher
                    .encrypt(&nonce, black_box(data.as_slice()))
                    .expect("Encryption failed");
            });
        });
    }

    group.finish();
}

/// Benchmark AES-256-GCM decryption throughput  
fn bench_aes_gcm_decryption(c: &mut Criterion) {
    use aes_gcm::{
        aead::{Aead, AeadCore, OsRng},
        Aes256Gcm, KeyInit,
    };
    use rand::RngCore;

    let mut group = c.benchmark_group("aes_gcm_decryption");

    // Generate a consistent key for benchmarking
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    for &size in SIZES {
        let data = generate_test_data(size);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, data.as_slice())
            .expect("Encryption failed");

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("decrypt", size),
            &ciphertext,
            |b, ciphertext| {
                b.iter(|| {
                    let _plaintext = cipher
                        .decrypt(&nonce, black_box(ciphertext.as_slice()))
                        .expect("Decryption failed");
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Ed25519 signature generation
fn bench_ed25519_signing(c: &mut Criterion) {
    use ed25519_dalek::{Signature, Signer, SigningKey};
    use rand::rngs::OsRng;

    let mut group = c.benchmark_group("ed25519_signing");

    // Generate a keypair for benchmarking
    let mut csprng = OsRng {};
    let keypair = SigningKey::generate(&mut csprng);

    for &size in SIZES {
        let data = generate_test_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("sign", size), &data, |b, data| {
            b.iter(|| {
                let _signature: Signature = keypair.sign(black_box(data));
            });
        });
    }

    group.finish();
}

/// Benchmark Ed25519 verification performance
fn bench_ed25519_verification(c: &mut Criterion) {
    use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
    use rand::rngs::OsRng;

    let mut group = c.benchmark_group("ed25519_verification");

    let mut csprng = OsRng {};
    let keypair = SigningKey::generate(&mut csprng);
    let public_key: VerifyingKey = keypair.verifying_key();

    for &size in SIZES {
        let data = generate_test_data(size);
        let signature: Signature = keypair.sign(&data);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("verify", size), &size, |b, _| {
            b.iter(|| {
                public_key
                    .verify(black_box(&data), black_box(&signature))
                    .expect("verification failure");
            });
        });
    }

    group.finish();
}

/// Benchmark P256 ECDSA signing performance
fn bench_p256_signing(c: &mut Criterion) {
    use p256::ecdsa::signature::Signer;
    use p256::{
        ecdsa::{Signature, SigningKey},
        SecretKey,
    };
    use rand::rngs::OsRng;

    let mut group = c.benchmark_group("p256_ecdsa_signing");

    let secret_key = SecretKey::random(&mut OsRng);
    let signing_key = SigningKey::from(secret_key);

    for &size in SIZES {
        let data = generate_test_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("sign", size), &size, |b, _| {
            b.iter(|| {
                let _signature: Signature = signing_key.sign(black_box(&data));
            });
        });
    }

    group.finish();
}

/// Benchmark P256 ECDSA verification performance
fn bench_p256_verification(c: &mut Criterion) {
    use p256::ecdsa::signature::{Signer, Verifier};
    use p256::{
        ecdsa::{Signature, SigningKey, VerifyingKey},
        SecretKey,
    };
    use rand::rngs::OsRng;

    let mut group = c.benchmark_group("p256_ecdsa_verification");

    let secret_key = SecretKey::random(&mut OsRng);
    let signing_key = SigningKey::from(secret_key);
    let verifying_key = VerifyingKey::from(&signing_key);

    for &size in SIZES {
        let data = generate_test_data(size);
        let signature: Signature = signing_key.sign(&data);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("verify", size), &size, |b, _| {
            b.iter(|| {
                verifying_key
                    .verify(black_box(&data), black_box(&signature))
                    .expect("verification failure");
            });
        });
    }

    group.finish();
}

/// Benchmark Universal Backend operation dispatch overhead
fn bench_universal_backend_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("universal_backend_dispatch");

    // Set up universal backend registry
    let registry = UniversalBackendRegistry::with_defaults()
        .expect("Failed to create universal backend registry");
    let backend_names = registry.list_backend_names();

    if backend_names.is_empty() {
        eprintln!("Warning: No universal backends available for benchmarking");
        return;
    }

    let backend = registry
        .get_backend(backend_names[0])
        .expect("Failed to get backend");

    // Benchmark hash operations (most backends support this)
    for &size in SIZES {
        let data = generate_test_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("hash_sha256", size), &size, |b, _| {
            b.iter(|| {
                let hash_operation = CryptoOperation::Hash {
                    algorithm: HashAlgorithm::Sha256,
                    data: black_box(data.clone()),
                };

                if backend.supports_operation(&hash_operation) {
                    let _result = backend
                        .perform_operation("bench_key", hash_operation)
                        .expect("Hash operation failed");
                }
            });
        });
    }

    // Benchmark backend selection overhead
    group.bench_function("backend_selection", |b| {
        b.iter(|| {
            let preferences = BackendPreferences::new();
            let hash_operation = CryptoOperation::Hash {
                algorithm: HashAlgorithm::Sha256,
                data: vec![0u8; 1024],
            };

            let _backend = registry
                .find_preferred_backend(black_box(&hash_operation), black_box(&preferences));
        });
    });

    group.finish();
}

/// Benchmark hash algorithm performance
fn bench_hash_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_algorithms");

    for &size in SIZES {
        let data = generate_test_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        // SHA-256
        group.bench_with_input(BenchmarkId::new("sha256", size), &size, |b, _| {
            use sha2::{Digest, Sha256};
            b.iter(|| {
                let mut hasher = Sha256::new();
                hasher.update(black_box(&data));
                let _hash = hasher.finalize();
            });
        });

        // SHA-384
        group.bench_with_input(BenchmarkId::new("sha384", size), &size, |b, _| {
            use sha2::{Digest, Sha384};
            b.iter(|| {
                let mut hasher = Sha384::new();
                hasher.update(black_box(&data));
                let _hash = hasher.finalize();
            });
        });

        // SHA-512
        group.bench_with_input(BenchmarkId::new("sha512", size), &size, |b, _| {
            use sha2::{Digest, Sha512};
            b.iter(|| {
                let mut hasher = Sha512::new();
                hasher.update(black_box(&data));
                let _hash = hasher.finalize();
            });
        });

        // BLAKE3
        group.bench_with_input(BenchmarkId::new("blake3", size), &size, |b, _| {
            b.iter(|| {
                let _hash = blake3::hash(black_box(&data));
            });
        });
    }

    group.finish();
}

/// Benchmark PBKDF2 key derivation
fn bench_pbkdf2_key_derivation(c: &mut Criterion) {
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;

    let mut group = c.benchmark_group("pbkdf2_key_derivation");

    let password = b"test_password_for_benchmarking";
    let salt = b"benchmark_salt16"; // 16 bytes
    let iterations = [1000, 10000, 100000];

    for &iter_count in &iterations {
        group.bench_with_input(
            BenchmarkId::new("pbkdf2_sha256", iter_count),
            &iter_count,
            |b, &iterations| {
                b.iter(|| {
                    let mut key = [0u8; 32];
                    pbkdf2_hmac::<Sha256>(
                        black_box(password),
                        black_box(salt),
                        black_box(iterations),
                        &mut key,
                    );
                });
            },
        );
    }

    group.finish();
}

/// Benchmark file format detection speed
fn bench_format_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_detection");

    // Create test data that mimics different file types
    let test_files = vec![
        ("trustedge", b"TRST\x01\x01".to_vec()), // TrustEdge header
        ("mp3", b"\xFF\xFB\x90\x00".to_vec()),   // MP3 header
        ("pdf", b"%PDF-1.4\n".to_vec()),         // PDF header
        ("json", b"{\n  \"test\": \"data\"\n}".to_vec()), // JSON data
        ("binary", (0..1024).map(|i| (i % 256) as u8).collect()), // Binary data
    ];

    for (file_type, data) in test_files {
        group.bench_with_input(BenchmarkId::new("detect", file_type), &data, |b, data| {
            b.iter(|| {
                // Simple format detection based on header bytes
                let detected = if data.starts_with(b"TRST") {
                    "trustedge"
                } else if data.starts_with(b"\xFF\xFB") {
                    "mp3"
                } else if data.starts_with(b"%PDF") {
                    "pdf"
                } else if data.starts_with(b"{") {
                    "json"
                } else {
                    "binary"
                };
                black_box(detected);
            });
        });
    }

    group.finish();
}

// Configure criterion for local development
fn configure_criterion() -> Criterion {
    // Check if user wants fast benchmarks locally
    let is_fast = std::env::var("BENCH_FAST").is_ok();

    if is_fast {
        // Fast local configuration: ~45 seconds total
        Criterion::default()
            .measurement_time(Duration::from_secs(2)) // 2 seconds per test
            .sample_size(20) // 20 samples for basic accuracy
            .warm_up_time(Duration::from_secs(1)) // 1 second warm-up
    } else {
        // Thorough local configuration: ~10 minutes total
        Criterion::default()
            .measurement_time(Duration::from_secs(10)) // 10 seconds per test
            .sample_size(100) // 100 samples for statistical accuracy
            .warm_up_time(Duration::from_secs(3)) // 3 seconds warm-up
    }
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets =
        bench_aes_gcm_encryption,
        bench_aes_gcm_decryption,
        bench_ed25519_signing,
        bench_ed25519_verification,
        bench_p256_signing,
        bench_p256_verification,
        bench_universal_backend_dispatch,
        bench_hash_algorithms,
        bench_pbkdf2_key_derivation,
        bench_format_detection
);

criterion_main!(benches);
