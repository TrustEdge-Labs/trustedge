//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Per-IP rate limiting middleware using `governor`.
//!
//! Applied only to the `/v1/verify` route — CPU-intensive BLAKE3+Ed25519
//! verification is the primary abuse target. Health and JWKS endpoints are
//! not rate-limited.

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::{
    net::{IpAddr, SocketAddr},
    num::NonZeroU32,
    sync::Arc,
};

/// Shared rate-limiter state keyed by client IP address.
#[derive(Clone)]
pub struct RateLimitState {
    pub limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
}

impl RateLimitState {
    /// Create a new rate limiter allowing `rps` requests per second per IP.
    ///
    /// Governor defaults burst capacity to the quota replenishment rate,
    /// which means a client can burst up to `rps` requests before being throttled.
    pub fn new(rps: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(rps).expect("rps must be greater than zero"));
        let limiter = Arc::new(RateLimiter::keyed(quota));
        Self { limiter }
    }
}

/// Axum middleware that enforces per-IP rate limiting.
///
/// If `ConnectInfo` is not available (e.g., in test environments using
/// `tower::ServiceExt::oneshot`), the middleware falls back to `127.0.0.1`
/// so tests can exercise rate limiting without a real TCP connection.
///
/// Returns `429 Too Many Requests` when the rate limit is exceeded.
pub async fn rate_limit_middleware(
    connect_info: Option<ConnectInfo<SocketAddr>>,
    State(state): State<RateLimitState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = connect_info
        .map(|ci| ci.0.ip())
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));

    match state.limiter.check_key(&ip) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err(StatusCode::TOO_MANY_REQUESTS),
    }
}
