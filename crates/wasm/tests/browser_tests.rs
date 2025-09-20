//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Browser Integration Tests for TrustEdge WASM
//!
//! These tests run in actual browser environments using wasm-bindgen-test.
//! They verify that the WASM module functions correctly in browser contexts
//! with real browser APIs and constraints.

#![cfg(target_arch = "wasm32")]

use trustedge_wasm::*;
use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_module_initialization() {
    // Test that the WASM module initializes correctly in browser
    let version = version();
    assert!(!version.is_empty());

    // Test basic functionality works
    let result = test_basic_functionality();
    assert_eq!(result, "TrustEdge WASM is working correctly!");
}

#[wasm_bindgen_test]
fn test_browser_crypto_operations() {
    // Test AES-256-GCM encryption/decryption in browser environment
    let test_data = "Hello, TrustEdge WASM in browser!";
    let key = generate_key();

    // Validate key generation
    assert!(validate_key(&key));

    // Test encryption
    let encrypted = encrypt_simple(test_data, &key).expect("Encryption failed");
    assert!(!encrypted.ciphertext().is_empty());
    assert!(!encrypted.nonce().is_empty());

    // Test decryption
    let decrypted = decrypt(&encrypted, &key).expect("Decryption failed");
    assert_eq!(decrypted, test_data);
}

#[wasm_bindgen_test]
fn test_browser_random_generation() {
    // Test that random number generation works in browser context
    let key1 = generate_key();
    let key2 = generate_key();

    // Keys should be different (extremely high probability)
    assert_ne!(key1, key2);

    // Test random bytes generation
    let random_bytes = generate_random_bytes(32);
    assert_eq!(random_bytes.len(), 44); // Base64 encoded 32 bytes = 44 chars

    // Generate different random bytes
    let random_bytes2 = generate_random_bytes(32);
    assert_ne!(random_bytes, random_bytes2);
}

#[wasm_bindgen_test]
fn test_browser_nonce_generation() {
    // Test nonce generation and validation in browser
    let nonce1 = generate_nonce();
    let nonce2 = generate_nonce();

    // Nonces should be different
    assert_ne!(nonce1, nonce2);

    // Nonces should be valid
    assert!(validate_nonce(&nonce1));
    assert!(validate_nonce(&nonce2));
}

#[wasm_bindgen_test]
fn test_browser_error_handling() {
    // Test error handling in browser environment

    // Invalid key length
    let result = encrypt_simple("test", "invalid_key");
    assert!(result.is_err());

    // Invalid base64 key
    let result = encrypt_simple("test", "not_base64!");
    assert!(result.is_err());

    // Key validation should fail for invalid formats
    assert!(!validate_key("invalid"));
    assert!(!validate_key(""));
    assert!(!validate_nonce("invalid"));
}

#[wasm_bindgen_test]
fn test_browser_large_data_encryption() {
    // Test encryption of larger datasets in browser
    let large_data = "x".repeat(10000); // 10KB of data
    let key = generate_key();

    let encrypted = encrypt_simple(&large_data, &key).expect("Large data encryption failed");
    let decrypted = decrypt(&encrypted, &key).expect("Large data decryption failed");

    assert_eq!(decrypted, large_data);
    assert_eq!(decrypted.len(), 10000);
}

#[wasm_bindgen_test]
fn test_browser_json_serialization() {
    // Test JSON serialization/deserialization in browser context
    let test_data = "JSON serialization test";
    let key = generate_key();

    let encrypted = encrypt_simple(test_data, &key).expect("Encryption failed");

    // Test JSON serialization
    let json = encrypted.to_json().expect("JSON serialization failed");
    assert!(json.contains("ciphertext"));
    assert!(json.contains("nonce"));

    // Test JSON deserialization
    let deserialized = EncryptedData::from_json(&json).expect("JSON deserialization failed");
    assert_eq!(deserialized.ciphertext(), encrypted.ciphertext());
    assert_eq!(deserialized.nonce(), encrypted.nonce());

    // Verify decryption still works
    let decrypted = decrypt(&deserialized, &key).expect("Decryption after JSON roundtrip failed");
    assert_eq!(decrypted, test_data);
}

#[wasm_bindgen_test]
fn test_browser_multiple_encryptions() {
    // Test multiple independent encryptions in browser
    let messages = vec![
        "First message",
        "Second message with more content",
        "Third message with special chars: !@#$%^&*()",
        "Fourth message with unicode: 🔒🌐📄",
    ];

    let key = generate_key();
    let mut encrypted_messages = Vec::new();

    // Encrypt all messages
    for msg in &messages {
        let encrypted = encrypt_simple(msg, &key).expect("Encryption failed");
        encrypted_messages.push(encrypted);
    }

    // Decrypt all messages and verify
    for (i, encrypted) in encrypted_messages.iter().enumerate() {
        let decrypted = decrypt(encrypted, &key).expect("Decryption failed");
        assert_eq!(decrypted, messages[i]);
    }
}

#[wasm_bindgen_test]
fn test_browser_unicode_handling() {
    // Test Unicode text handling in browser environment
    let unicode_text = "Testing Unicode: 🔐 安全 سلامة 安全性 безопасность セキュリティ";
    let key = generate_key();

    let encrypted = encrypt_simple(unicode_text, &key).expect("Unicode encryption failed");
    let decrypted = decrypt(&encrypted, &key).expect("Unicode decryption failed");

    assert_eq!(decrypted, unicode_text);
}

#[wasm_bindgen_test]
fn test_browser_memory_efficiency() {
    // Test that repeated operations don't cause memory issues
    let key = generate_key();
    let test_data = "Memory efficiency test";

    // Perform many encryption/decryption cycles
    for i in 0..100 {
        let data = format!("{} - iteration {}", test_data, i);
        let encrypted = encrypt_simple(&data, &key).expect("Encryption failed");
        let decrypted = decrypt(&encrypted, &key).expect("Decryption failed");
        assert_eq!(decrypted, data);
    }
}

#[wasm_bindgen_test]
fn test_browser_deterministic_encryption() {
    // Test that providing the same nonce produces deterministic results
    let test_data = "Deterministic test";
    let key = generate_key();
    let nonce = generate_nonce();

    let encrypted1 =
        encrypt(test_data, &key, Some(nonce.clone())).expect("First encryption failed");
    let encrypted2 = encrypt(test_data, &key, Some(nonce)).expect("Second encryption failed");

    // With same nonce, ciphertext should be identical
    assert_eq!(encrypted1.ciphertext(), encrypted2.ciphertext());
    assert_eq!(encrypted1.nonce(), encrypted2.nonce());
}

#[wasm_bindgen_test]
fn test_browser_key_id_functionality() {
    // Test key ID functionality for encrypted data
    let test_data = "Key ID test";
    let key = generate_key();

    let encrypted = encrypt_simple(test_data, &key).expect("Encryption failed");

    // Key ID should initially be None
    assert!(encrypted.key_id().is_none());

    // Test creating encrypted data with key ID
    let encrypted_with_id = EncryptedData::new(
        encrypted.ciphertext(),
        encrypted.nonce(),
        Some("test-key-123".to_string()),
    );

    assert_eq!(encrypted_with_id.key_id(), Some("test-key-123".to_string()));

    // Decryption should still work
    let decrypted = decrypt(&encrypted_with_id, &key).expect("Decryption with key ID failed");
    assert_eq!(decrypted, test_data);
}

#[wasm_bindgen_test]
fn test_browser_edge_cases() {
    // Test edge cases specific to browser environment

    // Empty string encryption
    let key = generate_key();
    let encrypted = encrypt_simple("", &key).expect("Empty string encryption failed");
    let decrypted = decrypt(&encrypted, &key).expect("Empty string decryption failed");
    assert_eq!(decrypted, "");

    // Single character
    let encrypted = encrypt_simple("a", &key).expect("Single char encryption failed");
    let decrypted = decrypt(&encrypted, &key).expect("Single char decryption failed");
    assert_eq!(decrypted, "a");

    // Whitespace only
    let encrypted = encrypt_simple("   ", &key).expect("Whitespace encryption failed");
    let decrypted = decrypt(&encrypted, &key).expect("Whitespace decryption failed");
    assert_eq!(decrypted, "   ");
}

#[wasm_bindgen_test]
fn test_browser_performance_characteristics() {
    // Test performance characteristics in browser
    let key = generate_key();
    let medium_data = "x".repeat(1000); // 1KB

    // Should complete quickly even in browser
    let start_time = js_sys::Date::now();

    let encrypted = encrypt_simple(&medium_data, &key).expect("Performance test encryption failed");
    let _decrypted = decrypt(&encrypted, &key).expect("Performance test decryption failed");

    let end_time = js_sys::Date::now();
    let duration = end_time - start_time;

    // Should complete in reasonable time (less than 1 second)
    assert!(
        duration < 1000.0,
        "Encryption/decryption took too long: {}ms",
        duration
    );
}
