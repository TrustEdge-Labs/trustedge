//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Integration tests for the consolidated verify module.
//!
//! Migrated from trustedge-verify-core/tests/integration_tests.rs.
//!
//! Test groupings:
//! - HTTP endpoint tests (feature = "http"): health, JWKS
//! - Pure crypto tests (always available): happy path, tampered, wrong key, empty, key manager

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signer, SigningKey};
use serde_json::json;
use trustedge_platform::verify::engine::{verify_to_report, SegmentDigest};
use trustedge_platform::verify::jwks::KeyManager;

// ---------------------------------------------------------------------------
// HTTP endpoint tests (require `http` feature)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "http", not(feature = "postgres")))]
mod http_tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;
    use trustedge_platform::http::{create_router, AppState};

    /// Build a test app using the real consolidated router (no postgres, stateless mode).
    async fn create_test_app() -> axum::Router {
        let key_manager = KeyManager::new().unwrap();
        let state = AppState {
            keys: Arc::new(RwLock::new(key_manager)),
        };
        create_router(state)
    }

    #[tokio::test]
    async fn test_health_endpoint() -> Result<()> {
        let app = create_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["status"], "OK");
        assert!(body_json.get("version").is_some());
        assert!(body_json.get("timestamp").is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_jwks_endpoint() -> Result<()> {
        let app = create_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/.well-known/jwks.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let jwks: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(jwks.get("keys").is_some());
        let keys = jwks["keys"].as_array().unwrap();
        assert!(!keys.is_empty());

        let key = &keys[0];
        assert_eq!(key["kty"], "OKP");
        assert_eq!(key["crv"], "Ed25519");
        assert_eq!(key["alg"], "EdDSA");
        assert!(key.get("kid").is_some());
        assert!(key.get("x").is_some());

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Pure crypto tests (no feature gates required)
// ---------------------------------------------------------------------------

#[test]
fn test_happy_path_verification() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let manifest = json!({
        "version": "1.0",
        "segments": 2,
        "device_id": "test_device"
    });

    let manifest_bytes = serde_json::to_string(&manifest)?.into_bytes();
    let signature = signing_key.sign(&manifest_bytes);

    let mut signed_manifest = manifest.clone();
    signed_manifest["signature"] = json!(BASE64.encode(signature.to_bytes()));

    let segments = vec![
        SegmentDigest {
            index: 0,
            hash: "b3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=".to_string(),
        },
        SegmentDigest {
            index: 1,
            hash: "b3:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb=".to_string(),
        },
    ];

    let device_pub = format!("ed25519:{}", BASE64.encode(verifying_key.as_bytes()));

    let report = verify_to_report(&signed_manifest, &segments, &device_pub)?;

    assert!(report.signature_verification.passed);
    assert!(report.continuity_verification.passed);
    assert_eq!(report.metadata.total_segments, 2);
    assert_eq!(report.metadata.verified_segments, 2);

    Ok(())
}

#[test]
fn test_tampered_segment_verification() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let manifest = json!({
        "version": "1.0",
        "segments": 2,
        "device_id": "test_device"
    });

    let manifest_bytes = serde_json::to_string(&manifest)?.into_bytes();
    let signature = signing_key.sign(&manifest_bytes);

    let mut signed_manifest = manifest.clone();
    signed_manifest["signature"] = json!(BASE64.encode(signature.to_bytes()));

    let segments = vec![
        SegmentDigest {
            index: 0,
            hash: "b3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=".to_string(),
        },
        SegmentDigest {
            index: 2, // Skipped index 1 — continuity failure
            hash: "b3:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb=".to_string(),
        },
    ];

    let device_pub = format!("ed25519:{}", BASE64.encode(verifying_key.as_bytes()));

    let report = verify_to_report(&signed_manifest, &segments, &device_pub)?;

    assert!(report.signature_verification.passed);
    assert!(!report.continuity_verification.passed); // Should fail due to missing segment
    assert!(report.continuity_verification.error.is_some());

    Ok(())
}

#[test]
fn test_wrong_key_verification() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let wrong_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let wrong_verifying_key = wrong_key.verifying_key();

    let manifest = json!({
        "version": "1.0",
        "segments": 1,
        "device_id": "test_device"
    });

    let manifest_bytes = serde_json::to_string(&manifest)?.into_bytes();
    let signature = signing_key.sign(&manifest_bytes);

    let mut signed_manifest = manifest.clone();
    signed_manifest["signature"] = json!(BASE64.encode(signature.to_bytes()));

    let segments = vec![SegmentDigest {
        index: 0,
        hash: "b3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=".to_string(),
    }];

    let wrong_device_pub = format!("ed25519:{}", BASE64.encode(wrong_verifying_key.as_bytes()));

    let report = verify_to_report(&signed_manifest, &segments, &wrong_device_pub)?;

    assert!(!report.signature_verification.passed); // Should fail with wrong key
    assert!(report.signature_verification.error.is_some());
    assert!(report.continuity_verification.passed); // Continuity should still pass

    Ok(())
}

#[test]
fn test_empty_segments_verification() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let manifest = json!({
        "version": "1.0",
        "segments": 0,
        "device_id": "test_device"
    });

    let manifest_bytes = serde_json::to_string(&manifest)?.into_bytes();
    let signature = signing_key.sign(&manifest_bytes);

    let mut signed_manifest = manifest.clone();
    signed_manifest["signature"] = json!(BASE64.encode(signature.to_bytes()));

    let segments = vec![];

    let device_pub = format!("ed25519:{}", BASE64.encode(verifying_key.as_bytes()));

    let report = verify_to_report(&signed_manifest, &segments, &device_pub)?;

    assert!(report.signature_verification.passed);
    assert!(report.continuity_verification.passed);
    assert_eq!(report.metadata.total_segments, 0);
    assert_eq!(report.metadata.verified_segments, 0);

    Ok(())
}

#[test]
fn test_key_manager_creation_and_jwks() -> Result<()> {
    let key_manager = KeyManager::new()?;

    let kid = key_manager.current_kid();
    assert!(!kid.is_empty());
    assert!(kid.starts_with("key_"));

    let jwks = key_manager.to_jwks();
    let keys = jwks["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 1);

    let key = &keys[0];
    assert_eq!(key["kty"], "OKP");
    assert_eq!(key["crv"], "Ed25519");
    assert_eq!(key["alg"], "EdDSA");
    assert_eq!(key["kid"], kid);

    Ok(())
}
