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
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use super::{
    handlers::{health_handler, jwks_handler, verify_handler},
    state::AppState,
};

/// Compose the full Axum router for the TrustEdge Platform service.
///
/// When the `postgres` feature is enabled, the router includes device and
/// receipt endpoints protected by the Bearer token auth middleware.
pub fn create_router(state: AppState) -> Router {
    let base = Router::new()
        .route("/v1/verify", post(verify_handler))
        .route("/.well-known/jwks.json", get(jwks_handler))
        .route("/healthz", get(health_handler));

    #[cfg(feature = "postgres")]
    let base = {
        use super::auth::auth_middleware;
        use super::handlers::{get_receipt_handler, register_device_handler};
        use axum::middleware;

        let cors = CorsLayer::new()
            .allow_origin([
                "http://localhost:3000".parse().expect("valid origin"),
                "http://localhost:8080".parse().expect("valid origin"),
            ])
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers(tower_http::cors::Any);

        base.route("/v1/devices", post(register_device_handler))
            .route("/v1/receipts/:id", get(get_receipt_handler))
            .layer(middleware::from_fn_with_state(
                state.db_pool.clone(),
                auth_middleware,
            ))
            .with_state(state)
            .layer(cors)
            .layer(TraceLayer::new_for_http())
    };

    #[cfg(not(feature = "postgres"))]
    let base = base
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    base
}
