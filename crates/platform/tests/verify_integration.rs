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
//! - HTTP endpoint tests (feature = "http"): health, JWKS, CORS parity, verify round-trip
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
    use axum::{body::Body, http::header::CONTENT_TYPE, http::Request};
    use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL;
    use ed25519_dalek::{Signer, VerifyingKey as DalekVerifyingKey};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;
    use trustedge_platform::http::{create_router, AppState};

    /// Build an independent test app state using the real consolidated router (no postgres).
    fn make_state() -> AppState {
        let key_manager = KeyManager::new().unwrap();
        AppState {
            keys: Arc::new(RwLock::new(key_manager)),
        }
    }

    /// Build a test app using the real consolidated router (no postgres, stateless mode).
    async fn create_test_app() -> axum::Router {
        create_router(make_state())
    }

    // -----------------------------------------------------------------------
    // Helper: build a signed VerifyRequest JSON body
    // -----------------------------------------------------------------------

    /// Build a canonically signed manifest and return (signed_manifest, device_pub_string).
    fn build_signed_manifest(
        signing_key: &ed25519_dalek::SigningKey,
    ) -> (serde_json::Value, String) {
        let manifest = json!({
            "version": "1.0",
            "segments": 1,
            "device_id": "test-device"
        });

        let manifest_bytes = serde_json::to_string(&manifest).unwrap().into_bytes();
        let signature = signing_key.sign(&manifest_bytes);
        let verifying_key = signing_key.verifying_key();

        let mut signed_manifest = manifest.clone();
        signed_manifest["signature"] = json!(BASE64.encode(signature.to_bytes()));

        let device_pub = format!("ed25519:{}", BASE64.encode(verifying_key.as_bytes()));

        (signed_manifest, device_pub)
    }

    /// Build a valid VerifyRequest body as JSON bytes.
    fn build_verify_body(
        signed_manifest: &serde_json::Value,
        device_pub: &str,
        return_receipt: bool,
    ) -> Vec<u8> {
        let body = json!({
            "device_pub": device_pub,
            "manifest": signed_manifest,
            "segments": [
                {
                    "index": 0,
                    "hash": "b3:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                }
            ],
            "options": {
                "return_receipt": return_receipt,
                "device_id": "test-device"
            }
        });
        serde_json::to_vec(&body).unwrap()
    }

    // -----------------------------------------------------------------------
    // Test 1: Health endpoint
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Test 2: JWKS endpoint
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Test 3: CORS parity — two independently constructed routers produce
    //         identical CORS headers for the same OPTIONS preflight (TST-02).
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_cors_preflight_parity() -> Result<()> {
        // Build two independent routers from the same AppState (cloned).
        // Both go through build_base_router -> create_router, so their
        // CORS layer configuration must be identical.
        let state = make_state();
        let app1 = create_router(state.clone());
        let app2 = create_router(state);

        let make_preflight = || {
            Request::builder()
                .method("OPTIONS")
                .uri("/v1/verify")
                .header("Origin", "http://evil.example.com")
                .header("Access-Control-Request-Method", "POST")
                .body(Body::empty())
                .unwrap()
        };

        let resp1 = app1.oneshot(make_preflight()).await.unwrap();
        let resp2 = app2.oneshot(make_preflight()).await.unwrap();

        // Both routers must agree on all CORS response headers.
        assert_eq!(
            resp1.headers().get("access-control-allow-origin"),
            resp2.headers().get("access-control-allow-origin"),
            "CORS allow-origin headers must be identical between router instances"
        );
        assert_eq!(
            resp1.headers().get("access-control-allow-methods"),
            resp2.headers().get("access-control-allow-methods"),
            "CORS allow-methods headers must be identical between router instances"
        );
        assert_eq!(
            resp1.headers().get("access-control-allow-headers"),
            resp2.headers().get("access-control-allow-headers"),
            "CORS allow-headers headers must be identical between router instances"
        );

        // In verify-only builds, CorsLayer::new() denies all cross-origin requests —
        // no Access-Control-Allow-Origin header is present in either response.
        assert!(
            resp1.headers().get("access-control-allow-origin").is_none(),
            "verify-only build must deny cross-origin: Access-Control-Allow-Origin must be absent"
        );

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Test 4: Full verify round-trip — sign a payload, POST to /v1/verify,
    //         receive HTTP 200 with a JWS receipt (TST-03 happy path).
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_verify_round_trip() -> Result<()> {
        let app = create_test_app().await;

        let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
        let (signed_manifest, device_pub) = build_signed_manifest(&signing_key);
        let body_bytes = build_verify_body(&signed_manifest, &device_pub, true);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/verify")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(body_bytes))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            axum::http::StatusCode::OK,
            "verify endpoint must return 200 for a valid signed payload"
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(
            resp["result"]["signature_verification"]["passed"]
                .as_bool()
                .unwrap_or(false),
            "signature_verification must pass for a correctly signed manifest"
        );
        assert!(
            resp["result"]["continuity_verification"]["passed"]
                .as_bool()
                .unwrap_or(false),
            "continuity_verification must pass for sequential segments"
        );
        assert!(
            resp["receipt"].is_string(),
            "receipt must be present when return_receipt=true and verification passed"
        );
        assert!(
            resp["verification_id"]
                .as_str()
                .unwrap_or("")
                .starts_with("v_"),
            "verification_id must start with 'v_'"
        );

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Test 5: Receipt JWS verified against JWKS (TST-03 full receipt check).
    //
    // Steps:
    //   1. POST /v1/verify with return_receipt=true → get JWS receipt
    //   2. GET /.well-known/jwks.json on a second router (same state) → get public key
    //   3. Verify the JWS Ed25519 signature using the JWKS public key
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_verify_receipt_matches_jwks() -> Result<()> {
        // Shared state so both routers have the same signing key.
        let state = make_state();
        let app_verify = create_router(state.clone());
        let app_jwks = create_router(state);

        // Step 1: POST /v1/verify → receive JWS receipt
        let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
        let (signed_manifest, device_pub) = build_signed_manifest(&signing_key);
        let body_bytes = build_verify_body(&signed_manifest, &device_pub, true);

        let verify_resp = app_verify
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/verify")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(body_bytes))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(verify_resp.status(), axum::http::StatusCode::OK);
        let body = axum::body::to_bytes(verify_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let jws = resp["receipt"]
            .as_str()
            .expect("receipt must be a string in the response");

        // Step 2: GET /.well-known/jwks.json → extract Ed25519 public key
        let jwks_resp = app_jwks
            .oneshot(
                Request::builder()
                    .uri("/.well-known/jwks.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(jwks_resp.status(), axum::http::StatusCode::OK);
        let jwks_body = axum::body::to_bytes(jwks_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let jwks: serde_json::Value = serde_json::from_slice(&jwks_body).unwrap();

        let first_key = &jwks["keys"][0];
        assert_eq!(
            first_key["alg"], "EdDSA",
            "JWKS key algorithm must be EdDSA"
        );

        // The JWKS 'x' field is base64-standard encoded (RFC 4648 with padding)
        let x_b64 = first_key["x"].as_str().expect("JWKS key must have 'x'");
        let pub_key_bytes: Vec<u8> = BASE64.decode(x_b64)?;
        assert_eq!(
            pub_key_bytes.len(),
            32,
            "Ed25519 public key must be 32 bytes"
        );

        let pub_key_arr: [u8; 32] = pub_key_bytes
            .try_into()
            .expect("Ed25519 public key must be 32 bytes");
        let verifying_key = DalekVerifyingKey::from_bytes(&pub_key_arr)?;

        // Step 3: Decode the JWS and verify the Ed25519 signature
        // JWS format: base64url(header).base64url(payload).base64url(signature)
        let parts: Vec<&str> = jws.split('.').collect();
        assert_eq!(parts.len(), 3, "JWS must have exactly 3 parts");

        // Parse header and verify alg
        let header_bytes = BASE64URL.decode(parts[0])?;
        let header: serde_json::Value = serde_json::from_slice(&header_bytes)?;
        assert_eq!(
            header["alg"], "EdDSA",
            "JWS header must specify EdDSA algorithm"
        );
        assert!(header.get("kid").is_some(), "JWS header must contain a kid");

        // Parse payload and check receipt fields
        let payload_bytes = BASE64URL.decode(parts[1])?;
        let payload: serde_json::Value = serde_json::from_slice(&payload_bytes)?;

        // The receipt is nested under the "receipt" field in JwsPayload
        let receipt_payload = &payload["receipt"];
        assert!(
            receipt_payload.get("device_id").is_some(),
            "JWS payload receipt must contain device_id"
        );
        assert!(
            receipt_payload.get("manifest_digest").is_some(),
            "JWS payload receipt must contain manifest_digest"
        );
        assert!(
            receipt_payload.get("chain_tip").is_some(),
            "JWS payload receipt must contain chain_tip"
        );

        // Verify Ed25519 signature: signing input = bytes of "header_b64.payload_b64"
        let signing_input = format!("{}.{}", parts[0], parts[1]);
        let sig_bytes = BASE64URL.decode(parts[2])?;
        let sig_arr: [u8; 64] = sig_bytes
            .try_into()
            .expect("Ed25519 signature must be 64 bytes");
        let signature = ed25519_dalek::Signature::from_bytes(&sig_arr);

        verifying_key
            .verify_strict(signing_input.as_bytes(), &signature)
            .expect("JWS Ed25519 signature must verify against JWKS public key");

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Test 6: Wrong key returns HTTP 200 with signature_verification.passed=false
    //         and receipt=None (negative case).
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_verify_wrong_key_returns_failed_signature() -> Result<()> {
        let app = create_test_app().await;

        // Sign with one key, but present a DIFFERENT key as device_pub
        let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
        let wrong_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);

        let (signed_manifest, _correct_pub) = build_signed_manifest(&signing_key);
        let wrong_device_pub = format!(
            "ed25519:{}",
            BASE64.encode(wrong_key.verifying_key().as_bytes())
        );

        let body_bytes = build_verify_body(&signed_manifest, &wrong_device_pub, true);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/verify")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(body_bytes))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Server returns 200 (verification completed), not an error status
        assert_eq!(
            response.status(),
            axum::http::StatusCode::OK,
            "wrong-key verification must return HTTP 200, not an error status"
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(
            !resp["result"]["signature_verification"]["passed"]
                .as_bool()
                .unwrap_or(true),
            "signature_verification must FAIL when device_pub does not match signing key"
        );
        assert!(
            resp["receipt"].is_null(),
            "receipt must be null when signature verification fails"
        );

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
