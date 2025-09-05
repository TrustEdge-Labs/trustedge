<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# TrustEdge Performance Benchmarking

This document provides an overview of the comprehensive performance benchmarking infrastructure implemented for the TrustEdge project.

## Overview

The TrustEdge project now includes comprehensive performance benchmarking capabilities to measure and monitor the performance of cryptographic operations, network streaming, and general system performance.

## Benchmark Suites

### 1. Cryptographic Performance Benchmarks (`crypto_benchmarks.rs`)

Tests the performance of core cryptographic operations:

#### AES-256-GCM Encryption/Decryption
- **Encryption Throughput**: ~1.5 GiB/s for large data sizes (1MB)
- **Decryption Throughput**: ~1.5 GiB/s for large data sizes (1MB)
- Performance scales well with data size, showing optimal throughput for larger chunks

#### Ed25519 Digital Signatures
- **Signing Performance**: ~350 MiB/s for large data (1MB)
- **Verification Performance**: ~680 MiB/s for large data (1MB)
- Ed25519 shows excellent scalability with data size

#### P-256 ECDSA Signatures
- **Signing Performance**: ~1.4 GiB/s for large data (1MB)
- **Verification Performance**: ~1.3 GiB/s for large data (1MB)
- P-256 performs well for throughput but has higher base latency than Ed25519

#### Hash Algorithm Performance
- **SHA256**: ~1.8 GiB/s
- **SHA384**: ~700 MiB/s
- **SHA512**: ~700 MiB/s  
- **BLAKE3**: ~4.0 GiB/s (fastest)

#### PBKDF2 Key Derivation
- **1,000 iterations**: ~95 µs
- **10,000 iterations**: ~970 µs
- **100,000 iterations**: ~9.6 ms

#### Universal Backend System
- **Backend Selection**: ~124 ns (extremely fast dispatch)
- **SHA256 through Universal Backend**: ~1.7 GiB/s (minimal overhead)

### 2. Network Performance Benchmarks (`network_benchmarks.rs`)

Tests streaming and network-related performance:

#### Audio Chunk Serialization/Deserialization
- **Serialization**: ~1.8 Gelem/s for various chunk sizes
- **Deserialization**: ~1.8 Gelem/s with consistent performance
- Scales well from 512 samples to 16K samples

#### Concurrent Audio Processing
- **Single Chunk**: ~18 µs
- **32 Concurrent Chunks**: ~95 µs
- Shows good parallelization with managed concurrency

#### Binary Data Encoding
- **Raw Bytes**: ~6.8 GiB/s (baseline)
- **Bincode**: ~6.8 GiB/s (minimal overhead)
- **JSON**: ~200 MiB/s (much slower but human-readable)

#### Memory Allocation Patterns
- **Fresh Allocation**: 50-175 ns depending on size
- **Buffer Reuse**: 20-135 ns (more efficient for smaller sizes)

## Key Performance Insights

### 1. Cryptographic Operations
- **BLAKE3 is the fastest hash algorithm** for this workload
- **AES-256-GCM provides excellent throughput** for encryption/decryption
- **Ed25519 offers good signing performance** with faster verification than P-256
- **Universal backend dispatch adds minimal overhead** (~1-2% impact)

### 2. Network and Streaming
- **Audio chunk processing is very efficient** with sub-microsecond per-sample costs
- **Bincode serialization is optimal** for network transfer (minimal overhead vs raw bytes)
- **Concurrent processing scales well** up to hardware limits
- **Buffer reuse provides measurable benefits** for smaller allocations

### 3. System Performance
- **Format detection is extremely fast** (sub-nanosecond for simple patterns)
- **PBKDF2 timing scales linearly** with iteration count as expected
- **Memory allocation patterns** show expected trade-offs between allocation and reuse

## Running Benchmarks

### Manual Execution (Local Only)

Benchmarks are designed for **local development only** and do not run in CI/CD pipelines.

#### Full Statistical Benchmarks (~15 minutes)
```bash
cargo bench                    # Run all benchmarks with full accuracy
cargo bench --bench crypto    # Run crypto benchmarks only (~10 minutes)
cargo bench --bench network   # Run network benchmarks only (~5 minutes)
```

#### Quick Performance Check (~1 minute)
```bash
# Set fast mode and run benchmarks
BENCH_FAST=1 cargo bench                    # All fast benchmarks (~1 minute)
BENCH_FAST=1 cargo bench --bench crypto     # Fast crypto only (~45 seconds)
BENCH_FAST=1 cargo bench --bench network    # Fast network only (~15 seconds)

# Or use the convenience script
../scripts/fast-bench.sh                    # Same as BENCH_FAST=1 cargo bench
../scripts/fast-bench.sh crypto             # Fast crypto benchmarks
../scripts/fast-bench.sh network            # Fast network benchmarks

# From project root:
./scripts/fast-bench.sh                     # All fast benchmarks
```

### When to Run Benchmarks

**Recommended usage:**
- **Before major releases** - Run full benchmarks to establish baselines
- **During performance optimization** - Use fast mode for quick iteration
- **When investigating performance issues** - Full benchmarks for statistical accuracy
- **After algorithmic changes** - Verify performance impact

**NOT recommended:**
- ❌ In CI/CD pipelines (too slow, can be flaky)
- ❌ On every commit (unnecessary overhead)
- ❌ For functional testing (use regular tests instead)

### Benchmark Output

Benchmarks generate:
- **Performance metrics** in the terminal
- **HTML reports** in `target/criterion/` (when plotters available)
- **Baseline comparisons** for regression detection
- **Statistical analysis** with outlier detection

## Configuration Modes

### Full Accuracy Mode (Default)
- **Runtime**: ~15 minutes total
- **Samples**: 50-100 per test for statistical significance
- **Use case**: Performance analysis, optimization, release preparation
- **Command**: `cargo bench`

### Fast Mode
- **Runtime**: ~1 minute total  
- **Samples**: 15-20 per test for basic trending
- **Use case**: Quick performance checks during development
- **Command**: `BENCH_FAST=1 cargo bench` or `./fast-bench.sh`

## Integration with Development Workflow

### Local Development Only
Benchmarks are intentionally **not integrated with CI/CD** to avoid:
- ❌ Slow CI build times (15+ minutes)
- ❌ Flaky test results due to CI environment variations
- ❌ Resource contention with other CI jobs
- ❌ Unnecessary complexity in the build pipeline

### Manual Performance Monitoring
Instead, performance monitoring relies on:
- ✅ **Developer-driven benchmarking** before releases
- ✅ **Performance documentation** of major changes
- ✅ **Baseline tracking** through local benchmark runs
- ✅ **Performance-aware code review** practices

## Future Enhancements

Potential areas for expansion:
1. **Memory usage benchmarking** to track allocation patterns
2. **Network latency simulation** for realistic streaming scenarios  
3. **Hardware-specific optimizations** benchmarking
4. **Real-world workload simulation** beyond synthetic tests
5. **Benchmark result visualization** and trending

## Contributing

When adding new features:
1. **Add corresponding benchmarks** for performance-critical code
2. **Run benchmarks** before and after changes
3. **Document performance characteristics** of new algorithms
4. **Consider performance implications** in design decisions

## Dependencies

The benchmarking infrastructure uses:
- **Criterion.rs** for statistical benchmarking
- **Tokio** for async concurrency testing
- **Standard library** crypto implementations where possible
- **Serde** ecosystem for serialization benchmarks

This comprehensive benchmarking suite ensures that performance remains a first-class concern in the TrustEdge project development process.
