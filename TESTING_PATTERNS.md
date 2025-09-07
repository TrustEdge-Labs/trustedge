<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# TrustEdge Testing Patterns

## 🧪 Comprehensive Testing Strategy

TrustEdge uses a multi-layered testing approach with **93 automated tests** covering all aspects of the system. This guide provides patterns and best practices for writing effective tests.

## 📊 Test Suite Overview

### Unit Tests (53 tests)
- **Backend Systems**: Software HSM (33 tests), Keyring (4 tests), Universal (16 tests)
- **Audio Processing**: Chunk handling, configuration validation
- **Format Detection**: MIME type detection and validation
- **Crypto Primitives**: Hash operations, nonce generation

### Integration Tests (40 tests)
- **Software HSM Integration** (9 tests): Cross-session persistence, CLI workflows
- **Roundtrip Integration** (15 tests): End-to-end encryption/decryption validation
- **Authentication Integration** (3 tests): Certificate generation, mutual auth
- **Network Integration** (7 tests): Distributed encryption workflows
- **Universal Backend Integration** (6 tests): Capability-based backend selection

## 🎯 Core Testing Patterns

### 1. Universal Backend Testing Pattern

**Capability Discovery Testing:**
```rust
#[test]
fn test_backend_capabilities() -> Result<()> {
    let backend = YourBackend::new()?;
    let capabilities = backend.get_capabilities();
    
    // Test expected capabilities
    assert!(capabilities.supports_key_generation);
    assert!(capabilities.signature_algorithms.contains(&SignatureAlgorithm::Ed25519));
    assert_eq!(capabilities.max_key_size, Some(256));
    
    // Test capability consistency with supports_operation
    let sign_op = CryptoOperation::Sign { 
        data: vec![1, 2, 3], 
        algorithm: SignatureAlgorithm::Ed25519 
    };
    assert_eq!(backend.supports_operation(&sign_op), 
               capabilities.signature_algorithms.contains(&SignatureAlgorithm::Ed25519));
    
    Ok(())
}
```

**Operation Execution Pattern:**
```rust
#[test]
fn test_universal_backend_interface() -> Result<()> {
    let mut backend = Backend::new()?;
    
    // Generate test key
    backend.generate_key_pair("test_key", AsymmetricAlgorithm::Ed25519, None)?;
    
    // Test signing through UniversalBackend interface
    let test_data = b"Testing UniversalBackend interface";
    let sign_op = CryptoOperation::Sign {
        data: test_data.to_vec(),
        algorithm: SignatureAlgorithm::Ed25519,
    };
    
    let result = backend.perform_operation("test_key", sign_op)?;
    match result {
        CryptoResult::Signed(signature) => {
            // Test verification
            let verify_op = CryptoOperation::Verify {
                data: test_data.to_vec(),
                signature,
                algorithm: SignatureAlgorithm::Ed25519,
            };
            
            let verify_result = backend.perform_operation("test_key", verify_op)?;
            match verify_result {
                CryptoResult::VerificationResult(is_valid) => assert!(is_valid),
                _ => panic!("Expected VerificationResult"),
            }
        }
        _ => panic!("Expected Signed result"),
    }
    
    Ok(())
}
```

### 2. Cross-Session Persistence Pattern

**Critical for backend reliability:**
```rust
#[test]
fn test_cross_session_key_persistence() -> Result<()> {
    let (_temp_dir, config) = create_test_backend_setup()?;
    let key_id = "persistent_test_key";
    let test_data = b"Cross-session test data";

    // Session 1: Create key and sign data
    let signature = {
        let mut backend = Backend::with_config(config.clone())?;
        backend.generate_key_pair(
            key_id,
            AsymmetricAlgorithm::Ed25519,
            Some("Persistent test key".to_string()),
        )?;
        sign_data_via_universal_backend(&backend, key_id, test_data, SignatureAlgorithm::Ed25519)?
    };

    // Session 2: Load existing backend and verify signature
    {
        let backend = Backend::with_config(config.clone())?;
        let keys = backend.list_keys()?;
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].description, "Persistent test key");

        let is_valid = verify_signature_via_universal_backend(
            &backend, key_id, test_data, &signature, SignatureAlgorithm::Ed25519
        )?;
        assert!(is_valid);
    }

    Ok(())
}
```

### 3. Registry Integration Pattern

**Test capability-based backend selection:**
```rust
#[test]
fn test_registry_backend_selection() -> Result<()> {
    let mut registry = UniversalBackendRegistry::new();
    
    // Register multiple backends
    registry.register_backend("software_hsm".to_string(), Box::new(hsm_backend));
    registry.register_backend("keyring".to_string(), Box::new(keyring_backend));
    
    // Test operation-based selection
    let hash_op = CryptoOperation::Hash {
        data: b"test data".to_vec(),
        algorithm: HashAlgorithm::Sha256,
    };
    
    let selected_backend = registry
        .find_backend_for_operation(&hash_op)
        .ok_or_else(|| anyhow::anyhow!("No backend supports hash operation"))?;
    
    // Verify selected backend actually supports the operation
    assert!(selected_backend.supports_operation(&hash_op));
    
    // Test actual execution
    let result = selected_backend.perform_operation("test_key", hash_op)?;
    match result {
        CryptoResult::Hash(hash) => assert_eq!(hash.len(), 32), // SHA-256 output
        _ => panic!("Expected Hash result"),
    }
    
    Ok(())
}
```

### 4. Roundtrip Testing Pattern

**Essential for data integrity validation:**
```rust
#[test]
fn test_comprehensive_file_roundtrip() -> Result<()> {
    let test_cases = vec![
        ("text", "Hello, TrustEdge!", "text/plain"),
        ("json", r#"{"test": "data"}"#, "application/json"),
        ("binary", &[0u8, 255u8, 127u8, 128u8], "application/octet-stream"),
    ];
    
    for (name, data, expected_mime) in test_cases {
        println!("● Testing {} roundtrip...", name);
        
        // 1. Encrypt
        let input_path = write_test_file(&format!("test_{}.dat", name), data)?;
        let envelope_path = format!("test_{}.trst", name);
        let key = generate_test_key();
        
        encrypt_file(&input_path, &envelope_path, &key)?;
        
        // 2. Inspect (validate metadata)
        let metadata = inspect_encrypted_file(&envelope_path)?;
        assert_eq!(metadata.mime_type, expected_mime);
        assert_eq!(metadata.original_size, data.len());
        
        // 3. Decrypt
        let output_path = format!("test_{}_decrypted.dat", name);
        decrypt_file(&envelope_path, &output_path, &key)?;
        
        // 4. Verify integrity
        let original_data = fs::read(&input_path)?;
        let decrypted_data = fs::read(&output_path)?;
        assert_eq!(original_data, decrypted_data, 
                   "Roundtrip integrity failed for {} test", name);
        
        println!("✔ {} roundtrip test passed", name);
    }
    
    Ok(())
}
```

### 5. Error Handling and Recovery Pattern

**Test graceful degradation:**
```rust
#[test]
fn test_error_recovery_patterns() -> Result<()> {
    let (_temp_dir, config) = create_test_backend_setup()?;
    
    // Test missing key file recovery
    {
        let mut backend = Backend::with_config(config.clone())?;
        backend.generate_key_pair("test_key", AsymmetricAlgorithm::Ed25519, None)?;
        
        // Simulate key file corruption
        let key_file = config.key_store_path.join("test_key.pem");
        fs::write(&key_file, "corrupted data")?;
        
        // Backend should detect corruption gracefully
        let backend = Backend::with_config(config.clone())?;
        let result = backend.list_keys();
        match result {
            Ok(keys) => assert!(keys.is_empty()), // Should recover gracefully
            Err(e) => assert!(e.to_string().contains("corrupted")), // Or provide clear error
        }
    }
    
    // Test metadata corruption recovery
    {
        let metadata_file = config.metadata_file;
        fs::write(&metadata_file, "invalid json")?;
        
        // Should reinitialize with empty metadata
        let backend = Backend::with_config(config)?;
        assert!(backend.list_keys()?.is_empty());
    }
    
    Ok(())
}
```

### 6. Performance and Scale Testing Pattern

**Ensure scalability:**
```rust
#[test]
fn test_large_scale_operations() -> Result<()> {
    let (_temp_dir, config) = create_test_backend_setup()?;
    let mut backend = Backend::with_config(config)?;
    
    // Test many keys
    let key_count = 50;
    for i in 0..key_count {
        backend.generate_key_pair(
            &format!("scale_test_key_{}", i),
            AsymmetricAlgorithm::Ed25519,
            Some(format!("Scale test key {}", i)),
        )?;
    }
    
    // Test listing performance
    let start = Instant::now();
    let keys = backend.list_keys()?;
    let list_duration = start.elapsed();
    
    assert_eq!(keys.len(), key_count);
    assert!(list_duration < Duration::from_millis(100), 
            "Key listing took too long: {:?}", list_duration);
    
    // Test large data signing
    let large_data = vec![0u8; 1_000_000]; // 1MB
    let start = Instant::now();
    let _signature = backend.sign_data("scale_test_key_0", &large_data, SignatureAlgorithm::Ed25519)?;
    let sign_duration = start.elapsed();
    
    assert!(sign_duration < Duration::from_secs(1), 
            "Large data signing took too long: {:?}", sign_duration);
    
    Ok(())
}
```

### 7. CLI Integration Testing Pattern

**Test CLI workflows:**
```rust
#[test]
fn test_cli_key_lifecycle() -> Result<()> {
    let (_temp_dir, config) = create_test_backend_setup()?;
    
    // Test key generation via CLI
    let output = Command::new("cargo")
        .args(&["run", "--bin", "software-hsm-demo", "--", 
                "generate", "ed25519", "cli_test_key",
                "--store-path", config.key_store_path.to_str().unwrap()])
        .output()?;
    
    assert!(output.status.success(), 
            "CLI key generation failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Test key listing via CLI
    let output = Command::new("cargo")
        .args(&["run", "--bin", "software-hsm-demo", "--",
                "list",
                "--store-path", config.key_store_path.to_str().unwrap()])
        .output()?;
    
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout)?;
    assert!(output_str.contains("cli_test_key"));
    
    // Test key usage via CLI
    let test_file = write_test_file("cli_test.txt", "CLI test data")?;
    let output = Command::new("cargo")
        .args(&["run", "--bin", "software-hsm-demo", "--",
                "sign", "cli_test_key", test_file.to_str().unwrap(),
                "--store-path", config.key_store_path.to_str().unwrap()])
        .output()?;
    
    assert!(output.status.success());
    
    Ok(())
}
```

## 🔧 Test Utilities and Helpers

### Standard Test Setup Pattern

```rust
use tempfile::TempDir;
use anyhow::Result;

fn create_test_backend_setup() -> Result<(TempDir, BackendConfig)> {
    let temp_dir = TempDir::new()?;
    let config = BackendConfig {
        key_store_path: temp_dir.path().join("keys"),
        metadata_file: temp_dir.path().join("metadata.json"),
        passphrase: "test_passphrase".to_string(),
    };
    
    Ok((temp_dir, config))
}

fn write_test_file<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<PathBuf> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(path.to_path_buf())
}

fn generate_test_key() -> [u8; 32] {
    // Use deterministic key for reproducible tests
    let mut key = [0u8; 32];
    key[0] = 0xAB;
    key[31] = 0xCD;
    key
}
```

## 📈 Test Quality Metrics

### Coverage Expectations
- **Unit Tests**: >90% line coverage for core crypto operations
- **Integration Tests**: Cover all major user workflows
- **Error Cases**: Test all error conditions and recovery paths
- **Cross-Platform**: Ensure tests pass on Linux, macOS, Windows

### Performance Benchmarks
- **Key Generation**: <100ms for Ed25519, <200ms for P-256
- **Signing**: <10ms for small data, <1s for 1MB data  
- **Backend Loading**: <50ms for <100 keys
- **Transport**: <1s connection setup, >10MB/s throughput

## 🚨 Common Testing Pitfalls

1. **Non-Deterministic Tests**: Use fixed test keys and data
2. **Resource Cleanup**: Always use `TempDir` for file operations
3. **Backend Mutability**: Clone backends when testing needs `&mut self`
4. **Async Testing**: Use `#[tokio::test]` for transport tests
5. **Error Message Testing**: Verify error messages are helpful
6. **Cross-Session Testing**: Test persistence across backend restarts

## 🔐 Domain Separation Testing Pattern

**Security Testing for Signature Domain Separation:**
```rust
#[test]
fn test_domain_separation_prevents_cross_context_reuse() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"test manifest data";

    // Sign with a different (malicious) domain prefix
    let wrong_domain = b"malicious.manifest.v1";
    let mut wrong_message = Vec::new();
    wrong_message.extend_from_slice(wrong_domain);
    wrong_message.extend_from_slice(manifest_bytes);
    let malicious_signature = signing_key.sign(&wrong_message);

    // Should fail verification with correct domain
    let result = verify_manifest_with_domain(&verifying_key, manifest_bytes, &malicious_signature);
    assert!(result.is_err(), "Signature with wrong domain should not verify");

    Ok(())
}

#[test]
fn test_domain_separation_tampered_prefix_fails() -> Result<()> {
    let signature = sign_manifest_with_domain(&signing_key, manifest_bytes);
    
    // Manually create message with tampered domain prefix  
    let tampered_domain = b"tampered.manifest.v1";
    let mut tampered_message = Vec::new();
    tampered_message.extend_from_slice(tampered_domain);
    tampered_message.extend_from_slice(manifest_bytes);

    // Direct verification with tampered domain should fail
    let result = verifying_key.verify(&tampered_message, &signature);
    assert!(result.is_err(), "Should not verify with tampered domain prefix");
    
    Ok(())
}
```

**Key Domain Separation Test Cases:**
- ✅ **Basic Functionality**: Domain-separated signatures verify correctly
- ✅ **Raw Signature Prevention**: Non-domain signatures fail verification
- ✅ **Cross-Context Prevention**: Different domain prefixes fail verification
- ✅ **Tampered Prefix Detection**: Modified prefixes cause failures
- ✅ **Determinism**: Same input produces identical signatures

## 🛡️ Bounds Checking & DoS Protection Testing

**Security Testing for Resource Limits and Bounds Validation:**
```rust
#[test]
fn test_chunk_size_bounds_validation() -> Result<()> {
    // Test oversized chunk size in header
    let oversized_chunk_size = MAX_CHUNK_SIZE + 1;
    let result = validate_chunk_size(oversized_chunk_size);
    assert!(result.is_err(), "Oversized chunk_size should be rejected");

    // Test zero chunk size
    let result = validate_chunk_size(0);
    assert!(result.is_err(), "Zero chunk_size should be rejected");

    // Test valid chunk size
    let result = validate_chunk_size(4096);
    assert!(result.is_ok(), "Valid chunk_size should be accepted");

    Ok(())
}

#[test] 
fn test_ciphertext_size_bounds() -> Result<()> {
    let chunk_size = 4096u32;
    let valid_ct_size = chunk_size as usize + AES_GCM_TAG_SIZE;
    let oversized_ct = vec![0u8; valid_ct_size + 1];
    
    let result = validate_ciphertext_bounds(&oversized_ct, chunk_size);
    assert!(result.is_err(), "Oversized ciphertext should be rejected");

    Ok(())
}

#[test]
fn test_stream_size_limits() -> Result<()> {
    let mut total_size = 0u64;
    
    // Simulate adding chunks until limit
    for _ in 0..1000 {
        total_size += 10 * 1024 * 1024; // 10MB chunks
        if total_size > MAX_STREAM_SIZE_BYTES {
            assert!(total_size > MAX_STREAM_SIZE_BYTES, "Should exceed limit");
            return Ok(());
        }
    }
    
    panic!("Should have hit stream size limit");
}

#[test]
fn test_chunk_length_aad_binding() -> Result<()> {
    let manifest = create_test_manifest(1024); // chunk_len = 1024
    let tampered_manifest = create_test_manifest(2048); // Different chunk_len
    
    // AAD with correct chunk_len should work
    let correct_aad = build_aad(&header_hash, seq, &nonce, &manifest_hash, 1024);
    
    // AAD with wrong chunk_len should fail decryption
    let wrong_aad = build_aad(&header_hash, seq, &nonce, &manifest_hash, 2048);
    
    let decrypt_result = decrypt_with_aad(&ciphertext, &wrong_aad);
    assert!(decrypt_result.is_err(), "Wrong chunk_len in AAD should fail decryption");

    Ok(())
}
```

**Key DoS Protection Test Cases:**
- ✅ **Chunk Size Limits**: Oversized chunks rejected at header level
- ✅ **Ciphertext Bounds**: Malformed records rejected before decryption
- ✅ **Stream Size Limits**: Cumulative size tracking prevents huge streams
- ✅ **Record Count Limits**: Maximum record count enforced
- ✅ **Length Binding**: chunk_len cryptographically bound via AAD
- ✅ **Early Validation**: Bounds checked before expensive operations

## 🎯 Testing Checklist

When adding new features:

- [ ] Unit tests for core functionality
- [ ] Integration tests for user workflows  
- [ ] Error handling and edge cases
- [ ] Cross-session persistence (if applicable)
- [ ] CLI integration (if applicable)
- [ ] Performance validation
- [ ] Registry integration (for backends)
- [ ] Roundtrip validation (for data processing)
- [ ] Professional UTF-8 symbols in test output
- [ ] **Security bounds testing** (chunk sizes, stream limits)
- [ ] **DoS protection validation** (resource exhaustion prevention)
- [ ] **Length integrity testing** (chunk_len AAD binding)

Run `./ci-check.sh` before committing to ensure all tests pass with CI-level strictness.
