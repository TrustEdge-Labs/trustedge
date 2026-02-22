//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! HTTP layer — Axum-based REST API for TrustEdge Platform.
//!
//! Provides:
//! - Unified router combining all endpoints
//! - Auth middleware for Bearer token validation
//! - Handlers: verify, register_device, get_receipt, jwks, health
//! - AppState and Config for service wiring

pub mod auth;
pub mod config;
pub mod handlers;
pub mod router;
pub mod state;

pub use config::Config;
pub use router::create_router;
pub use state::AppState;
