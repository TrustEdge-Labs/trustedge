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
//!
//! ## Proxy-aware IP extraction
//!
//! When deployed behind a reverse proxy (e.g., Docker nginx), the TCP peer IP
//! is always the proxy, meaning all clients share one rate-limit bucket. To
//! address this, `TRUSTED_PROXIES` can be set to a comma-separated list of
//! CIDR blocks (e.g., `10.0.0.0/8,172.16.0.0/12`). When the peer IP is within
//! a trusted CIDR, the rightmost non-trusted IP from `X-Forwarded-For` is used
//! as the effective client IP.
//!
//! When `TRUSTED_PROXIES` is unset or empty, the peer IP is always used
//! (preserves prior behavior).

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request},
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
    /// CIDR blocks for trusted reverse proxies.
    /// When non-empty, X-Forwarded-For is trusted when the peer is in this list.
    pub trusted_proxies: Vec<ipnet::IpNet>,
}

impl RateLimitState {
    /// Create a new rate limiter allowing `rps` requests per second per IP.
    ///
    /// Governor defaults burst capacity to the quota replenishment rate,
    /// which means a client can burst up to `rps` requests before being throttled.
    ///
    /// `trusted_proxies` is the list of CIDR blocks for trusted reverse proxies.
    /// Pass an empty `Vec` to preserve the prior behavior (peer IP only).
    pub fn new(rps: u32, trusted_proxies: Vec<ipnet::IpNet>) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(rps).expect("rps must be greater than zero"));
        let limiter = Arc::new(RateLimiter::keyed(quota));
        Self {
            limiter,
            trusted_proxies,
        }
    }
}

/// Determine the effective client IP, respecting trusted proxy headers.
///
/// Rules (in priority order):
/// 1. If `trusted_proxies` is empty → return `peer_ip` unchanged.
/// 2. If `peer_ip` is NOT in any trusted CIDR → return `peer_ip` (ignore header).
/// 3. If `peer_ip` IS trusted AND `X-Forwarded-For` is present → parse the
///    comma-separated list from right to left and return the rightmost IP that
///    is NOT in the trusted proxy list.  If all IPs are trusted or the header
///    is malformed → fall back to `peer_ip`.
/// 4. If `peer_ip` IS trusted but `X-Forwarded-For` is absent → return `peer_ip`.
pub fn extract_client_ip(
    peer_ip: IpAddr,
    headers: &HeaderMap,
    trusted_proxies: &[ipnet::IpNet],
) -> IpAddr {
    // Rule 1: no trusted proxies configured — use peer IP.
    if trusted_proxies.is_empty() {
        return peer_ip;
    }

    let peer_is_trusted = trusted_proxies
        .iter()
        .any(|cidr| cidr.contains(&peer_ip));

    // Rule 2: peer is not a trusted proxy — ignore X-Forwarded-For.
    if !peer_is_trusted {
        return peer_ip;
    }

    // Rule 3/4: peer is trusted — look at X-Forwarded-For.
    let xff = match headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        Some(v) => v,
        None => return peer_ip, // Rule 4: absent header
    };

    // Walk from right to left, skip trusted IPs, take first untrusted one.
    for part in xff.split(',').rev() {
        let trimmed = part.trim();
        if let Ok(ip) = trimmed.parse::<IpAddr>() {
            let ip_is_trusted = trusted_proxies.iter().any(|cidr| cidr.contains(&ip));
            if !ip_is_trusted {
                return ip;
            }
        }
    }

    // All IPs were trusted or header was fully malformed — fall back to peer.
    peer_ip
}

/// Axum middleware that enforces per-IP rate limiting.
///
/// If `ConnectInfo` is not available (e.g., in test environments using
/// `tower::ServiceExt::oneshot`), the middleware falls back to `127.0.0.1`
/// so tests can exercise rate limiting without a real TCP connection.
///
/// When the rate limit is exceeded, returns `429 Too Many Requests` with a
/// `Retry-After: 1` header (RFC 6585 §4).
pub async fn rate_limit_middleware(
    connect_info: Option<ConnectInfo<SocketAddr>>,
    State(state): State<RateLimitState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let peer_ip = connect_info
        .map(|ci| ci.0.ip())
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));

    let effective_ip = extract_client_ip(peer_ip, req.headers(), &state.trusted_proxies);

    match state.limiter.check_key(&effective_ip) {
        Ok(_) => next.run(req).await,
        Err(_) => Response::builder()
            .status(429)
            .header("retry-after", "1")
            .body(Body::empty())
            .expect("static 429 response must always be buildable"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use std::net::IpAddr;

    fn parse_cidr(s: &str) -> ipnet::IpNet {
        s.parse().unwrap()
    }

    fn peer(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    fn headers_with_xff(xff: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", xff.parse().unwrap());
        h
    }

    // -------------------------------------------------------------------
    // extract_client_ip unit tests
    // -------------------------------------------------------------------

    /// With empty trusted_proxies, peer IP is always returned regardless of
    /// X-Forwarded-For header.
    #[test]
    fn test_extract_client_ip_empty_trusted_proxies() {
        let trusted: Vec<ipnet::IpNet> = vec![];
        let headers = headers_with_xff("10.99.99.1");
        let result = extract_client_ip(peer("10.1.2.3"), &headers, &trusted);
        assert_eq!(result, peer("10.1.2.3"));
    }

    /// When peer is NOT in the trusted list, X-Forwarded-For must be ignored.
    #[test]
    fn test_extract_client_ip_untrusted_peer_ignores_xff() {
        let trusted = vec![parse_cidr("10.0.0.0/8")];
        let headers = headers_with_xff("192.168.1.1");
        // peer 172.16.0.1 is not in 10.0.0.0/8, so use peer IP
        let result = extract_client_ip(peer("172.16.0.1"), &headers, &trusted);
        assert_eq!(result, peer("172.16.0.1"));
    }

    /// When peer is trusted and X-Forwarded-For is present, the forwarded IP
    /// (rightmost non-trusted) is returned.
    #[test]
    fn test_extract_client_ip_trusted_peer_returns_forwarded_ip() {
        let trusted = vec![parse_cidr("10.0.0.0/8")];
        let headers = headers_with_xff("203.0.113.5");
        // peer 10.0.0.1 is trusted → use XFF
        let result = extract_client_ip(peer("10.0.0.1"), &headers, &trusted);
        assert_eq!(result, peer("203.0.113.5"));
    }

    /// When peer is trusted but X-Forwarded-For is absent, peer IP is returned.
    #[test]
    fn test_extract_client_ip_trusted_peer_no_xff_returns_peer() {
        let trusted = vec![parse_cidr("10.0.0.0/8")];
        let headers = HeaderMap::new(); // no X-Forwarded-For
        let result = extract_client_ip(peer("10.0.0.1"), &headers, &trusted);
        assert_eq!(result, peer("10.0.0.1"));
    }

    /// Multi-hop X-Forwarded-For: client → proxy1 (trusted) → proxy2 (trusted).
    /// The rightmost non-trusted IP is the real client.
    #[test]
    fn test_extract_client_ip_multi_hop_picks_rightmost_non_trusted() {
        let trusted = vec![parse_cidr("10.0.0.0/8")];
        // XFF built left-to-right: real client, then proxies added rightward
        // "203.0.113.5, 10.1.0.1, 10.2.0.2" — rightmost non-trusted is 203.0.113.5
        let headers = headers_with_xff("203.0.113.5, 10.1.0.1, 10.2.0.2");
        let result = extract_client_ip(peer("10.0.0.1"), &headers, &trusted);
        assert_eq!(result, peer("203.0.113.5"));
    }

    /// RateLimitState::new correctly stores a CIDR block.
    #[test]
    fn test_rate_limit_state_stores_trusted_proxies() {
        let proxies = vec![parse_cidr("10.0.0.0/8")];
        let state = RateLimitState::new(10, proxies.clone());
        assert_eq!(state.trusted_proxies.len(), 1);
        assert_eq!(state.trusted_proxies[0], proxies[0]);
    }
}
