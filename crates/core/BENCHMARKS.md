<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->


# Sealedge Benchmarking Setup - Summary

## **Final Configuration: Local Development Only**

The benchmarking infrastructure is now configured for **local development workflows only** - no CI integration.

## **Two Benchmark Modes**

### 1. **Full Accuracy Mode** (Default)
```bash
cargo bench
```
- **Runtime**: ~15 minutes total
- **Purpose**: Performance analysis, optimization work, release preparation
- **Accuracy**: High statistical confidence (50-100 samples per test)

### 2. **Fast Mode** (Quick Checks)
```bash
BENCH_FAST=1 cargo bench
# OR
../scripts/fast-bench.sh       # From sealedge-core/
./scripts/fast-bench.sh        # From project root
```
- **Runtime**: ~1 minute total
- **Purpose**: Quick performance checks during development
- **Accuracy**: Basic trending (15-20 samples per test)

## **Available Commands**

```bash
# Full benchmarks (slow but accurate)
cargo bench                          # All benchmarks (~15 min)
cargo bench --bench crypto_benchmarks   # Crypto only (~10 min)
cargo bench --bench network_benchmarks  # Network only (~5 min)

# Fast benchmarks (quick checks)
BENCH_FAST=1 cargo bench               # All fast (~1 min)
../scripts/fast-bench.sh               # Same as above (from crates/core/)
./scripts/fast-bench.sh                # Same as above (from project root)
../scripts/fast-bench.sh crypto        # Fast crypto (~45s)
../scripts/fast-bench.sh network       # Fast network (~15s)
```

## **What's NOT Included**

- No CI/CD integration - benchmarks don't run on push/PR
- No automatic regression testing - manual performance monitoring
- No scheduled benchmark runs - purely on-demand
- No benchmark failure blocking - development workflow unaffected

## **Recommended Workflow**

1. **During Development**: Use `../scripts/fast-bench.sh` for quick performance checks
2. **Before Optimization**: Run `cargo bench` to establish baseline
3. **After Optimization**: Run `cargo bench` again to measure improvement
4. **Before Releases**: Run full benchmarks to document performance characteristics

## **What Gets Measured**

### Cryptographic Operations
- AES-256-GCM encryption/decryption (~1.5 GiB/s)
- Ed25519 signatures (~350 MiB/s signing, ~680 MiB/s verification)
- P-256 ECDSA signatures (~1.4 GiB/s)
- Hash algorithms (BLAKE3 fastest at ~4 GiB/s)
- PBKDF2 key derivation timing
- Universal backend dispatch overhead

### Network & Streaming
- Audio chunk serialization/deserialization
- Concurrent processing scalability
- Memory allocation patterns
- Binary encoding comparisons (raw vs bincode vs JSON)

## **Benefits of This Approach**

- Fast CI/CD - No benchmark overhead in automated builds
- Reliable Development - No flaky benchmark failures blocking PRs
- Flexible Performance Testing - Run when you need performance data
- Developer Control - Choose between fast checks and thorough analysis
- Resource Efficient - No wasted CI minutes on performance testing

## **Documentation**

- **Full details**: See `PERFORMANCE.md`
- **Benchmark results**: Terminal output + HTML reports in `target/criterion/`
- **Performance tracking**: Manual baseline comparison

This setup gives you powerful performance measurement tools when you need them, without slowing down your daily development workflow!
