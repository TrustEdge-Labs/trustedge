//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Integration tests for platform-server startup wiring.
//!
//! Validates: Config loading from environment, AppState construction, and
//! router health check response. These tests target verify-only mode
//! (no postgres) — run with `--no-default-features`.
//!
//! NOTE: Tests that manipulate environment variables (PORT) share the same
//! process address space. They use a per-process Mutex to run serially and
//! avoid races when setting/clearing PORT.

use axum::{body::Body, http::Request};
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::RwLock;
use tower::ServiceExt;
use trustedge_platform::http::{create_router, AppState, Config};
use trustedge_platform::verify::jwks::KeyManager;

/// Global lock for tests that mutate environment variables.
///
/// Rust integration tests that share a binary run on separate threads.
/// Holding this lock while reading/writing env vars prevents races between
/// `test_config_from_env_*` tests.
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

// ---------------------------------------------------------------------------
// Config::from_env tests
// ---------------------------------------------------------------------------

/// Config::from_env() returns sensible defaults when no env vars are set.
///
/// In verify-only mode (no postgres feature) all env vars are optional:
/// - PORT defaults to 3001
/// - JWT_AUDIENCE defaults to "trustedge-platform"
#[tokio::test]
async fn test_config_from_env_defaults() {
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());

    std::env::remove_var("PORT");
    std::env::remove_var("JWT_AUDIENCE");

    let config = Config::from_env().expect("Config::from_env() should succeed with no env vars");

    assert_eq!(config.port, 3001, "default port should be 3001");
    assert_eq!(
        config.jwt_audience, "trustedge-platform",
        "default jwt_audience should be 'trustedge-platform'"
    );
}

/// Config::from_env() reads PORT from the environment.
///
/// When PORT=9999 is set, the resulting config should use port 9999.
/// The env var is restored after the test to avoid affecting other tests.
#[tokio::test]
async fn test_config_from_env_custom_port() {
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());

    std::env::set_var("PORT", "9999");

    let config = Config::from_env().expect("Config::from_env() should succeed with PORT=9999 set");

    std::env::remove_var("PORT");

    assert_eq!(
        config.port, 9999,
        "port should match the PORT env var value"
    );
}

/// Config::from_env() falls back to port 3001 when PORT is not a valid number.
///
/// The implementation uses `.parse().unwrap_or(3001)`, so an invalid value
/// (e.g. "not_a_number") silently falls back to the default port rather than
/// returning an error. This is the most likely misconfiguration path in
/// verify-only mode since there are no required env vars.
#[tokio::test]
async fn test_config_from_env_invalid_port_uses_default() {
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());

    std::env::set_var("PORT", "not_a_number");

    let config = Config::from_env()
        .expect("Config::from_env() should succeed even with an unparseable PORT value");

    std::env::remove_var("PORT");

    assert_eq!(
        config.port, 3001,
        "invalid PORT value should fall back to default port 3001"
    );
}

// ---------------------------------------------------------------------------
// AppState + router tests
// ---------------------------------------------------------------------------

/// AppState constructs successfully and the router responds to GET /healthz.
///
/// Verifies the full wiring path: KeyManager::new() → AppState → create_router()
/// → oneshot request → HTTP 200 with `{"status":"OK","version":...,"timestamp":...}`.
#[tokio::test]
async fn test_appstate_construction_and_router_health() {
    let key_manager = KeyManager::new().expect("KeyManager::new() should succeed");
    let state = AppState {
        keys: Arc::new(RwLock::new(key_manager)),
    };

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .expect("request builder should not fail"),
        )
        .await
        .expect("oneshot should not fail");

    assert_eq!(
        response.status(),
        axum::http::StatusCode::OK,
        "GET /healthz should return HTTP 200"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");
    let body_json: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid JSON");

    assert_eq!(
        body_json["status"], "OK",
        "health response body should contain status:OK"
    );
    assert!(
        body_json.get("version").is_some(),
        "health response should include a version field"
    );
    assert!(
        body_json.get("timestamp").is_some(),
        "health response should include a timestamp field"
    );
}

/// POST /v1/verify rejects an empty request body with HTTP 422 Unprocessable Entity.
///
/// VerifyRequest requires device_pub, manifest, and segments fields (deny_unknown_fields).
/// Sending an empty JSON object `{}` causes Axum's JSON extractor to fail
/// deserialization, which returns 422 Unprocessable Entity (Axum's standard
/// response for JSON extraction failures).
#[tokio::test]
async fn test_router_verify_rejects_empty_body() {
    let key_manager = KeyManager::new().expect("KeyManager::new() should succeed");
    let state = AppState {
        keys: Arc::new(RwLock::new(key_manager)),
    };

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/verify")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .expect("request builder should not fail"),
        )
        .await
        .expect("oneshot should not fail");

    // Axum returns 422 Unprocessable Entity when JSON deserialization fails
    // (missing required fields in VerifyRequest).
    assert_eq!(
        response.status(),
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        "POST /v1/verify with empty body should return HTTP 422"
    );
}
