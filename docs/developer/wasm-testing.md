<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# WebAssembly Testing Guide

This document provides comprehensive guidance for testing TrustEdge WebAssembly modules in browser environments.

## Overview

TrustEdge includes two WebAssembly crates with comprehensive browser integration testing:

- **`trustedge-wasm`**: Core cryptographic operations (AES-256-GCM encryption/decryption)
- **`trst-wasm`**: Archive verification and .trst format validation

Both crates include real browser tests that verify functionality in actual browser environments using `wasm-bindgen-test`.

## Test Architecture

### Browser Integration Tests

All WASM tests are configured to run in actual browser environments:

```rust
#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);
```

### Test Categories

#### 1. **Core Functionality Tests**
- Module initialization and version checking
- Basic API interface validation
- WebAssembly module loading verification

#### 2. **Cryptographic Operation Tests**
- Real AES-256-GCM encryption/decryption cycles
- Key and nonce generation in browser environment
- Random number generation using browser crypto APIs
- Error handling for invalid inputs

#### 3. **Data Handling Tests**
- Large data encryption (10KB+ datasets)
- Unicode text processing
- JSON serialization/deserialization
- Memory efficiency validation

#### 4. **Archive Verification Tests**
- Manifest signature validation
- .trst format parsing and verification
- Error handling for malformed archives
- Different profile type support

#### 5. **Browser-Specific Tests**
- Performance characteristics in browser environment
- Memory management and cleanup
- Concurrent operation simulation
- Edge case handling

## Running WASM Tests

### Prerequisites

Install required tools:

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install test dependencies
npm install -g chromedriver geckodriver
```

### Running Tests in Browser

#### Chrome/Chromium
```bash
# Test trustedge-wasm in Chrome
cd trustedge-wasm
wasm-pack test --chrome --headless

# Test trst-wasm in Chrome
cd crates/wasm
wasm-pack test --chrome --headless
```

#### Firefox
```bash
# Test trustedge-wasm in Firefox
cd trustedge-wasm
wasm-pack test --firefox --headless

# Test trst-wasm in Firefox
cd crates/wasm
wasm-pack test --firefox --headless
```

#### Safari (macOS only)
```bash
# Test trustedge-wasm in Safari
cd trustedge-wasm
wasm-pack test --safari --headless

# Test trst-wasm in Safari
cd crates/wasm
wasm-pack test --safari --headless
```

### Running All WASM Tests
```bash
# Run all WASM tests across all browsers
make test-wasm-all
```

### Development Testing
```bash
# Run tests with browser window visible (for debugging)
cd trustedge-wasm
wasm-pack test --chrome

# Run specific test
wasm-pack test --chrome --headless -- --grep "test_browser_crypto_operations"
```

## Test Coverage

### trustedge-wasm Tests (18 comprehensive tests)

| Test Category | Test Count | Description |
|---------------|------------|-------------|
| **Initialization** | 1 | Module loading and version validation |
| **Cryptographic Operations** | 4 | Encryption, decryption, key generation |
| **Data Handling** | 6 | Large data, Unicode, JSON, memory efficiency |
| **Error Handling** | 3 | Invalid inputs, format validation |
| **Browser-Specific** | 4 | Performance, edge cases, deterministic operations |

**Key Tests:**
- `test_browser_crypto_operations`: Real AES-256-GCM encryption/decryption
- `test_browser_large_data_encryption`: 10KB data processing
- `test_browser_unicode_handling`: Unicode text encryption
- `test_browser_memory_efficiency`: 100-iteration stress test
- `test_browser_performance_characteristics`: Performance validation

### trst-wasm Tests (15 comprehensive tests)

| Test Category | Test Count | Description |
|---------------|------------|-------------|
| **Archive Verification** | 3 | Manifest and archive verification interfaces |
| **Error Handling** | 4 | Invalid data, keys, formats |
| **Data Processing** | 5 | Large manifests, Unicode content, different profiles |
| **Edge Cases** | 3 | Concurrent operations, memory efficiency, minimal structures |

**Key Tests:**
- `test_browser_manifest_verification_interface`: Core verification API
- `test_browser_large_manifest_handling`: 100-segment manifest processing
- `test_browser_unicode_in_manifest`: Unicode content handling
- `test_browser_concurrent_verification_simulation`: Concurrent operation testing
- `test_browser_error_message_quality`: Error message validation

## Test Quality Standards

### ✅ **Real Operations, No Mocking**
- All cryptographic operations use actual browser crypto APIs
- All verification uses real signature validation
- All data processing handles actual binary content

### ✅ **Browser Environment Validation**
- Tests verify WebAssembly module loads correctly in browsers
- JavaScript interop functionality is validated
- Browser-specific constraints are tested (memory, performance)

### ✅ **Comprehensive Error Handling**
- Invalid input validation
- Malformed data handling
- Browser-specific error scenarios
- Network constraint simulation

### ✅ **Security-Focused Testing**
- Real cryptographic key generation and validation
- Signature verification with actual algorithms
- Memory safety validation in browser environment
- Performance characteristics under browser constraints

## Continuous Integration

### CI Configuration

WASM tests are integrated into the main CI pipeline with proper browser automation:

```yaml
# .github/workflows/wasm-tests.yml
name: WASM Browser Tests
on: [push, pull_request]

jobs:
  wasm-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Install Chrome
        run: |
          wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
          echo 'deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main' | tee /etc/apt/sources.list.d/google-chrome.list
          apt-get update
          apt-get install google-chrome-stable

      - name: Test trustedge-wasm
        run: |
          cd trustedge-wasm
          wasm-pack test --chrome --headless

      - name: Test trst-wasm
        run: |
          cd crates/wasm
          wasm-pack test --chrome --headless
```

### Local Development Integration

Add to your `Makefile`:

```makefile
test-wasm: ## Run all WASM tests in Chrome
	@echo "Running WASM tests..."
	@cd trustedge-wasm && wasm-pack test --chrome --headless
	@cd crates/wasm && wasm-pack test --chrome --headless
	@echo "✅ All WASM tests passed"

test-wasm-all: ## Run WASM tests in all browsers
	@echo "Running WASM tests in all browsers..."
	@cd trustedge-wasm && wasm-pack test --chrome --headless && wasm-pack test --firefox --headless
	@cd crates/wasm && wasm-pack test --chrome --headless && wasm-pack test --firefox --headless
	@echo "✅ All cross-browser WASM tests passed"

test-wasm-dev: ## Run WASM tests with visible browser (for debugging)
	@echo "Running WASM tests in development mode..."
	@cd trustedge-wasm && wasm-pack test --chrome
```

## Debugging WASM Tests

### Browser Console Output

WASM tests include console logging for debugging:

```rust
#[wasm_bindgen_test]
fn test_browser_crypto_operations() {
    console_log!("Starting cryptographic operations test");

    // Test implementation...

    console_log!("Cryptographic operations test completed");
}
```

### Debugging Failed Tests

1. **Run with visible browser:**
   ```bash
   cd trustedge-wasm
   wasm-pack test --chrome  # Remove --headless
   ```

2. **Check browser console:**
   - Open Developer Tools
   - Look for console.log output from tests
   - Check for JavaScript errors

3. **Add debug output:**
   ```rust
   console_log!("Debug: key length = {}", key.len());
   ```

### Common Issues and Solutions

#### Issue: `wasm-pack not found`
**Solution:**
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
source ~/.bashrc
```

#### Issue: `chromedriver not found`
**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install chromium-chromedriver

# macOS
brew install chromedriver

# Manual download
wget https://chromedriver.storage.googleapis.com/LATEST_RELEASE
```

#### Issue: Tests timeout in CI
**Solution:** Increase timeout in test configuration:
```rust
#[wasm_bindgen_test]
#[timeout(30000)] // 30 seconds
fn test_large_operation() {
    // Test implementation
}
```

## Performance Benchmarks

WASM tests include performance validation:

```rust
#[wasm_bindgen_test]
fn test_browser_performance_characteristics() {
    let start_time = js_sys::Date::now();

    // Perform operations...

    let end_time = js_sys::Date::now();
    let duration = end_time - start_time;

    // Should complete in reasonable time
    assert!(duration < 1000.0, "Operation took too long: {}ms", duration);
}
```

### Expected Performance Targets

- **Small data encryption (1KB)**: < 10ms
- **Medium data encryption (10KB)**: < 100ms
- **Large data encryption (100KB)**: < 1000ms
- **Archive verification**: < 50ms
- **Key generation**: < 5ms

## Security Considerations

### Browser Security Context

WASM tests validate security in browser environments:

1. **Secure Random Generation**: Uses browser crypto APIs
2. **Memory Safety**: Validates no memory leaks in repeated operations
3. **Input Validation**: Tests all input validation paths
4. **Error Handling**: Ensures no sensitive data leaks in error messages

### Cryptographic Validation

All cryptographic tests use real algorithms:

```rust
#[wasm_bindgen_test]
fn test_browser_crypto_operations() {
    // Real AES-256-GCM encryption
    let key = generate_key();  // Real 256-bit key
    let encrypted = encrypt_simple(test_data, &key);  // Real encryption
    let decrypted = decrypt(&encrypted, &key);  // Real decryption

    // Verify byte-perfect recovery
    assert_eq!(decrypted, test_data);
}
```

## Future Enhancements

### Planned Test Additions

1. **Service Worker Tests**: Test WASM modules in service worker context
2. **Node.js Tests**: Validate Node.js compatibility
3. **Mobile Browser Tests**: Test on mobile browser engines
4. **WebWorker Tests**: Test in WebWorker environments
5. **Streaming Tests**: Test with large streaming data

### Integration with Main Test Suite

WASM tests are integrated with the main TrustEdge test suite:

```bash
# Run all tests including WASM
./scripts/ci-check.sh

# Just WASM tests
make test-wasm

# Full cross-platform validation
make test-all-platforms
```

This comprehensive WASM testing ensures TrustEdge cryptographic operations work reliably across all major browser environments with the same security guarantees as native implementations.