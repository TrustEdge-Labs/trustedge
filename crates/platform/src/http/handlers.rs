//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! HTTP endpoint handlers for the TrustEdge Platform service.
//!
//! The key consolidation change: `verify_handler` now calls
//! `crate::verify::engine::verify_to_report()` directly instead of forwarding
//! to a separate verify-core service via HTTP.

use axum::{extract::State, http::StatusCode, response::Json};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use serde_json::Value;
use tracing::{info, warn};

use crate::verify::{
    engine::{receipt_from_report, verify_to_report},
    signing::sign_receipt_jws,
    types::{HealthResponse, VerifyRequest, VerifyResponse},
    validation::{validate_segment_hashes, ValidationError},
};

use super::state::AppState;

// ---------------------------------------------------------------------------
// Always-available handlers (no postgres required)
// ---------------------------------------------------------------------------

/// GET /.well-known/jwks.json — returns the local KeyManager's JWKS.
///
/// Serves keys from the local KeyManager. No proxy to an external service.
pub async fn jwks_handler(State(state): State<AppState>) -> Json<Value> {
    let keys = state.keys.read().await;
    Json(keys.to_jwks())
}

/// GET /healthz — returns service health status.
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

/// POST /v1/verify — inline verification (stateless, no DB storage).
///
/// Validates the request, calls `verify_to_report()` directly, and optionally
/// signs a JWS receipt. This handler does not require the `postgres` feature.
///
/// When the `postgres` feature is enabled, use `verify_handler` instead for
/// full multi-tenant operation with DB audit trail.
#[cfg(not(feature = "postgres"))]
pub async fn verify_handler(
    State(state): State<AppState>,
    Json(request): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, Json<ValidationError>)> {
    info!(
        "Processing verification request for device: {}",
        request.device_pub
    );

    // Ordered validation: empty segments → device_pub → manifest → hash format.
    // This order ensures the most specific error is returned first.
    if request.segments.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_segments",
                "segments array cannot be empty",
            )),
        ));
    }

    if request.device_pub.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_device_pub",
                "device_pub cannot be empty",
            )),
        ));
    }

    if request.manifest.is_null()
        || request.manifest == serde_json::Value::Object(Default::default())
        || request.manifest.as_str() == Some("")
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_manifest",
                "manifest cannot be empty",
            )),
        ));
    }

    if let Err(validation_error) = validate_segment_hashes(&request.segments) {
        warn!("Validation failed: {}", validation_error.detail);
        return Err((StatusCode::BAD_REQUEST, Json(validation_error)));
    }

    let report = match verify_to_report(&request.manifest, &request.segments, &request.device_pub) {
        Ok(report) => report,
        Err(e) => {
            warn!("Verification failed: {}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ValidationError::new(
                    "verification_failed",
                    &format!("Cryptographic verification failed: {}", e),
                )),
            ));
        }
    };

    let verification_id = format!("v_{}", uuid::Uuid::new_v4().simple());
    let mut receipt = None;

    if let Some(options) = &request.options {
        if options.return_receipt.unwrap_or(false)
            && report.signature_verification.passed
            && report.continuity_verification.passed
        {
            let device_id = options.device_id.as_deref().unwrap_or("unknown_device");
            let manifest_digest = compute_manifest_digest_blake3(&request.manifest);
            let now_rfc3339 = Utc::now().to_rfc3339();

            let keys = state.keys.read().await;
            let kid = keys.current_kid();

            let receipt_obj = receipt_from_report(
                &report,
                &manifest_digest,
                device_id,
                &kid,
                &now_rfc3339,
                &report.metadata.chain_tip,
            );

            match sign_receipt_jws(&receipt_obj, &keys).await {
                Ok(jws) => receipt = Some(jws),
                Err(e) => {
                    warn!("Failed to sign receipt: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ValidationError::new(
                            "receipt_signing_failed",
                            &format!("Failed to sign receipt: {}", e),
                        )),
                    ));
                }
            }
        }
    }

    Ok(Json(VerifyResponse {
        verification_id,
        result: report,
        receipt,
    }))
}

// ---------------------------------------------------------------------------
// Postgres-gated handlers (full multi-tenant with DB audit trail)
// ---------------------------------------------------------------------------

/// POST /v1/verify — inline verification with DB audit trail.
///
/// Consolidation change: calls `verify_to_report()` directly instead of
/// forwarding to a separate verify-core service via HTTP. Requires postgres.
#[cfg(feature = "postgres")]
pub async fn verify_handler(
    State(state): State<AppState>,
    axum::extract::Extension(org_ctx): axum::extract::Extension<crate::http::auth::OrgContext>,
    Json(request): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, Json<ValidationError>)> {
    info!(
        "Processing verification request for device: {}",
        request.device_pub
    );

    // Ordered validation: empty segments → device_pub → manifest → hash format.
    // This order ensures the most specific error is returned first.
    if request.segments.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_segments",
                "segments array cannot be empty",
            )),
        ));
    }

    if request.device_pub.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_device_pub",
                "device_pub cannot be empty",
            )),
        ));
    }

    if request.manifest.is_null()
        || request.manifest == serde_json::Value::Object(Default::default())
        || request.manifest.as_str() == Some("")
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationError::new(
                "invalid_manifest",
                "manifest cannot be empty",
            )),
        ));
    }

    if let Err(validation_error) = validate_segment_hashes(&request.segments) {
        warn!("Validation failed: {}", validation_error.detail);
        return Err((StatusCode::BAD_REQUEST, Json(validation_error)));
    }

    // Look up device record if device_id option was provided
    let device_id = if let Some(ref options) = request.options {
        if let Some(ref device_id_str) = options.device_id {
            crate::database::get_device(&state.db_pool, org_ctx.org_id, device_id_str)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ValidationError::new(
                            "database_error",
                            "Failed to query device",
                        )),
                    )
                })?
        } else {
            None
        }
    } else {
        None
    };

    // Inline verification — direct call, no HTTP forwarding
    let report = match verify_to_report(&request.manifest, &request.segments, &request.device_pub) {
        Ok(report) => report,
        Err(e) => {
            warn!("Verification failed: {}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ValidationError::new(
                    "verification_failed",
                    &format!("Cryptographic verification failed: {}", e),
                )),
            ));
        }
    };

    // SHA-256 manifest digest for DB storage (compatibility with existing schema)
    let manifest_digest_sha256 = compute_manifest_digest_sha256(&request.manifest);

    let result_for_db = serde_json::json!({
        "signature_verification": {
            "passed": report.signature_verification.passed,
            "error": report.signature_verification.error,
        },
        "continuity_verification": {
            "passed": report.continuity_verification.passed,
            "error": report.continuity_verification.error,
        },
        "metadata": {
            "total_segments": report.metadata.total_segments,
            "verified_segments": report.metadata.verified_segments,
            "chain_tip": report.metadata.chain_tip,
            "genesis_hash": report.metadata.genesis_hash,
        }
    });

    let verification_id_uuid = crate::database::create_verification(
        &state.db_pool,
        org_ctx.org_id,
        device_id,
        &manifest_digest_sha256,
        &result_for_db,
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ValidationError::new(
                "database_error",
                "Failed to create verification record",
            )),
        )
    })?;

    let verification_id = verification_id_uuid.to_string();
    let mut receipt = None;
    let mut receipt_id = None;

    if let Some(ref options) = request.options {
        if options.return_receipt.unwrap_or(false)
            && report.signature_verification.passed
            && report.continuity_verification.passed
        {
            let device_id_str = options.device_id.as_deref().unwrap_or("unknown_device");

            // BLAKE3 digest for receipt construction (per verify-service convention)
            let manifest_digest_blake3 = compute_manifest_digest_blake3(&request.manifest);
            let now_rfc3339 = Utc::now().to_rfc3339();

            let keys = state.keys.read().await;
            let kid = keys.current_kid();

            let receipt_obj = receipt_from_report(
                &report,
                &manifest_digest_blake3,
                device_id_str,
                &kid,
                &now_rfc3339,
                &report.metadata.chain_tip,
            );

            match sign_receipt_jws(&receipt_obj, &keys).await {
                Ok(jws) => {
                    // Store receipt in DB
                    match crate::database::create_receipt(
                        &state.db_pool,
                        verification_id_uuid,
                        &jws,
                        &kid,
                    )
                    .await
                    {
                        Ok(rid) => {
                            receipt = Some(jws);
                            receipt_id = Some(rid.to_string());
                        }
                        Err(_) => {
                            warn!("Failed to store receipt in database");
                            // Non-fatal: return receipt without storing
                            receipt = Some(jws);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to sign receipt: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ValidationError::new(
                            "receipt_signing_failed",
                            &format!("Failed to sign receipt: {}", e),
                        )),
                    ));
                }
            }
        }
    }

    // Build response — include receipt_id in verification_id field for DB-backed mode
    let response_id = receipt_id
        .map(|rid| format!("{}/{}", verification_id, rid))
        .unwrap_or(verification_id);

    Ok(Json(VerifyResponse {
        verification_id: response_id,
        result: report,
        receipt,
    }))
}

/// POST /v1/devices — register a device for an organization.
#[cfg(feature = "postgres")]
pub async fn register_device_handler(
    State(state): State<AppState>,
    axum::extract::Extension(org_ctx): axum::extract::Extension<crate::http::auth::OrgContext>,
    Json(req): Json<DeviceRequest>,
) -> Result<Json<DeviceResponse>, StatusCode> {
    let device_uuid = crate::database::create_device(
        &state.db_pool,
        org_ctx.org_id,
        &req.device_id,
        &req.device_pub,
        req.label.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DeviceResponse {
        id: device_uuid,
        device_id: req.device_id,
        device_pub: req.device_pub,
        label: req.label,
        status: "active".to_string(),
    }))
}

/// GET /v1/receipts/:id — retrieve a verification receipt by ID.
#[cfg(feature = "postgres")]
pub async fn get_receipt_handler(
    State(state): State<AppState>,
    axum::extract::Extension(org_ctx): axum::extract::Extension<crate::http::auth::OrgContext>,
    axum::extract::Path(receipt_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<ReceiptResponse>, StatusCode> {
    let (jws, kid) = crate::database::get_receipt(&state.db_pool, org_ctx.org_id, receipt_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let claims = parse_jws_claims(&jws).unwrap_or(Value::Null);

    Ok(Json(ReceiptResponse {
        id: receipt_id,
        jws,
        kid,
        claims,
    }))
}

// ---------------------------------------------------------------------------
// Request/response types (postgres-gated — DB-specific ops)
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceRequest {
    pub device_id: String,
    pub device_pub: String,
    pub label: Option<String>,
}

#[cfg(feature = "postgres")]
#[derive(Debug, serde::Serialize)]
pub struct DeviceResponse {
    pub id: uuid::Uuid,
    pub device_id: String,
    pub device_pub: String,
    pub label: Option<String>,
    pub status: String,
}

#[cfg(feature = "postgres")]
#[derive(Debug, serde::Serialize)]
pub struct ReceiptResponse {
    pub id: uuid::Uuid,
    pub jws: String,
    pub kid: String,
    pub claims: Value,
}

// ---------------------------------------------------------------------------
// Test utilities
// ---------------------------------------------------------------------------

#[cfg(all(any(test, feature = "test-utils"), feature = "postgres"))]
pub fn create_test_app(pool: sqlx::PgPool) -> axum::Router {
    use crate::http::auth::auth_middleware;

    let keys = std::sync::Arc::new(tokio::sync::RwLock::new(
        crate::verify::jwks::KeyManager::new().expect("KeyManager should initialize for test"),
    ));

    let app_state = AppState {
        db_pool: pool.clone(),
        keys,
    };

    axum::Router::new()
        .route("/v1/verify", axum::routing::post(verify_handler))
        .route("/v1/devices", axum::routing::post(register_device_handler))
        .route("/v1/receipts/:id", axum::routing::get(get_receipt_handler))
        .route("/.well-known/jwks.json", axum::routing::get(jwks_handler))
        .route("/healthz", axum::routing::get(health_handler))
        .layer(axum::middleware::from_fn_with_state(pool, auth_middleware))
        .with_state(app_state)
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Compute BLAKE3 manifest digest (for receipt construction).
fn compute_manifest_digest_blake3(manifest: &Value) -> String {
    let canonical = serde_json::to_string(manifest).unwrap_or_default();
    let hash = trustedge_core::chain::segment_hash(canonical.as_bytes());
    format!("b3:{}", BASE64.encode(hash))
}

/// Compute SHA-256 manifest digest (for DB storage — compatible with platform-api schema).
#[cfg(feature = "postgres")]
fn compute_manifest_digest_sha256(manifest: &Value) -> String {
    use sha2::{Digest, Sha256};
    let canonical = serde_json::to_string(manifest).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Parse JWS claims payload from a JWT string.
#[cfg(feature = "postgres")]
fn parse_jws_claims(jws: &str) -> Option<Value> {
    let parts: Vec<&str> = jws.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let payload = parts[1];
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .ok()?;
    serde_json::from_slice(&decoded).ok()
}
