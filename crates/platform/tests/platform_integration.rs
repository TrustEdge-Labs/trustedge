//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Integration tests for the consolidated platform HTTP layer and database.
//!
//! Migrated from trustedge-platform-api/platform-api/tests/integration_test.rs.
//!
//! ALL tests are marked `#[ignore]` — they require a running PostgreSQL instance.
//! Run with: `cargo test -p trustedge-platform --test platform_integration
//!            --features "http,postgres,test-utils" -- --include-ignored`
//!
//! Environment variable: TEST_DATABASE_URL (default: postgres://postgres:password@localhost:5432/trustedge_test)
//!
//! Behavioral changes from original platform-api tests:
//! - test_jwks_proxy: original expected 502 (proxy to verify-core). Now expects 200 (local JWKS).
//! - test_verify_valid_payload: original expected 502 (proxy to verify-core). Now expects 400
//!   (inline validation catches invalid segment hash format before verification).

#![cfg(all(feature = "http", feature = "postgres"))]

use axum_test::TestServer;
use serde_json::json;
use sqlx::PgPool;
use trustedge_platform::{
    database::{create_api_key, create_connection_pool, create_organization, run_migrations},
    http::{auth::generate_token, auth::hash_token_for_storage, handlers::create_test_app},
};
use uuid::Uuid;

async fn setup_test_db() -> (PgPool, Uuid, String) {
    let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:password@localhost:5432/trustedge_test".to_string()
    });

    let pool = create_connection_pool(&database_url).await.unwrap();

    run_migrations(&pool).await.unwrap();

    let org_id = create_organization(&pool, "Test Org", "enterprise")
        .await
        .unwrap();

    let token = generate_token();
    let token_hash = hash_token_for_storage(&token);
    create_api_key(&pool, org_id, &token_hash).await.unwrap();

    (pool, org_id, token)
}

#[tokio::test]
#[ignore]
async fn test_auth_middleware() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    // Without token — should return 401
    let response = server
        .post("/v1/devices")
        .json(&json!({
            "device_id": "test-device",
            "device_pub": "test-pubkey",
            "label": "Test Device"
        }))
        .await;

    assert_eq!(response.status_code(), 401);

    // With valid token — should return 200
    let response = server
        .post("/v1/devices")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_id": "test-device",
            "device_pub": "test-pubkey",
            "label": "Test Device"
        }))
        .await;

    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
#[ignore]
async fn test_device_registration() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/devices")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_id": "test-device-001",
            "device_pub": "test-pubkey-data",
            "label": "Test Device"
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["device_id"], "test-device-001");
    assert_eq!(body["device_pub"], "test-pubkey-data");
    assert_eq!(body["label"], "Test Device");
    assert_eq!(body["status"], "active");
}

#[tokio::test]
#[ignore]
async fn test_org_isolation() {
    let (pool, _org1_id, token1) = setup_test_db().await;

    let org2_id = create_organization(&pool, "Test Org 2", "free")
        .await
        .unwrap();
    let token2 = generate_token();
    let token2_hash = hash_token_for_storage(&token2);
    create_api_key(&pool, org2_id, &token2_hash).await.unwrap();

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response1 = server
        .post("/v1/devices")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token1).parse().unwrap(),
        )
        .json(&json!({
            "device_id": "org1-device",
            "device_pub": "org1-pubkey",
            "label": "Org 1 Device"
        }))
        .await;

    assert_eq!(response1.status_code(), 200);

    let response2 = server
        .post("/v1/devices")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token2).parse().unwrap(),
        )
        .json(&json!({
            "device_id": "org2-device",
            "device_pub": "org2-pubkey",
            "label": "Org 2 Device"
        }))
        .await;

    assert_eq!(response2.status_code(), 200);
}

/// Tests that JWKS endpoint returns local keys (not a 502 proxy error).
///
/// Behavioral change from original platform-api: previously proxied to verify-core (502 when
/// mock server not running). Now served from local KeyManager → 200 with valid JWKS structure.
#[tokio::test]
#[ignore]
async fn test_jwks_endpoint_returns_local_keys() {
    let (pool, _org_id, _token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/.well-known/jwks.json").await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body.get("keys").is_some());
    let keys = body["keys"].as_array().unwrap();
    assert!(!keys.is_empty());
    assert_eq!(keys[0]["kty"], "OKP");
    assert_eq!(keys[0]["crv"], "Ed25519");
    assert_eq!(keys[0]["alg"], "EdDSA");
}

#[tokio::test]
#[ignore]
async fn test_verify_invalid_payload_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "",
            "manifest": "test manifest",
            "segments": []
        }))
        .await;

    assert_eq!(response.status_code(), 400);
    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "invalid_segments");
}

#[tokio::test]
#[ignore]
async fn test_verify_empty_device_pub_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "",
            "manifest": "test manifest",
            "segments": [{"index": 0, "hash": "a".repeat(64)}]
        }))
        .await;

    assert_eq!(response.status_code(), 400);
    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "invalid_device_pub");
}

#[tokio::test]
#[ignore]
async fn test_verify_empty_manifest_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "ed25519:test",
            "manifest": "",
            "segments": [{"index": 0, "hash": "a".repeat(64)}]
        }))
        .await;

    assert_eq!(response.status_code(), 400);
    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "invalid_manifest");
}

#[tokio::test]
#[ignore]
async fn test_verify_invalid_segments_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "ed25519:test",
            "manifest": "test manifest",
            "segments": [{"index": 1, "hash": "invalid"}]
        }))
        .await;

    assert_eq!(response.status_code(), 400);
    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "invalid_segments");
}

/// Tests that a valid payload structure returns 400 due to invalid segment hash format.
///
/// Behavioral change from original platform-api: previously forwarded to verify-core via HTTP
/// (returning 502 when mock server not running). Now performs inline validation and verification.
/// The segment hash "a"*64 lacks the required "b3:" prefix, so validation fails with
/// `invalid_segments` before reaching the cryptographic verification step.
#[tokio::test]
#[ignore]
async fn test_verify_valid_payload_inline_verification() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "ed25519:test",
            "manifest": "test manifest",
            "segments": [{"index": 0, "hash": "a".repeat(64)}]
        }))
        .await;

    // Inline validation rejects the segment hash format (no "b3:" prefix)
    assert_eq!(response.status_code(), 400);
    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "invalid_segments");
}

#[tokio::test]
#[ignore]
async fn test_verify_malformed_json_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .add_header(
            "Content-Type".parse().unwrap(),
            "application/json".parse().unwrap(),
        )
        .text("{invalid json")
        .await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[ignore]
async fn test_verify_unknown_fields_returns_400() {
    let (pool, _org_id, token) = setup_test_db().await;

    let app = create_test_app(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/v1/verify")
        .add_header(
            "Authorization".parse().unwrap(),
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "device_pub": "ed25519:test",
            "manifest": "test manifest",
            "segments": [{"index": 0, "hash": "a".repeat(64)}],
            "unknown_field": "should_cause_error"
        }))
        .await;

    assert_eq!(response.status_code(), 400);
}
