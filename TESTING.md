# TrustEdge Testing Guide

Comprehensive testing, validation, and verification procedures for TrustEdge.

## Table of Contents
- [Test Vectors](#test-vectors)
- [Integration Testing](#integration-testing)
- [Manual Verification](#manual-verification)
- [Performance Testing](#performance-testing)
- [Security Testing](#security-testing)

---

## Test Vectors

### Golden Test Vector

TrustEdge includes comprehensive deterministic test vectors for format validation:

```bash
# Run format compliance test with golden hash verification
cargo test vectors::tests::golden_trst_digest_is_stable

# Run integration tests (round-trip, tamper detection)
cargo test --test vectors

# Run all tests
cargo test
```

**Golden Test Vector:**
- **Input**: 32KB deterministic pseudo-random data
- **Chunk Size**: 4KB chunks  
- **Expected Hash**: `8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c`
- **Purpose**: Ensures format stability and enables external validation

### Test Vector Details

The golden test vector validates:
- **Deterministic encryption**: Same input + keys = same output
- **Format stability**: Binary format doesn't change between versions
- **Cross-platform compatibility**: Works identically across different systems
- **Cryptographic correctness**: All security properties maintained

---

## Integration Testing

### Automated Test Suite

The test suite validates:
- ✅ **Format compliance**: Deterministic envelope generation with known cryptographic material
- ✅ **Round-trip integrity**: Encrypt → envelope → decrypt cycle verification
- ✅ **Tamper detection**: Corrupted envelopes correctly rejected
- ✅ **CLI functionality**: End-to-end testing via command-line interface
- ✅ **Network protocol**: Client/server chunk transfer validation
- ✅ **Backend system**: Key management backend validation
- ✅ **Error handling**: Proper error reporting for invalid inputs

### Running Tests

```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test backends::keyring

# Run tests in release mode
cargo test --release
```

---

## Manual Verification

### Quick Smoke Test

```bash
# Quick smoke test
echo "test data" > input.txt
./target/release/trustedge-audio \
  --input input.txt --out output.txt --envelope test.trst \
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

./target/release/trustedge-audio \
  --decrypt --input test.trst --out decrypted.txt \
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

diff input.txt decrypted.txt  # Should be identical
```

### Comprehensive Validation

#### 1. Format Validation
```bash
# Test different file sizes
for size in 100 1000 10000 100000; do
  dd if=/dev/urandom of=test_${size}.bin bs=1 count=$size
  ./target/release/trustedge-audio \
    --input test_${size}.bin \
    --envelope test_${size}.trst \
    --key-hex $(openssl rand -hex 32)
  echo "Size $size: OK"
done
```

#### 2. Key Management Validation
```bash
# Test keyring backend
./target/release/trustedge-audio --set-passphrase "test_passphrase"
./target/release/trustedge-audio \
  --input test.txt \
  --envelope keyring_test.trst \
  --backend keyring \
  --salt-hex "1234567890abcdef1234567890abcdef" \
  --use-keyring

# Verify decryption works
./target/release/trustedge-audio \
  --decrypt \
  --input keyring_test.trst \
  --out keyring_decrypted.txt \
  --backend keyring \
  --salt-hex "1234567890abcdef1234567890abcdef" \
  --use-keyring
```

#### 3. Error Handling Validation
```bash
# Test invalid inputs
./target/release/trustedge-audio --decrypt --input nonexistent.trst
./target/release/trustedge-audio --salt-hex "invalid"
./target/release/trustedge-audio --backend nonexistent
```

---

## Performance Testing

### Throughput Benchmarking

```bash
# Create large test file
dd if=/dev/urandom of=large_test.bin bs=1M count=100

# Time encryption
time ./target/release/trustedge-audio \
  --input large_test.bin \
  --envelope large_test.trst \
  --key-hex $(openssl rand -hex 32)

# Time decryption
time ./target/release/trustedge-audio \
  --decrypt \
  --input large_test.trst \
  --out large_decrypted.bin \
  --key-hex $(cat last_key.hex)
```

### Memory Usage Testing

```bash
# Monitor memory usage during processing
/usr/bin/time -v ./target/release/trustedge-audio \
  --input large_test.bin \
  --envelope large_test.trst \
  --key-hex $(openssl rand -hex 32)
```

### Chunk Size Performance

```bash
# Test different chunk sizes
for chunk_size in 1024 4096 8192 16384 65536; do
  echo "Testing chunk size: $chunk_size"
  time ./target/release/trustedge-audio \
    --input test_1mb.bin \
    --envelope test_chunk_${chunk_size}.trst \
    --chunk $chunk_size \
    --key-hex $(openssl rand -hex 32)
done
```

---

## Security Testing

### Tamper Detection Tests

#### 1. Header Tampering
```bash
# Create valid envelope
./target/release/trustedge-audio \
  --input test.txt \
  --envelope original.trst \
  --key-hex $(openssl rand -hex 32)

# Corrupt header
dd if=/dev/urandom of=corrupted.trst bs=1 count=10 conv=notrunc

# Verify detection
./target/release/trustedge-audio \
  --decrypt \
  --input corrupted.trst \
  --out should_fail.txt \
  --key-hex $(cat last_key.hex)
# Should fail with "bad magic" or similar
```

#### 2. Record Tampering
```bash
# Corrupt middle of file
dd if=/dev/urandom of=original.trst bs=1 seek=100 count=10 conv=notrunc

# Verify detection  
./target/release/trustedge-audio \
  --decrypt \
  --input original.trst \
  --out should_fail.txt \
  --key-hex $(cat last_key.hex)
# Should fail with "AES-GCM decrypt/verify failed"
```

### Key Validation Tests

#### 1. Wrong Key Detection
```bash
# Encrypt with one key
./target/release/trustedge-audio \
  --input test.txt \
  --envelope test.trst \
  --key-hex $(openssl rand -hex 32)

# Try to decrypt with different key
./target/release/trustedge-audio \
  --decrypt \
  --input test.trst \
  --out should_fail.txt \
  --key-hex $(openssl rand -hex 32)
# Should fail with "AES-GCM decrypt/verify failed"
```

#### 2. Salt Validation Tests
```bash
# Test with wrong salt
./target/release/trustedge-audio \
  --decrypt \
  --input keyring_test.trst \
  --out should_fail.txt \
  --backend keyring \
  --salt-hex "deadbeefdeadbeefdeadbeefdeadbeef" \
  --use-keyring
# Should fail with "AES-GCM decrypt/verify failed"
```

---

## Network Testing

### Client-Server Testing

#### 1. Basic Network Flow
```bash
# Terminal 1: Start server
./target/release/trustedge-server \
  --port 8080 \
  --decrypt \
  --key-hex $(openssl rand -hex 32) \
  --output-dir ./server_output

# Terminal 2: Run client
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input test_audio.wav \
  --key-hex $(cat shared_key.hex)
```

#### 2. Network Error Handling
```bash
# Test connection failures
./target/release/trustedge-client \
  --server 127.0.0.1:9999 \
  --input test.wav \
  --key-hex $(openssl rand -hex 32)
# Should fail with connection error

# Test invalid data
# (Send corrupted chunks and verify server handling)
```

---

## Envelope Integrity Testing

### Nonce Validation
All envelope integrity invariants are strictly enforced:
- Each record's nonce prefix (first 4 bytes) must match the stream header's nonce prefix
- The nonce counter (last 8 bytes) must equal the record's sequence number
- The manifest's `seq` field must match the record's `seq` field

### Validation Failure Testing
If any validation fails (e.g., signature, nonce prefix, nonce counter, manifest sequence, hash), the record is rejected and an error is reported.

```bash
# These tests are built into the unit test suite
cargo test envelope_integrity
cargo test nonce_validation
cargo test signature_verification
```

---

## Continuous Integration Testing

### GitHub Actions Tests
The CI pipeline runs:
- ✅ `cargo test` - All unit and integration tests
- ✅ `cargo clippy` - Code quality and linting
- ✅ `cargo fmt --check` - Code formatting validation
- ✅ Cross-platform testing (Linux, macOS, Windows)
- ✅ Multiple Rust versions (stable, beta, nightly)

### Test Coverage
```bash
# Generate test coverage report (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html
# Open tarpaulin-report.html to view coverage
```

---

## Debugging Failed Tests

### Common Issues and Solutions

#### 1. Keyring Access Failures
If tests fail due to keyring access (common in CI):
```bash
# The tests now validate salt before keyring access
# This should work in headless environments
```

#### 2. Timing-Related Failures
```bash
# Add debug output to see actual vs expected values
RUST_LOG=debug cargo test failing_test_name -- --nocapture
```

#### 3. Platform-Specific Issues
```bash
# Test on specific platform
cargo test --target x86_64-unknown-linux-gnu
cargo test --target x86_64-pc-windows-msvc
cargo test --target x86_64-apple-darwin
```

---

For more information about the security model and threat analysis, see [THREAT_MODEL.md](./THREAT_MODEL.md).

For protocol details and technical specifications, see [PROTOCOL.md](./PROTOCOL.md).
