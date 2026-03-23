//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Axum router composition for the TrustEdge Platform HTTP layer.
//!
//! Routes:
//!   POST  /v1/verify              — verify archive (always available)
//!   POST  /v1/devices             — register device (postgres only)
//!   GET   /v1/receipts/:id        — get receipt (postgres only)
//!   GET   /.well-known/jwks.json  — local JWKS (no proxy)
//!   GET   /healthz                — health check

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, trace::TraceLayer};

use super::{
    handlers::{health_handler, jwks_handler, verify_handler},
    rate_limit::{rate_limit_middleware, RateLimitState},
    state::AppState,
};

/// Build the base router with routes shared across all feature configurations.
///
/// The `/v1/verify` route is NOT included here — it is added in `create_router`
/// with rate limiting applied. All other routes (healthz, jwks) are public and
/// unthrottled.
///
/// Both `create_router` and `create_test_app` ultimately call this function,
/// ensuring a single source of truth for the route set (TST-02 parity).
pub fn build_base_router() -> Router<AppState> {
    Router::new()
        .route("/.well-known/jwks.json", get(jwks_handler))
        .route("/healthz", get(health_handler))
}

/// Compose the full Axum router for the TrustEdge Platform service.
///
/// Applies:
/// - `RequestBodyLimitLayer` (2 MB) on all routes to prevent body-flood DoS.
/// - Per-IP rate limiting on `/v1/verify` only (configurable via `RATE_LIMIT_RPS`,
///   default 10 req/sec) to protect the CPU-intensive verify endpoint.
///
/// When the `postgres` feature is enabled, the router includes device and
/// receipt endpoints protected by the Bearer token auth middleware.
pub fn create_router(state: AppState) -> Router {
    // Read rate limit RPS from environment, default to 10.
    let rps = std::env::var("RATE_LIMIT_RPS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);
    let rl_state = RateLimitState::new(rps);

    // Rate-limited verify sub-router — only /v1/verify is throttled.
    let verify_router = Router::new()
        .route("/v1/verify", post(verify_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            rl_state,
            rate_limit_middleware,
        ));

    let base = build_base_router().merge(verify_router);

    #[cfg(feature = "postgres")]
    let base = {
        use super::auth::auth_middleware;
        use super::handlers::{get_receipt_handler, register_device_handler};
        use axum::middleware;

        // Dashboard dev origins — restrict to Content-Type, Authorization, Accept
        let cors = CorsLayer::new()
            .allow_origin([
                "http://localhost:3000".parse().expect("valid origin"),
                "http://localhost:8080".parse().expect("valid origin"),
            ])
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
            ]);

        // Auth-protected routes (devices, receipts)
        let protected = Router::new()
            .route("/v1/devices", post(register_device_handler))
            .route("/v1/receipts/:id", get(get_receipt_handler))
            .layer(middleware::from_fn_with_state(
                state.db_pool.clone(),
                auth_middleware,
            ));

        // Merge: base routes (healthz, verify, jwks) are public, protected routes need auth
        base.merge(protected)
            .with_state(state)
            .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
            .layer(cors)
            .layer(TraceLayer::new_for_http())
    };

    #[cfg(not(feature = "postgres"))]
    let base = base
        .with_state(state)
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
        // Same-origin only — no cross-origin requests allowed for verify-only builds
        .layer(CorsLayer::new())
        .layer(TraceLayer::new_for_http());

    base
}
