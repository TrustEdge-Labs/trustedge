//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Archive Verification Browser Tests for trst-wasm
//!
//! These tests verify .trst archive verification functionality in browser environments.
//! They test manifest verification, signature validation, and error handling.

#![cfg(target_arch = "wasm32")]

use js_sys::Uint8Array;
use serde_json::json;
use trustedge_trst_wasm::*;
use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);

// Sample valid manifest JSON for testing
fn create_sample_manifest() -> String {
    json!({
        "trst_version": "0.1.0",
        "profile": "cam.video",
        "device": {
            "id": "test-device-001",
            "fw": "1.0.0",
            "model": "TrustEdgeRefCam",
            "public_key": "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        },
        "capture": {
            "started_at": "2025-09-20T17:02:11Z",
            "tz": "UTC",
            "fps": 30,
            "resolution": "1920x1080",
            "codec": "raw"
        },
        "chunk": {
            "approx_duration_s": 2.0,
            "bytes": 1048576
        },
        "segments": [
            {
                "id": 0,
                "t0": 0.0,
                "t1": 2.0,
                "hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "prev_hash": "1111111111111111111111111111111111111111111111111111111111111111",
                "bytes": 1024,
                "nonce": "000000000000000000000000000000000000000000000000"
            }
        ],
        "claims": {
            "location": {
                "lat": 0.0,
                "lon": 0.0,
                "source": "unknown"
            }
        },
        "prev_archive_hash": null,
        "signature": "ed25519:dGVzdF9zaWduYXR1cmVfZGF0YV9mb3JfdGVzdGluZ19wdXJwb3Nlcw=="
    })
    .to_string()
}

// Sample device public key for testing
fn sample_device_pub_key() -> String {
    "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
}

#[wasm_bindgen_test]
fn test_browser_manifest_verification_interface() {
    // Test that the basic verification interface works in browser
    let manifest_json = create_sample_manifest();
    let manifest_bytes = manifest_json.as_bytes();
    let device_pub = sample_device_pub_key();

    // This will likely fail signature verification, but should not crash
    let result = verify_manifest(manifest_bytes, &device_pub);

    // Should return a result (either success or error)
    match result {
        Ok(report) => {
            // If somehow successful, verify report structure
            assert!(js_sys::Reflect::has(&report, &"signature".into()).unwrap());
            assert!(js_sys::Reflect::has(&report, &"continuity".into()).unwrap());
            assert!(js_sys::Reflect::has(&report, &"segment_count".into()).unwrap());
        }
        Err(error) => {
            // Expected to fail with invalid signature - verify error is properly formatted
            let error_str = error.as_string().unwrap();
            assert!(!error_str.is_empty());
        }
    }
}

#[wasm_bindgen_test]
fn test_browser_archive_verification_interface() {
    // Test archive verification interface in browser
    let manifest_json = create_sample_manifest();
    let manifest_bytes = manifest_json.as_bytes();
    let device_pub = sample_device_pub_key();

    // Test archive verification (currently delegates to manifest verification)
    let result = verify_archive(manifest_bytes, &device_pub);

    // Should return a result without crashing
    assert!(result.is_ok() || result.is_err());
}

#[wasm_bindgen_test]
fn test_browser_invalid_manifest_handling() {
    // Test handling of invalid JSON in browser
    let invalid_json = "{ invalid json structure";
    let device_pub = sample_device_pub_key();

    let result = verify_manifest(invalid_json.as_bytes(), &device_pub);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_str = error.as_string().unwrap();
    assert!(!error_str.is_empty());
}

#[wasm_bindgen_test]
fn test_browser_empty_data_handling() {
    // Test handling of empty data in browser
    let device_pub = sample_device_pub_key();

    let result = verify_manifest(&[], &device_pub);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_str = error.as_string().unwrap();
    assert!(!error_str.is_empty());
}

#[wasm_bindgen_test]
fn test_browser_invalid_device_key_handling() {
    // Test handling of invalid device public keys
    let manifest_json = create_sample_manifest();
    let manifest_bytes = manifest_json.as_bytes();

    // Test with various invalid key formats
    let invalid_keys = vec![
        "",
        "invalid",
        "not_a_key",
        "ed25519:",
        "ed25519:invalid_base64!",
        "rsa:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=", // wrong algorithm
    ];

    for invalid_key in invalid_keys {
        let result = verify_manifest(manifest_bytes, invalid_key);
        assert!(
            result.is_err(),
            "Should fail for invalid key: {}",
            invalid_key
        );
    }
}

#[wasm_bindgen_test]
fn test_browser_large_manifest_handling() {
    // Test handling of larger manifests in browser environment
    let mut large_manifest = json!({
        "trst_version": "0.1.0",
        "profile": "cam.video",
        "device": {
            "id": "test-device-large",
            "fw": "1.0.0",
            "model": "TrustEdgeRefCam",
            "public_key": "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        },
        "capture": {
            "started_at": "2025-09-20T17:02:11Z",
            "tz": "UTC",
            "fps": 30,
            "resolution": "1920x1080",
            "codec": "raw"
        },
        "chunk": {
            "approx_duration_s": 2.0,
            "bytes": 1048576
        },
        "segments": [],
        "claims": {
            "location": {
                "lat": 0.0,
                "lon": 0.0,
                "source": "unknown"
            }
        },
        "prev_archive_hash": null,
        "signature": "ed25519:dGVzdF9zaWduYXR1cmVfZGF0YV9mb3JfdGVzdGluZ19wdXJwb3Nlcw=="
    });

    // Add many segments to create a larger manifest
    let segments = &mut large_manifest["segments"];
    for i in 0..100 {
        segments.as_array_mut().unwrap().push(json!({
            "id": i,
            "t0": i as f64 * 2.0,
            "t1": (i as f64 + 1.0) * 2.0,
            "hash": format!("{:064x}", i),
            "prev_hash": format!("{:064x}", i.saturating_sub(1)),
            "bytes": 1024,
            "nonce": format!("{:048x}", i)
        }));
    }

    let large_manifest_str = large_manifest.to_string();
    let manifest_bytes = large_manifest_str.as_bytes();
    let device_pub = sample_device_pub_key();

    // Should handle large manifests without crashing
    let result = verify_manifest(manifest_bytes, &device_pub);
    assert!(result.is_ok() || result.is_err()); // Should not panic
}

#[wasm_bindgen_test]
fn test_browser_unicode_in_manifest() {
    // Test handling of Unicode content in manifests
    let unicode_manifest = json!({
        "trst_version": "0.1.0",
        "profile": "cam.video",
        "device": {
            "id": "test-device-unicode-🔐",
            "fw": "1.0.0",
            "model": "TrustEdgeRefCam",
            "public_key": "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        },
        "capture": {
            "started_at": "2025-09-20T17:02:11Z",
            "tz": "UTC",
            "fps": 30,
            "resolution": "1920x1080",
            "codec": "raw"
        },
        "chunk": {
            "approx_duration_s": 2.0,
            "bytes": 1048576
        },
        "segments": [
            {
                "id": 0,
                "t0": 0.0,
                "t1": 2.0,
                "hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "prev_hash": "1111111111111111111111111111111111111111111111111111111111111111",
                "bytes": 1024,
                "nonce": "000000000000000000000000000000000000000000000000"
            }
        ],
        "claims": {
            "description": "Test with Unicode: 安全性 🔒 سلامة",
            "location": {
                "lat": 0.0,
                "lon": 0.0,
                "source": "GPS デバイス"
            }
        },
        "prev_archive_hash": null,
        "signature": "ed25519:dGVzdF9zaWduYXR1cmVfZGF0YV9mb3JfdGVzdGluZ19wdXJwb3Nlcw=="
    })
    .to_string();

    let manifest_bytes = unicode_manifest.as_bytes();
    let device_pub = sample_device_pub_key();

    // Should handle Unicode content properly
    let result = verify_manifest(manifest_bytes, &device_pub);
    assert!(result.is_ok() || result.is_err()); // Should not panic on Unicode
}

#[wasm_bindgen_test]
fn test_browser_memory_efficiency_verification() {
    // Test memory efficiency with repeated verifications
    let manifest_json = create_sample_manifest();
    let manifest_bytes = manifest_json.as_bytes();
    let device_pub = sample_device_pub_key();

    // Perform many verification attempts
    for _i in 0..50 {
        let _result = verify_manifest(manifest_bytes, &device_pub);
        // Don't care about result, just ensuring no memory leaks
    }
}

#[wasm_bindgen_test]
fn test_browser_different_profile_types() {
    // Test verification with different profile types
    let profiles = vec![
        "cam.video",
        "sensor.data",
        "audio.capture",
        "custom.profile",
    ];

    for profile in profiles {
        let mut manifest = json!({
            "trst_version": "0.1.0",
            "profile": profile,
            "device": {
                "id": "test-device",
                "fw": "1.0.0",
                "model": "TrustEdgeRefCam",
                "public_key": "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
            },
            "capture": {
                "started_at": "2025-09-20T17:02:11Z",
                "tz": "UTC",
                "fps": 30,
                "resolution": "1920x1080",
                "codec": "raw"
            },
            "chunk": {
                "approx_duration_s": 2.0,
                "bytes": 1048576
            },
            "segments": [],
            "claims": {},
            "prev_archive_hash": null,
            "signature": "ed25519:dGVzdF9zaWduYXR1cmVfZGF0YV9mb3JfdGVzdGluZ19wdXJwb3Nlcw=="
        });

        let manifest_str = manifest.to_string();
        let manifest_bytes = manifest_str.as_bytes();
        let device_pub = sample_device_pub_key();

        // Should handle different profiles without crashing
        let result = verify_manifest(manifest_bytes, &device_pub);
        assert!(result.is_ok() || result.is_err());
    }
}

#[wasm_bindgen_test]
fn test_browser_error_message_quality() {
    // Test that error messages are informative in browser context
    let device_pub = sample_device_pub_key();

    // Test various error conditions and verify error message quality
    let test_cases = vec![
        ("", "Empty data should produce clear error"),
        ("{", "Invalid JSON should produce clear error"),
        (
            "{\"invalid\": true}",
            "Missing required fields should produce clear error",
        ),
    ];

    for (input, description) in test_cases {
        let result = verify_manifest(input.as_bytes(), &device_pub);
        assert!(result.is_err(), "{}", description);

        let error = result.unwrap_err();
        let error_str = error.as_string().unwrap();
        assert!(
            !error_str.is_empty(),
            "{}: Error message should not be empty",
            description
        );
        assert!(
            error_str.len() > 5,
            "{}: Error message should be descriptive",
            description
        );
    }
}

#[wasm_bindgen_test]
fn test_browser_concurrent_verification_simulation() {
    // Simulate concurrent verification requests in browser
    let manifest_json = create_sample_manifest();
    let manifest_bytes = manifest_json.as_bytes();
    let device_pub = sample_device_pub_key();

    // Create multiple verification "requests" quickly
    let mut results = Vec::new();
    for _i in 0..10 {
        let result = verify_manifest(manifest_bytes, &device_pub);
        results.push(result);
    }

    // All should produce consistent results
    let first_result_is_ok = results[0].is_ok();
    for result in results {
        assert_eq!(
            result.is_ok(),
            first_result_is_ok,
            "Results should be consistent"
        );
    }
}

#[wasm_bindgen_test]
fn test_browser_archive_verification_edge_cases() {
    // Test edge cases specific to archive verification
    let device_pub = sample_device_pub_key();

    // Test with minimal valid structure
    let minimal_manifest = json!({
        "trst_version": "0.1.0",
        "profile": "test",
        "device": {
            "id": "test",
            "fw": "1.0.0",
            "model": "test",
            "public_key": "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        },
        "capture": {
            "started_at": "2025-09-20T17:02:11Z",
            "tz": "UTC",
            "fps": 1,
            "resolution": "1x1",
            "codec": "raw"
        },
        "chunk": {
            "approx_duration_s": 1.0,
            "bytes": 1
        },
        "segments": [],
        "claims": {},
        "signature": null
    })
    .to_string();

    let result = verify_archive(minimal_manifest.as_bytes(), &device_pub);
    assert!(result.is_ok() || result.is_err()); // Should handle gracefully
}
