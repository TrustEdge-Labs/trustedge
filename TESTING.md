<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->
# TrustEdge Testing Guide

Comprehensive testing, validation, and verification procedures for TrustEdge with **109 total tests** covering all components.

## Table of Contents
- [Test Architecture](#test-architecture)
- [Test Categories](#test-categories)
- [YubiKey Testing](#yubikey-testing)
- [Transport Testing](#transport-testing)
- [Test Vectors](#test-vectors)
- [Integration Testing](#integration-testing)
- [Manual Verification](#manual-verification)
- [Performance Testing](#performance-testing)
- [Security Testing](#security-testing)
- [Audio System Testing](#audio-system-testing)

---

## Test Architecture

TrustEdge employs a comprehensive 3-tier testing strategy:

### Test Statistics
- **109 Total Tests** covering all production features
- **86 Core Tests** (trustedge-core: envelope encryption, backends, transport, YubiKey)
- **23 Receipt Tests** (trustedge-receipts: digital receipts, security, attack resistance)
- **100% Feature Coverage** (all major components tested)

### Quick Test Commands
```bash
# All tests (recommended before commit)
./ci-check.sh                    # Runs format, lint, build, and all tests

# By crate
cargo test -p trustedge-core --lib        # Core cryptography tests (86)
cargo test -p trustedge-receipts          # Receipt system tests (23)

# Hardware features
cargo test --features yubikey    # Include YubiKey hardware tests

# With features
cargo test --features yubikey    # Include YubiKey hardware tests
```

[↑ Back to top](#table-of-contents)

---

## Test Categories

### 1. Core Cryptography Tests (86 Tests)
**Envelope System** (`trustedge-core`):
- AES-256-GCM encryption/decryption with real cryptography
- PBKDF2 key derivation with 100,000 iterations
- Memory-safe key handling with zeroization
- Envelope seal/unseal roundtrip testing
- Large payload chunking and reassembly

**Universal Backend System**:
- Software HSM operations with persistent key storage
- Universal registry management and capability discovery
- Keyring backend integration with OS keyring
- YubiKey hardware backend with real PKCS#11 operations

**Transport Layer**:
- QUIC/TCP transport configuration and validation
- NetworkChunk serialization compatibility
- Concurrent connection handling
- Security configuration validation

### 2. Digital Receipt System Tests (23 Tests)
**Receipt Creation & Assignment** (`trustedge-receipts`):
```bash
cargo test -p trustedge-receipts
```
- **Cryptographic Security**: Real encryption/decryption with production algorithms
- **Ownership Transfer**: Multi-party receipt assignment chains (Alice → Bob → Charlie → Dave → Eve)
- **Amount Preservation**: Cryptographic protection of receipt amounts through assignment chains
- **Attack Resistance**: Comprehensive security testing against various attack scenarios

**Security Test Categories**:
- **Cryptographic Key Isolation**: Ensures only intended recipients can decrypt receipts
- **Signature Forgery Resistance**: Prevents impersonation using Ed25519 signatures
- **Replay Attack Prevention**: Each receipt has unique cryptographic fingerprint
- **Amount Tampering Resistance**: Receipt amounts are cryptographically bound and protected
- **Chain Integrity Validation**: Broken or out-of-order chains are properly rejected
```
- Certificate generation and verification
- Mutual authentication workflows
- Session management and lifecycle

#### Network Integration (7 Tests)
```bash
cargo test --test network_integration
```
- Authenticated file transfer
- Connection error handling
- Data integrity verification
- Multiple file type support

#### Roundtrip Integration (15 Tests)
```bash
cargo test --test roundtrip_integration
```
- End-to-end encryption/decryption workflows
- Format detection accuracy
- Byte-perfect data restoration
- Multiple chunk size validation

#### Software HSM Integration (9 Tests)
```bash
cargo test --test software_hsm_integration
```
- Cross-session key persistence
- Large-scale key management
- Corruption recovery mechanisms
- CLI integration workflows

#### Universal Backend Integration (6 Tests)
```bash
cargo test --test universal_backend_integration
```
- Capability-based backend selection
- Multi-operation workflows
- Performance characteristics
- Registry management

#### Domain Separation Security (7 Tests)
```bash
cargo test --test domain_separation_test
```
- Cross-context attack prevention
- Signature reuse protection
- Manifest tampering detection
- Cryptographic domain isolation

[↑ Back to top](#table-of-contents)

---

## YubiKey Testing

### Hardware Testing Requirements
```bash
# YubiKey with PIV applet
sudo apt install opensc-pkcs11      # PKCS#11 module
ykman piv keys generate 9a /tmp/pubkey.pem  # Generate test keys

# Run YubiKey-specific tests
cargo test --features yubikey --test yubikey_integration
```

### Phase Testing Architecture

#### Phase 1: X.509 Certificate Validation
```bash
cargo test test_phase1_certificate_validation
```
- Certificate parameter validation
- x509-cert crate integration testing
- Standards compliance verification

#### Phase 2: Hardware-Signed Certificates  
```bash
cargo test test_phase2_certificate_generation
```
- PIV slot enumeration (9a, 9c, 9d, 9e)
- Hardware key generation workflows
- Certificate signing with hardware keys

#### Phase 3: QUIC Transport Integration
```bash
cargo test test_phase3_quic_integration
```
- Hardware certificate export for QUIC
- Certificate-transport compatibility
- End-to-end secure communication

### PIV Slot Testing
- **Slot 9a**: PIV Authentication (client auth certificates)
- **Slot 9c**: PIV Digital Signature (code signing, email protection)
- **Slot 9d**: PIV Key Management (encryption/decryption)
- **Slot 9e**: PIV Card Authentication (device certificates)

[↑ Back to top](#table-of-contents)

---

## Transport Testing

### QUIC Transport Testing
```bash
cargo test quic::tests  # QUIC-specific unit tests
```
- Transport configuration validation
- Certificate requirements for QUIC
- Connection setup and teardown
- Stream management testing

### TCP Transport Testing  
```bash
cargo test tcp::tests   # TCP-specific unit tests
```
- Length-delimited framing validation
- Connection state management
- Large data chunk handling
- Resource cleanup verification

### Protocol Selection Testing
- **QUIC Preferred**: High throughput, multi-stream scenarios
- **TCP Fallback**: Legacy compatibility, simple point-to-point
- **Configuration Testing**: Security-focused settings, extreme limits

[↑ Back to top](#table-of-contents)

---

## Test Vectors

### Golden Test Vector

TrustEdge includes comprehensive deterministic test vectors for format validation:

```bash
# Run format compliance test with golden hash verification
cargo test vectors::tests::golden_trst_digest_is_stable

# Run integration tests (round-trip, tamper detection)
cargo test --test vectors

# All tests with output
cargo test -- --nocapture
```

**Updated Golden Test Vector:**
- **Input**: 32KB deterministic pseudo-random data
- **Chunk Size**: 4KB chunks  
- **Expected Hash**: `162efe3e02b010bd871bb8c69befe673aeb5828ab650131a1664d326609a8bb0`
- **Purpose**: Ensures format stability and enables external validation

### Test Vector Details

The golden test vector validates:
- **Deterministic encryption**: Same input + keys = same output
- **Format stability**: Binary format doesn't change between versions
- **Cross-platform compatibility**: Works identically across different systems
- **Cryptographic correctness**: All security properties maintained

[↑ Back to top](#table-of-contents)

---

## Integration Testing

### Automated Test Suite

The test suite validates:
- ✔ **Format compliance**: Deterministic envelope generation with known cryptographic material
- ✔ **Round-trip integrity**: Comprehensive encrypt → envelope → decrypt cycle verification
- ✔ **Real data validation**: Full workflow testing with actual file content (NEW!)
- ✔ **Multiple data types**: Text files, JSON, binary data, empty files
- ✔ **Variable chunk sizes**: Testing with 1KB, 4KB, 8KB chunk configurations
- ✔ **MIME type detection**: Format-aware processing validation
- ✔ **File size testing**: Small (1KB), medium (100KB), large (1MB) file handling
- ✔ **Tamper detection**: Corrupted envelopes correctly rejected
- ✔ **CLI functionality**: End-to-end testing via command-line interface
- ✔ **Network protocol**: Client/server mutual authentication and chunk transfer
- ✔ **Backend system**: Key management backend validation
- ✔ **Error handling**: Proper error reporting for invalid inputs

### Test Categories

**1. Unit Tests (53 tests)**
- Audio configuration and chunk handling
- Keyring backend functionality  
- Universal Backend system
- Software HSM backend (33 tests): Key generation, signing/verification, error handling, persistence, registry integration
- Golden test vector validation

**2. Software HSM Integration Tests (9 tests)**
- Cross-session key persistence and metadata integrity
- Universal Backend registry integration and capability testing
- File-based signing workflows and document verification
- CLI tool integration (software-hsm-demo lifecycle testing)
- Error recovery and resilience (corruption, permissions, partial failures)
- Performance and scale testing (large-scale key management)

**3. Authentication Integration Tests (3 tests)**
- Certificate generation and verification
- Session management
- Mutual authentication flow

**4. Roundtrip Integration Tests (15 tests)**
- Small, medium, and large file roundtrips
- Text and JSON format validation
- Binary data integrity
- Empty file handling
- Metadata inspection
- Multiple chunk size validation
- Format-specific tests (PDF, MP3, unknown formats)
- Comprehensive MIME type detection (39 file formats)
- Byte-perfect restoration validation

**5. Network Integration Tests (7 tests)**
- Client-server data transfer validation
- Multiple file type network transfer
- Data integrity across network
- Large file chunked transfer
- Authentication workflow testing
- Connection error handling
- Empty file network transfer

**6. Universal Backend Integration Tests (6 tests)**
- End-to-end crypto workflows using Universal Backend
- Capability-based backend selection
- Registry management and backend discovery
- Multi-operation workflow validation
- Performance characteristics testing
- Error handling and edge cases

**Total: 93 tests** with comprehensive workflow validation

### Running Tests

```bash
# Run all tests (93 total)
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific test suites
cargo test --test software_hsm_integration       # Software HSM integration tests (9)
cargo test --test roundtrip_integration          # Roundtrip tests (15)
cargo test --test auth_integration               # Authentication tests (3)
cargo test --test network_integration            # Network tests (7)
cargo test --test universal_backend_integration  # Universal Backend tests (6)
cargo test --lib                                 # Unit tests only (53)

# Software HSM specific tests
cargo test software_hsm --lib                    # Software HSM unit tests only (33)
cargo test software_hsm                          # All Software HSM tests (42 total)

# Run specific test modules
cargo test backends::keyring
cargo test vectors::tests::golden_trst_digest_is_stable

# Run tests in release mode
cargo test --release

# Test specific functionality
cargo test roundtrip                       # All roundtrip tests
cargo test authentication                  # All auth tests
```

### Roundtrip Integration Tests

The comprehensive roundtrip test suite ([`tests/roundtrip_integration.rs`](trustedge-core/tests/roundtrip_integration.rs)) provides full workflow validation with **15 tests**:

**Test Coverage:**
```bash
# Individual test examples
cargo test test_small_file_roundtrip       # 1KB file validation
cargo test test_medium_file_roundtrip      # 100KB file validation  
cargo test test_text_file_roundtrip        # UTF-8 text with emoji
cargo test test_json_file_roundtrip        # JSON structure preservation
cargo test test_pdf_file_roundtrip         # PDF format testing
cargo test test_mp3_file_roundtrip         # Audio format testing
cargo test test_unknown_format_roundtrip   # Unknown format handling
cargo test test_binary_file_roundtrip      # Binary data patterns
cargo test test_empty_file_roundtrip       # Edge case: zero bytes
cargo test test_inspect_encrypted_file     # Metadata validation
cargo test test_multiple_chunk_sizes       # 1KB, 4KB, 8KB chunks
cargo test test_format_detection_accuracy  # MIME type validation
cargo test test_byte_perfect_restoration   # Integrity verification
cargo test test_comprehensive_chunk_sizes  # Extended chunk testing
```

### Network Integration Tests

The network integration test suite ([`tests/network_integration.rs`](trustedge-core/tests/network_integration.rs)) validates client-server workflows with **7 tests**:

**Test Coverage:**
```bash
# Network test examples  
cargo test test_basic_file_transfer        # Basic client-server transfer
cargo test test_multiple_file_types        # Various file formats over network
cargo test test_data_integrity             # End-to-end data integrity
cargo test test_large_file_transfer        # Chunked large file transfer
cargo test test_authenticated_transfer     # Authentication workflow
cargo test test_connection_error_handling  # Error handling and timeouts
cargo test test_empty_file_transfer        # Empty file edge cases
```

**What Each Test Validates:**
- **Data Integrity**: Byte-for-byte comparison of original vs decrypted
- **Format Preservation**: MIME type detection and metadata handling
- **CLI Interface**: Real binary execution with proper argument handling
- **Error Handling**: Meaningful error messages on failure
- **Network Protocol**: TCP communication and chunked transfer
- **Authentication**: Certificate-based mutual authentication
- **Performance**: Tests complete efficiently with proper resource cleanup

**Sample Test Output:**
```
✔ Small file (1KB) roundtrip test passed!
✔ Medium file (100KB) roundtrip test passed!
✔ Text file roundtrip test passed!
✔ JSON file roundtrip test passed!
✔ PDF file roundtrip test passed!
✔ MP3 file roundtrip test passed!
✔ Unknown format roundtrip test passed!
✔ Empty file roundtrip test passed!
✔ Binary file roundtrip test passed!
✔ Inspect encrypted file test passed!
✔ Format detection accuracy test passed!
✔ Byte-perfect restoration test passed!
✔ Chunk size 1024 test passed!
✔ Chunk size 4096 test passed!
✔ Chunk size 8192 test passed!

Network Integration Tests:
✔ Basic file transfer test passed!
✔ Multiple file types test passed!
✔ Data integrity test passed!
✔ Large file transfer test passed!
✔ Authentication test passed!
✔ Connection error handling test passed!
✔ Empty file transfer test passed!

Universal Backend Integration Tests:
✔ Universal Backend encrypt/decrypt workflow validated
✔ Universal Backend capability-based selection validated
✔ Universal Backend multiple operations workflow validated
✔ Universal Backend error handling validated
✔ Universal Backend performance test completed
✔ Universal Backend registry management validated
```

### Universal Backend Integration Tests

The Universal Backend integration test suite ([`tests/universal_backend_integration.rs`](trustedge-core/tests/universal_backend_integration.rs)) validates the capability-based backend system with **6 tests**:

**Test Coverage:**
```bash
# Universal Backend integration test examples
cargo test test_universal_backend_encrypt_decrypt_workflow
cargo test test_universal_backend_capability_based_selection
cargo test test_universal_backend_multiple_operations_workflow
cargo test test_universal_backend_error_handling
cargo test test_universal_backend_performance_characteristics
cargo test test_universal_backend_registry_management
```

**What Each Test Validates:**
- **End-to-End Workflows**: Complete crypto operations through Universal Backend
- **Capability Discovery**: Automatic backend selection based on operation requirements
- **Registry Management**: Backend registration, discovery, and preference-based routing
- **Multi-Operation Workflows**: Sequential operations with deterministic results
- **Performance Validation**: Acceptable response times for crypto operations
- **Error Handling**: Graceful handling of unsupported operations and edge cases

**Key Features Tested:**
- **Backend Selection**: Automatic choice of appropriate backend for each operation
- **Operation Dispatch**: Enum-based operation routing with type safety
- **Deterministic Results**: Consistent key derivation across multiple calls
- **Resource Management**: Proper cleanup and resource handling
- **Edge Cases**: Zero-byte salts, empty contexts, invalid parameters

[↑ Back to top](#table-of-contents)

---

## Manual Verification

### Quick Smoke Test

```bash
# Quick smoke test
echo "test data" > input.txt
./target/release/trustedge-core \
  --input input.txt --out output.txt --envelope test.trst \
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

./target/release/trustedge-core \
  --decrypt --input test.trst --out decrypted.txt \
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

diff input.txt decrypted.txt  # Should be identical
```

### Format-Aware Testing

**Test Different File Types:**
```bash
# Create test files of different types
echo '{"test": "data"}' > test.json
echo "%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n>>\nendobj" > test.pdf
dd if=/dev/urandom bs=1024 count=1 of=test.bin 2>/dev/null

# Test encryption with format detection
./target/release/trustedge-core --input test.json --envelope test_json.trst --key-out json.key --verbose
./target/release/trustedge-core --input test.pdf --envelope test_pdf.trst --key-out pdf.key --verbose
./target/release/trustedge-core --input test.bin --envelope test_bin.trst --key-out bin.key --verbose

# Test inspection
./target/release/trustedge-core --input test_json.trst --inspect --verbose
./target/release/trustedge-core --input test_pdf.trst --inspect --verbose
./target/release/trustedge-core --input test_bin.trst --inspect --verbose

# Test decryption with format awareness
./target/release/trustedge-core --decrypt --input test_json.trst --out restored.json --key-hex $(cat json.key) --verbose
./target/release/trustedge-core --decrypt --input test_pdf.trst --out restored.pdf --key-hex $(cat pdf.key) --verbose
./target/release/trustedge-core --decrypt --input test_bin.trst --out restored.bin --key-hex $(cat bin.key) --verbose

# Verify format preservation
diff test.json restored.json
diff test.pdf restored.pdf
diff test.bin restored.bin
file restored.json  # Should show JSON data
file restored.pdf   # Should show PDF document
```

**Expected Output Verification:**
```bash
# JSON file inspection should show:
# MIME Type: application/json
# Output Behavior: Original file format preserved

# PDF file inspection should show:
# MIME Type: application/pdf
# Output Behavior: Original file format preserved

# Binary file inspection should show:
# MIME Type: application/octet-stream
# Output Behavior: Original file format preserved

# Decryption should show format-aware messages:
# ● Input Type: File
#   MIME Type: application/json
# ✔ Output: Original file format preserved
```

### Comprehensive Validation

#### 1. Format Validation
```bash
# Test different file sizes
for size in 100 1000 10000 100000; do
  dd if=/dev/urandom of=test_${size}.bin bs=1 count=$size
  ./target/release/trustedge-core \
    --input test_${size}.bin \
    --envelope test_${size}.trst \
    --key-hex $(openssl rand -hex 32)
  echo "Size $size: OK"
done
```

#### 2. Key Management Validation
```bash
# Test keyring backend
./target/release/trustedge-core --set-passphrase "test_passphrase"
./target/release/trustedge-core \
  --input test.txt \
  --envelope keyring_test.trst \
  --backend keyring \
  --salt-hex "1234567890abcdef1234567890abcdef" \
  --use-keyring

# Verify decryption works
./target/release/trustedge-core \
  --decrypt \
  --input keyring_test.trst \
  --out keyring_decrypted.txt \
  --backend keyring \
  --salt-hex "1234567890abcdef1234567890abcdef" \
  --use-keyring
```

#### 3. Error Handling Validation

**📖 For detailed error testing procedures and expected error messages, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).**

**Validation Test Categories:**
- File system errors (missing files, permissions)
- Configuration errors (invalid backends, salt formats)  
- Cryptographic errors (wrong keys, corrupted data)
- Network errors (connection failures, timeouts)
- Authentication errors (certificate issues, session timeouts)

**Quick Validation Tests:**
```bash
# Test error reporting for common issues
./target/release/trustedge-core --decrypt --input nonexistent.trst    # File not found
./target/release/trustedge-core --salt-hex "invalid"                  # Invalid salt
./target/release/trustedge-core --backend nonexistent                 # Invalid backend
```

[↑ Back to top](#table-of-contents)

---

## Performance Testing

### Throughput Benchmarking

```bash
# Create large test file
dd if=/dev/urandom of=large_test.bin bs=1M count=100

# Time encryption
time ./target/release/trustedge-core \
  --input large_test.bin \
  --envelope large_test.trst \
  --key-hex $(openssl rand -hex 32)

# Time decryption
time ./target/release/trustedge-core \
  --decrypt \
  --input large_test.trst \
  --out large_decrypted.bin \
  --key-hex $(cat last_key.hex)
```

### Memory Usage Testing

```bash
# Monitor memory usage during processing
/usr/bin/time -v ./target/release/trustedge-core \
  --input large_test.bin \
  --envelope large_test.trst \
  --key-hex $(openssl rand -hex 32)
```

### Chunk Size Performance

```bash
# Test different chunk sizes
for chunk_size in 1024 4096 8192 16384 65536; do
  echo "Testing chunk size: $chunk_size"
  time ./target/release/trustedge-core \
    --input test_1mb.bin \
    --envelope test_chunk_${chunk_size}.trst \
    --chunk $chunk_size \
    --key-hex $(openssl rand -hex 32)
done
```

[↑ Back to top](#table-of-contents)

---

## Security Testing

TrustEdge includes **comprehensive security testing** covering cryptographic attacks and security scenarios:

### Receipt System Security Tests (23 Tests)

**Production-Ready Security Testing:**
```bash
# Run all security tests
cargo test -p trustedge-receipts

# Run specific security test categories
cargo test -p trustedge-receipts test_cryptographic_key_isolation
cargo test -p trustedge-receipts test_signature_forgery_resistance
cargo test -p trustedge-receipts test_replay_attack_resistance
cargo test -p trustedge-receipts test_amount_tampering_resistance
cargo test -p trustedge-receipts test_chain_integrity_validation
```

**Security Test Categories:**

1. **Cryptographic Key Isolation**: Ensures attackers cannot decrypt others' receipts
2. **Signature Forgery Resistance**: Prevents impersonation using Ed25519 signatures  
3. **Replay Attack Prevention**: Each receipt has unique cryptographic fingerprint
4. **Amount Tampering Resistance**: Receipt amounts are cryptographically protected
5. **Chain Integrity Validation**: Broken/out-of-order chains are rejected
6. **Memory Safety**: Cryptographic key material is properly zeroized
7. **Multi-Party Chain Testing**: Complex ownership transfer scenarios (Alice → Bob → Charlie → Dave → Eve)

### Envelope System Security Tests

**Real Cryptography Validation:**

**Core Envelope Tests:**
- **AES-256-GCM Encryption**: Real production cryptography with PBKDF2 key derivation
- **Key Derivation Security**: 100,000 PBKDF2 iterations with proper salt handling
- **Memory Safety**: Cryptographic key material properly zeroized after use
- **Tampering Detection**: Envelope verification fails when data is modified
- **Wrong Key Rejection**: Decryption fails with incorrect keys

**Manual Security Validation:**
```bash
# Test envelope tampering detection
./target/release/trustedge-core \
  --input test.txt \
  --envelope original.trst \
  --key-out test.key

# Tamper with envelope (should fail decryption)
dd if=/dev/urandom of=original.trst bs=1 seek=100 count=10 conv=notrunc

# Verify tampering is detected
./target/release/trustedge-core \
  --decrypt \
  --input original.trst \
  --out should_fail.txt \
  --key-hex $(cat test.key)
# Expected: Decryption failure due to tampering
  --key-hex $(openssl rand -hex 32)

# Verify wrong key detection (should fail)
./target/release/trustedge-core \
  --decrypt \
  --input test.trst \
  --out should_fail.txt \
  --key-hex $(openssl rand -hex 32)
# Expected: "AES-GCM decrypt/verify failed" error
```

#### 2. Salt Validation Tests
```bash
# Test PBKDF2 validation (should fail)
./target/release/trustedge-core \
  --decrypt \
  --input keyring_test.trst \
  --out should_fail.txt \
  --backend keyring \
  --salt-hex "deadbeefdeadbeefdeadbeefdeadbeef" \
  --use-keyring
# Should fail with "AES-GCM decrypt/verify failed"
```

[↑ Back to top](#table-of-contents)

---

## Network Testing

**📖 For network error diagnosis and connection troubleshooting, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md#network-problems).**

### Client-Server Testing

#### 1. Basic Network Flow Validation
```bash
# Terminal 1: Start server
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --decrypt \
  --key-hex $(openssl rand -hex 32) \
  --output-dir ./server_output \
  --verbose

# Terminal 2: Test client connection
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input test_audio.wav \
  --key-hex $(cat shared_key.hex) \
  --verbose
```

#### 2. Network Resilience Testing
```bash
# Test connection failure handling
./target/release/trustedge-client \
  --server 127.0.0.1:9999 \
  --input test.wav \
  --key-hex $(openssl rand -hex 32) \
  --retry-attempts 3 \
  --connect-timeout 5
# Expected: Connection refused with retry attempts

# Test authentication flow (if authentication enabled)
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --input test.wav \
  --require-auth \
  --client-identity "Test Client" \
  --verbose
```

[↑ Back to top](#table-of-contents)

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

[↑ Back to top](#table-of-contents)

---

## Continuous Integration Testing

### GitHub Actions Tests
The CI pipeline runs:
- ✅ `cargo test` - All unit and integration tests
- ✅ `cargo clippy` - Code quality and linting
- ✅ `cargo fmt --check` - Code formatting validation
- ✅ Cross-platform testing (Linux, macOS, Windows)
- ✅ Multiple Rust versions (stable, beta, nightly)

[↑ Back to top](#table-of-contents)

---

## Audio System Testing

### Prerequisites for Audio Testing

**Build with Audio Features:**
```bash
# Required: Build with audio support
cargo build --release --features audio

# Verify audio features are enabled
./target/release/trustedge-core --help | grep -i audio
```

**Install System Dependencies:**
```bash
# Linux (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install libasound2-dev pkg-config alsa-utils

# Verify ALSA installation
arecord --list-devices

# macOS (Homebrew - optional utilities)
brew install sox  # For audio testing utilities

# Windows
# Audio libraries included with Windows SDK
```

### Device Discovery and Validation

#### 1. List Available Audio Devices

```bash
# Always start with device discovery
./target/release/trustedge-core --list-audio-devices
```

**Expected Output Examples:**
```
Available audio input devices:
  - "hw:CARD=PCH,DEV=0" (Built-in Audio Analog Stereo)
  - "hw:CARD=USB_AUDIO,DEV=0" (USB Audio CODEC)
  - "default" (System Default)
  - "pulse" (PulseAudio System)
```

#### 2. Test Device Access

```bash
# Test with system default device
./target/release/trustedge-core \
  --live-capture \
  --max-duration 3 \
  --envelope test_default_device.trst \
  --key-hex $(openssl rand -hex 32)

# Test with specific device
./target/release/trustedge-core \
  --live-capture \
  --audio-device "hw:CARD=PCH,DEV=0" \
  --max-duration 3 \
  --envelope test_specific_device.trst \
  --key-hex $(openssl rand -hex 32)
```

### Common Audio Issues and Diagnostics

#### 1. No Audio Devices Found

**Symptoms:**
```
Error: No audio input devices found
```

**Diagnostic Steps:**
```bash
# Check system audio devices
arecord --list-devices  # Linux
system_profiler SPAudioDataType  # macOS
dxdiag  # Windows

# Check permissions (Linux)
groups $USER | grep audio
ls -la /dev/snd/

# Add user to audio group if needed
sudo usermod -a -G audio $USER
# Logout and login required
```

#### 2. Device Access Denied

**Symptoms:**
```
Error: Failed to open audio device: Permission denied
```

**Solutions:**
```bash
# Linux: Check audio group membership
sudo usermod -a -G audio $USER

# macOS: Check microphone permissions
# System Preferences → Security & Privacy → Privacy → Microphone
# Enable for Terminal or your application

# Test with PulseAudio (Linux)
./target/release/trustedge-core \
  --live-capture \
  --audio-device "pulse" \
  --max-duration 5 \
  --envelope test_pulse.trst \
  --key-hex $(openssl rand -hex 32)
```

#### 3. Silent Audio Capture

**Symptoms:** Audio captures but produces silent/empty audio

**Diagnostic Steps:**
```bash
# Test with system audio tools first
arecord -d 3 -f cd test_system_audio.wav  # Linux
sox -d test_system_audio.wav trim 0 3     # macOS/Linux with sox

# Check microphone levels
alsamixer  # Linux - check capture levels
# macOS: System Preferences → Sound → Input
# Windows: Sound Settings → Input → Device Properties

# Test with verbose output
./target/release/trustedge-core \
  --live-capture \
  --audio-device "default" \
  --max-duration 5 \
  --envelope test_levels.trst \
  --key-hex $(openssl rand -hex 32) \
  --verbose
```

#### 4. Invalid Device Name

**Symptoms:**
```
Error: Audio device "wrong_name" not found
```

**Solutions:**
```bash
# Always check exact device names first
./target/release/trustedge-core --list-audio-devices

# Copy device name exactly (with quotes)
./target/release/trustedge-core \
  --live-capture \
  --audio-device "hw:CARD=USB_AUDIO,DEV=0" \
  --max-duration 5 \
  --envelope test_correct_name.trst \
  --key-hex $(openssl rand -hex 32)

# Common device name patterns:
# Linux: "hw:CARD=CardName,DEV=0", "default", "pulse"
# macOS: "Built-in Microphone", "USB Audio CODEC"
# Windows: "Microphone (Realtek Audio)", "USB Audio Device"
```

#### 5. Audio Quality Issues

**Symptoms:** Choppy, distorted, or poor quality audio

**Solutions:**
```bash
# Check sample rate compatibility
./target/release/trustedge-core \
  --live-capture \
  --sample-rate 44100 \  # Try standard rates: 44100, 48000
  --channels 1 \         # Start with mono
  --chunk-duration-ms 1000 \  # Larger chunks for stability
  --max-duration 10 \
  --envelope test_quality.trst \
  --key-hex $(openssl rand -hex 32)

# Test different configurations
./target/release/trustedge-core \
  --live-capture \
  --sample-rate 48000 \
  --channels 2 \
  --chunk-duration-ms 500 \
  --max-duration 10 \
  --envelope test_hifi.trst \
  --key-hex $(openssl rand -hex 32)
```

### Audio Feature Testing Matrix

| Test Case | Command | Expected Result |
|-----------|---------|-----------------|
| Device Discovery | `--list-audio-devices` | Lists available devices |
| Default Device | `--live-capture --max-duration 3` | Captures 3 seconds |
| Specific Device | `--audio-device "hw:CARD=PCH,DEV=0"` | Uses specified device |
| High Quality | `--sample-rate 48000 --channels 2` | Stereo 48kHz capture |
| Long Capture | `--max-duration 60` | 1-minute capture |
| Unlimited Capture | `--max-duration 0` | Continues until Ctrl+C |

### Platform-Specific Testing

#### Linux (ALSA/PulseAudio)
```bash
# Test ALSA direct access
./target/release/trustedge-core \
  --live-capture \
  --audio-device "hw:CARD=PCH,DEV=0" \
  --max-duration 5 \
  --envelope test_alsa.trst \
  --key-hex $(openssl rand -hex 32)

# Test PulseAudio integration
./target/release/trustedge-core \
  --live-capture \
  --audio-device "pulse" \
  --max-duration 5 \
  --envelope test_pulse.trst \
  --key-hex $(openssl rand -hex 32)
```

#### macOS (Core Audio)
```bash
# Test built-in microphone
./target/release/trustedge-core \
  --live-capture \
  --audio-device "Built-in Microphone" \
  --max-duration 5 \
  --envelope test_builtin.trst \
  --key-hex $(openssl rand -hex 32)

# Test USB audio device
./target/release/trustedge-core \
  --live-capture \
  --audio-device "USB Audio CODEC" \
  --max-duration 5 \
  --envelope test_usb.trst \
  --key-hex $(openssl rand -hex 32)
```

#### Windows (WASAPI)
```bash
# Test default microphone
./target/release/trustedge-core.exe \
  --live-capture \
  --max-duration 5 \
  --envelope test_windows.trst \
  --key-hex $(openssl rand -hex 32)

# Test specific device
./target/release/trustedge-core.exe \
  --live-capture \
  --audio-device "Microphone (Realtek Audio)" \
  --max-duration 5 \
  --envelope test_realtek.trst \
  --key-hex $(openssl rand -hex 32)
```

### Audio Validation Testing

#### 1. Round-trip Audio Test with Format Verification
```bash
# Capture audio with known parameters
./target/release/trustedge-core \
  --live-capture \
  --sample-rate 44100 \
  --channels 2 \
  --max-duration 10 \
  --envelope captured_audio.trst \
  --key-out audio_key.hex \
  --verbose

# Decrypt and verify (produces raw PCM f32le data)
./target/release/trustedge-core \
  --decrypt \
  --input captured_audio.trst \
  --out recovered_audio.raw \
  --key-hex $(cat audio_key.hex) \
  --verbose

# Verify file size matches expected PCM data size
# Formula: size = sample_rate * channels * duration * 4 bytes (f32)
# Expected: 44100 * 2 * 10 * 4 = 3,528,000 bytes
actual_size=$(wc -c < recovered_audio.raw)
expected_size=$((44100 * 2 * 10 * 4))
echo "Actual size: $actual_size bytes, Expected: ~$expected_size bytes"

# Convert to playable format for verification
ffmpeg -f f32le -ar 44100 -ac 2 -i recovered_audio.raw test_playback.wav

# Verify conversion worked
ffprobe test_playback.wav 2>&1 | grep -E "(Duration|Stream|Audio)"
```

#### 2. PCM Format Validation
```bash
# Test different audio configurations
for sample_rate in 22050 44100 48000; do
  for channels in 1 2; do
    echo "Testing ${sample_rate}Hz, ${channels} channel(s)"
    
    # Capture with specific parameters
    ./target/release/trustedge-core \
      --live-capture \
      --sample-rate $sample_rate \
      --channels $channels \
      --max-duration 3 \
      --envelope test_${sample_rate}_${channels}ch.trst \
      --key-hex $(openssl rand -hex 32) \
      --verbose
    
    # Decrypt to raw PCM
    ./target/release/trustedge-core \
      --decrypt \
      --input test_${sample_rate}_${channels}ch.trst \
      --out test_${sample_rate}_${channels}ch.raw \
      --key-hex $(openssl rand -hex 32) \
      --verbose
    
    # Convert and validate
    ffmpeg -f f32le -ar $sample_rate -ac $channels \
      -i test_${sample_rate}_${channels}ch.raw \
      test_${sample_rate}_${channels}ch.wav
    
    # Check if playable
    ffprobe test_${sample_rate}_${channels}ch.wav >/dev/null 2>&1 && \
      echo "✅ ${sample_rate}Hz ${channels}ch: Valid" || \
      echo "❌ ${sample_rate}Hz ${channels}ch: Invalid"
  done
done
```

#### 3. Audio Metadata Verification
```bash
# Capture with metadata logging
./target/release/trustedge-core \
  --live-capture \
  --sample-rate 48000 \
  --channels 2 \
  --max-duration 5 \
  --envelope metadata_test.trst \
  --key-hex $(openssl rand -hex 32) \
  --verbose 2>&1 | tee capture_log.txt

# Decrypt with metadata extraction
./target/release/trustedge-core \
  --decrypt \
  --input metadata_test.trst \
  --out metadata_test.raw \
  --key-hex $(openssl rand -hex 32) \
  --verbose 2>&1 | tee decrypt_log.txt

# Verify metadata consistency
echo "Verifying audio metadata consistency:"
grep -E "Sample Rate|Channels|Format" capture_log.txt
grep -E "Sample Rate|Channels|Format" decrypt_log.txt

# Verify PCM data matches metadata
pcm_size=$(wc -c < metadata_test.raw)
sample_rate=$(grep "Sample Rate:" decrypt_log.txt | grep -o '[0-9]*')
channels=$(grep "Channels:" decrypt_log.txt | grep -o '[0-9]*')
duration=5
expected_size=$((sample_rate * channels * duration * 4))
echo "PCM size: $pcm_size, Expected: $expected_size (tolerance: ±10%)"
```
```

#### 2. Multi-Device Testing
```bash
# Test all available devices
for device in $(./target/release/trustedge-core --list-audio-devices | grep -o '"[^"]*"'); do
  echo "Testing device: $device"
  ./target/release/trustedge-core \
    --live-capture \
    --audio-device $device \
    --max-duration 3 \
    --envelope "test_${device//[^a-zA-Z0-9]/_}.trst" \
    --key-hex $(openssl rand -hex 32) || echo "Failed: $device"
done
```

[↑ Back to top](#table-of-contents)

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

[↑ Back to top](#table-of-contents)

---

For more information about the security model and threat analysis, see [THREAT_MODEL.md](./THREAT_MODEL.md).

For protocol details and technical specifications, see [PROTOCOL.md](./PROTOCOL.md).
