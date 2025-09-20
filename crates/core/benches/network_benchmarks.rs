//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
// GitHub: https://github.com/TrustEdge-Labs/trustedge
//

//! Network streaming performance benchmarks
//!
//! This module benchmarks network streaming throughput, including:
//! - Audio chunk serialization/deserialization
//! - Concurrent audio processing
//! - Memory allocation patterns

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::{Duration, Instant};
use trustedge_core::AudioChunk;

// Test data sizes for network streaming
const CHUNK_SIZES: &[usize] = &[512, 1024, 2048, 4096, 8192, 16384];

/// Generate audio test data
fn generate_audio_data(samples: usize) -> Vec<f32> {
    (0..samples)
        .map(|i| {
            // Generate a simple sine wave
            let freq = 440.0; // A4 note
            let sample_rate = 44100.0;
            let time = i as f32 / sample_rate;
            (2.0 * std::f32::consts::PI * freq * time).sin() * 0.1
        })
        .collect()
}

/// Benchmark audio chunk serialization performance
fn bench_audio_chunk_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_chunk_serialization");

    for &samples in CHUNK_SIZES {
        let audio_data = generate_audio_data(samples);
        let chunk = AudioChunk {
            data: audio_data,
            timestamp: Instant::now(),
            sample_rate: 44100,
            channels: 1,
            sequence: 0,
        };

        group.throughput(Throughput::Elements(samples as u64));

        group.bench_with_input(
            BenchmarkId::new("serialize", samples),
            &chunk,
            |b, chunk| {
                b.iter(|| {
                    let _bytes = chunk.to_bytes();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark audio chunk deserialization performance
fn bench_audio_chunk_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_chunk_deserialization");

    for &samples in CHUNK_SIZES {
        let audio_data = generate_audio_data(samples);
        let chunk = AudioChunk {
            data: audio_data,
            timestamp: Instant::now(),
            sample_rate: 44100,
            channels: 1,
            sequence: 0,
        };
        let bytes = chunk.to_bytes();

        group.throughput(Throughput::Elements(samples as u64));

        group.bench_with_input(
            BenchmarkId::new("deserialize", samples),
            &bytes,
            |b, bytes| {
                b.iter(|| {
                    let _chunk = AudioChunk::from_bytes(black_box(bytes), 44100, 1, 0)
                        .expect("Failed to deserialize audio chunk");
                });
            },
        );
    }

    group.finish();
}

/// Benchmark concurrent audio chunk processing
fn bench_concurrent_audio_processing(c: &mut Criterion) {
    use std::sync::Arc;
    use tokio::sync::Semaphore;
    use tokio::task::JoinSet;

    let mut group = c.benchmark_group("concurrent_audio_processing");

    let rt = tokio::runtime::Runtime::new().unwrap();

    let chunk_counts = [1, 4, 8, 16, 32];

    for &chunk_count in &chunk_counts {
        group.bench_with_input(
            BenchmarkId::new("process_chunks", chunk_count),
            &chunk_count,
            |b, &count| {
                b.to_async(&rt).iter(|| async {
                    let semaphore = Arc::new(Semaphore::new(8)); // Limit concurrency
                    let mut join_set = JoinSet::new();

                    for i in 0..count {
                        let sem = semaphore.clone();

                        join_set.spawn(async move {
                            let _permit = sem.acquire().await.unwrap();

                            // Simulate audio processing
                            let audio_data = generate_audio_data(1024);
                            let chunk = AudioChunk {
                                data: audio_data,
                                timestamp: Instant::now(),
                                sample_rate: 44100,
                                channels: 1,
                                sequence: i as u64,
                            };

                            // Serialize and deserialize to simulate network transfer
                            let bytes = chunk.to_bytes();
                            let _reconstructed = AudioChunk::from_bytes(&bytes, 44100, 1, i as u64)
                                .expect("Failed to process chunk");
                        });
                    }

                    // Wait for all tasks to complete
                    while join_set.join_next().await.is_some() {}
                });
            },
        );
    }

    group.finish();
}

/// Benchmark binary data encoding performance
fn bench_binary_data_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_data_encoding");

    for &samples in CHUNK_SIZES {
        let audio_data = generate_audio_data(samples);

        group.throughput(Throughput::Bytes((samples * 4) as u64)); // f32 = 4 bytes

        // Benchmark raw bytes (baseline)
        group.bench_with_input(
            BenchmarkId::new("raw_bytes", samples),
            &audio_data,
            |b, data| {
                b.iter(|| {
                    let mut bytes = Vec::with_capacity(data.len() * 4);
                    for &sample in data {
                        bytes.extend_from_slice(&sample.to_le_bytes());
                    }
                    black_box(bytes);
                });
            },
        );

        // Benchmark bincode serialization
        group.bench_with_input(
            BenchmarkId::new("bincode_encode", samples),
            &audio_data,
            |b, data| {
                b.iter(|| {
                    let _encoded =
                        bincode::serialize(black_box(data)).expect("Failed to encode with bincode");
                });
            },
        );

        // Benchmark JSON serialization (for comparison)
        group.bench_with_input(
            BenchmarkId::new("json_encode", samples),
            &audio_data,
            |b, data| {
                b.iter(|| {
                    let _encoded =
                        serde_json::to_vec(black_box(data)).expect("Failed to encode with JSON");
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation patterns for streaming
fn bench_streaming_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_memory_allocation");

    let buffer_sizes = [512, 1024, 2048, 4096, 8192];

    for &buffer_size in &buffer_sizes {
        group.bench_with_input(
            BenchmarkId::new("allocate_buffer", buffer_size),
            &buffer_size,
            |b, &size| {
                b.iter(|| {
                    // Simulate repeated buffer allocation/deallocation
                    let _buffer: Vec<f32> = vec![0.0; black_box(size)];
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("reuse_buffer", buffer_size),
            &buffer_size,
            |b, &size| {
                let mut buffer = Vec::with_capacity(size);
                b.iter(|| {
                    buffer.clear();
                    buffer.resize(black_box(size), 0.0);
                    black_box(&buffer);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark network packet simulation
fn bench_network_packet_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("network_packet_simulation");

    // Simulate packet creation and parsing overhead
    group.bench_function("packet_creation", |b| {
        b.iter(|| {
            // Simulate creating a network packet structure
            let header = vec![0u8; 20]; // Simulated header
            let payload = vec![0u8; black_box(1024)]; // 1KB payload
            let _packet = [header, payload].concat();
        });
    });

    group.bench_function("connection_state_update", |b| {
        use std::collections::HashMap;

        let mut connection_state = HashMap::new();
        b.iter(|| {
            // Simulate connection state updates
            let connection_id = black_box(12345u64);
            let state = black_box("ESTABLISHED");
            connection_state.insert(connection_id, state);
        });
    });

    // Benchmark throughput calculations
    group.bench_function("throughput_calculation", |b| {
        let mut bytes_transferred = 0u64;
        let mut last_time = Instant::now();

        b.iter(|| {
            bytes_transferred += black_box(1024);
            let now = Instant::now();
            let elapsed = now.duration_since(last_time);

            if elapsed.as_millis() > 100 {
                let _throughput_mbps =
                    (bytes_transferred as f64 * 8.0) / (elapsed.as_secs_f64() * 1_000_000.0);
                bytes_transferred = 0;
                last_time = now;
            }
        });
    });

    group.finish();
}

// Configure criterion for local development
fn configure_criterion() -> Criterion {
    // Check if user wants fast benchmarks locally
    let is_fast = std::env::var("BENCH_FAST").is_ok();

    if is_fast {
        // Fast local configuration: ~15 seconds total
        Criterion::default()
            .measurement_time(Duration::from_secs(2)) // 2 seconds per test
            .sample_size(15) // 15 samples for basic accuracy
            .warm_up_time(Duration::from_secs(1)) // 1 second warm-up
    } else {
        // Thorough local configuration: ~5 minutes total
        Criterion::default()
            .measurement_time(Duration::from_secs(8)) // 8 seconds per test
            .sample_size(50) // 50 samples for good accuracy
            .warm_up_time(Duration::from_secs(2)) // 2 seconds warm-up
    }
}

criterion_group!(
    name = network_benches;
    config = configure_criterion();
    targets =
        bench_audio_chunk_serialization,
        bench_audio_chunk_deserialization,
        bench_concurrent_audio_processing,
        bench_binary_data_encoding,
        bench_streaming_memory_allocation,
        bench_network_packet_simulation
);

criterion_main!(network_benches);
