# Testing Patterns

**Analysis Date:** 2026-02-09

## Test Framework

**Runner:**
- Rust built-in test framework (no external runner; uses `cargo test`)
- Tokio for async tests: `#[tokio::test]` for async test functions
- 150+ tests across workspace (101 in `trustedge-core`, 23 in `trustedge-receipts`, 7 acceptance tests)

**Assertion Library:**
- Standard Rust assertions: `assert!()`, `assert_eq!()`, `assert_ne!()`
- Custom error messages supported: `assert_eq!(x, y, "message on failure")`
- No external assertion library; built-in macros sufficient for codebase

**Run Commands:**
```bash
# Run all tests
cargo test --workspace

# Test specific crate
cargo test -p trustedge-core --lib          # Unit tests only
cargo test -p trustedge-cli --test acceptance  # Integration tests

# Run single test with output
cargo test -p trustedge-core test_name -- --nocapture

# Test with specific features
cargo test --features yubikey --test yubikey_integration
cargo test --features audio

# Run benchmarks
cargo bench -p trustedge-core --bench crypto_benchmarks
cargo bench -p trustedge-core --bench network_benchmarks
```

## Test File Organization

**Location:**
- **Unit tests:** Colocated in same file as implementation in `mod tests` block at end of source files
  - Example: `crates/core/src/archive.rs` has `mod tests { ... }` at bottom
  - Example: `crates/attestation/src/lib.rs` has `mod tests { ... }`
- **Integration tests:** Separate files in `tests/` directory
  - `crates/core/tests/auth_integration.rs`
  - `crates/core/tests/universal_backend_integration.rs`
  - `crates/core/tests/transport_integration.rs`
  - `crates/core/tests/yubikey_integration.rs` (feature-gated)

**Naming:**
- Integration test files: `{component}_integration.rs` (e.g., `auth_integration.rs`, `network_integration.rs`)
- Unit test modules: `mod tests` (Rust convention)
- Helper functions in tests: `create_test_data()`, `write_test_file()`, `create_test_manifest()`

**Structure:**
```
crates/
├── core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── archive.rs       # Contains: mod tests { ... }
│   │   ├── auth.rs          # Contains: unit tests
│   │   └── ...
│   └── tests/               # Integration tests
│       ├── auth_integration.rs
│       ├── universal_backend_integration.rs
│       ├── transport_integration.rs
│       └── yubikey_integration.rs
└── attestation/
    └── src/
        └── lib.rs           # Contains: mod tests { ... }
```

## Test Structure

**Suite Organization:**
```rust
#[tokio::test]
async fn test_mutual_authentication() -> Result<()> {
    // Setup phase
    let mut server_manager = SessionManager::new("test-server".to_string())?;
    let client_cert = ClientCertificate::generate("test-client")?;

    // Test phase (spawn async tasks for client/server)
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let server_addr = listener.local_addr()?;

    // Verify phase
    assert_eq!(session_id, auth_session.session_id);

    // Cleanup (automatic via RAII)
    Ok(())
}
```

**Patterns:**
- **Setup:** Create test fixtures, initialize data structures with `new()` methods
  - Example: `SessionManager::new()`, `ClientCertificate::generate()`, `NamedTempFile::new()`
- **Execution:** Call the function/module being tested
  - Example: `client_authenticate(&mut stream, ...)`, `backend.perform_operation(...)`
- **Assertions:** Verify expected outcomes
  - Example: `assert_eq!(session_id, auth_session.session_id)`, `assert_eq!(client_cert.identity, "test-client")`
- **Cleanup:** Automatic through RAII and `Drop` implementations; no explicit cleanup needed
  - `NamedTempFile` automatically deletes on drop
  - `TcpListener` automatically closes on drop

**Async Test Pattern:**
```rust
#[tokio::test]
async fn test_session_management() -> Result<()> {
    // Spawn server task
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let mut server_manager_copy = session_manager.clone();
    let server_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let result = server_authenticate(&mut stream, &mut server_manager_copy).await;
        (result, server_manager_copy)
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Client connection with timeout
    let mut client_stream = timeout(Duration::from_secs(5), TcpStream::connect(server_addr)).await??;

    // Wait for server completion
    let (auth_result, mut updated_manager) = server_handle.await?;

    Ok(())
}
```

## Mocking

**Framework:** No external mocking library; uses in-process test doubles and traits

**Patterns:**
- **Test doubles:** Create minimal implementations of traits for testing
  - Example in `universal_backend_integration.rs`: Create test data with `create_test_data(size: usize) -> Vec<u8>`
  - Example: Write temporary files with `write_test_file(data: &[u8]) -> Result<NamedTempFile>`
- **Trait implementations:** Tests use actual trait implementations (no mock/stub trait)
  - Example: `UniversalBackendRegistry::with_defaults()?` creates real backends available on system
  - Tests skip gracefully if backends unavailable: `if backend_names.is_empty() { return Ok(()); }`
- **In-process servers:** Tests spawn actual server tasks with tokio for network tests
  - See `auth_integration.rs`: Creates real `TcpListener` and `TcpStream` for testing mutual auth

**What to Mock:**
- External system dependencies that are slow or unreliable (not present in this codebase; tests use real APIs)
- Hardware backends when unavailable: Tests skip YubiKey tests if hardware not detected
- File I/O: Use `tempfile` crate for temporary test files instead of mocking filesystem

**What NOT to Mock:**
- Cryptographic operations: Always test with real crypto to catch implementation bugs
- Serialization/deserialization: Use actual serde implementations
- Core library types: Test real `Envelope`, `SessionManager`, `ClientCertificate` implementations

## Fixtures and Factories

**Test Data:**
```rust
// Factory function for test data
fn create_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

// Factory for test manifest
fn create_test_manifest() -> CamVideoManifest {
    let mut manifest = CamVideoManifest::new();
    manifest.device.id = "TEST001".to_string();
    manifest.device.public_key = "ed25519:test_key".to_string();
    manifest.segments = vec![
        SegmentInfo {
            chunk_file: "00000.bin".to_string(),
            blake3_hash: hex::encode(crate::chain::segment_hash(b"test_chunk_0")),
            start_time: "2025-01-15T10:30:00Z".to_string(),
            duration_seconds: 2.0,
            continuity_hash: "placeholder".to_string(),
        },
        // ... more segments
    ];
    manifest
}

// Factory for temporary file
fn write_test_file(data: &[u8]) -> Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(data)?;
    file.flush()?;
    Ok(file)
}
```

**Location:**
- Fixture functions defined at top of test modules (after imports, before tests)
- Factories use descriptive names: `create_test_*()`, `new_test_*()`, `write_test_*()`
- Fixtures return owned types or `Result<T>` for resource allocation

**Fixture Cleanup:**
- `tempfile::NamedTempFile` automatically deleted on drop
- Sockets (`TcpListener`, `TcpStream`) automatically closed on drop
- No explicit cleanup required due to RAII patterns

## Coverage

**Requirements:** Not enforced; no coverage targets configured in `Cargo.toml`

**View Coverage:**
```bash
# Install tarpaulin (Rust code coverage tool)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html
```

**Coverage Status (observed):**
- `trustedge-core`: Heavily tested (101 tests)
- `trustedge-attestation`: Tested (functions in lib.rs covered by unit tests in `mod tests`)
- `trustedge-cli`: Integration tests use example files and real I/O
- `trustedge-receipts`: 23 dedicated tests

## Test Types

**Unit Tests:**
- Scope: Single function or small module
- Location: `mod tests` block at end of source file
- Example from `archive.rs`:
  ```rust
  mod tests {
      use super::*;
      use tempfile::TempDir;

      fn create_test_manifest() -> CamVideoManifest { ... }

      #[test]
      fn test_archive_validation() -> Result<()> {
          // Test manifest validation logic
          let manifest = create_test_manifest();
          assert!(manifest.validate().is_ok());
          Ok(())
      }
  }
  ```

**Integration Tests:**
- Scope: Multiple modules working together (e.g., client-server authentication)
- Location: `tests/{component}_integration.rs`
- Real network I/O (TCP sockets) and async tasks
- Example from `auth_integration.rs`:
  ```rust
  #[tokio::test]
  async fn test_mutual_authentication() -> Result<()> {
      // Sets up server and client, tests authentication flow
      let listener = TcpListener::bind("127.0.0.1:0").await?;
      let server_handle = tokio::spawn(async move { ... });
      let mut client_stream = TcpStream::connect(server_addr).await?;
      let (session_id, _) = client_authenticate(...).await?;
      assert_eq!(session_id, auth_session.session_id);
      Ok(())
  }
  ```

**E2E Tests:**
- Scope: Not explicitly named; some acceptance tests in `tests/` directory
- Example: `trustedge-trst-cli` has 7 acceptance tests for archive wrap/verify operations
- Run with: `cargo test -p trustedge-trst-cli --test acceptance`

**Feature-Gated Tests:**
- YubiKey tests require `yubikey` feature: `cargo test --features yubikey --test yubikey_integration`
- Audio tests require `audio` feature: `cargo test --features audio`
- Tests gracefully skip when dependencies unavailable

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_name() -> Result<()> {
    // Use tokio::spawn() for concurrent tasks
    let handle = tokio::spawn(async move {
        // Background task
    });

    // Use tokio::time::timeout() for deadline enforcement
    let result = timeout(Duration::from_secs(5), async_operation()).await??;

    // Wait for background task
    let output = handle.await?;

    Ok(())
}
```

**Error Testing:**
```rust
#[test]
fn test_error_case() {
    // Verify error is returned
    let result = failing_operation();
    assert!(result.is_err());

    // Verify specific error
    match result {
        Err(e) => assert!(e.to_string().contains("expected message")),
        Ok(_) => panic!("Expected error"),
    }
}
```

**Conditional Skipping:**
```rust
#[test]
fn test_backend_feature() -> Result<()> {
    let registry = UniversalBackendRegistry::with_defaults()?;
    let backend_names = registry.list_backend_names();

    // Skip test if no backends available
    if backend_names.is_empty() {
        println!("⚠  No backends available, skipping test");
        return Ok(());
    }

    // Test proceeds with available backend
    Ok(())
}
```

## Development Dependencies

**Key dev dependencies** (from `crates/core/Cargo.toml`):
- `tempfile = "3"` - Temporary files for test I/O
- `assert_cmd = "2"` - CLI testing (command invocation)
- `assert_fs = "1"` - Filesystem assertions
- `predicates = "3"` - Predicate combinators for assertions
- `criterion = { version = "0.5", features = ["html_reports", "async_tokio"] }` - Benchmarking

**Benchmarks:**
- Located in `benches/` directory: `crypto_benchmarks.rs`, `network_benchmarks.rs`
- Run with: `cargo bench -p trustedge-core`
- Use `criterion` crate for statistical analysis

---

*Testing analysis: 2026-02-09*
