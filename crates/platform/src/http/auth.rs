//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Bearer token authentication middleware.
//!
//! `auth_middleware` validates Bearer tokens by hashing them via SHA-256 and
//! looking up the hash in the database. Requires the `postgres` feature.
//!
//! `generate_token` and `hash_token_for_storage` are always available as
//! pure utility functions.

use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Organization context injected by the auth middleware into request extensions.
#[derive(Clone)]
pub struct OrgContext {
    pub org_id: Uuid,
}

/// Hash a raw token using SHA-256. Returns lowercase hex string.
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generate a cryptographically random 32-character alphanumeric API token.
pub fn generate_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const TOKEN_LEN: usize = 32;

    let mut rng = rand::thread_rng();
    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Hash a token for storage in the database. Alias for `hash_token`.
pub fn hash_token_for_storage(token: &str) -> String {
    hash_token(token)
}

/// Auth middleware — validates Bearer tokens via SHA-256 hash lookup in the database.
///
/// On success, injects `OrgContext` into the request extensions.
/// Requires the `postgres` feature.
#[cfg(feature = "postgres")]
pub async fn auth_middleware(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    use axum::http::header::AUTHORIZATION;

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let token_hash = hash_token(token);

    let org_id = crate::database::get_org_by_token_hash(&pool, &token_hash)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let org_context = OrgContext { org_id };
    request.extensions_mut().insert(org_context);

    Ok(next.run(request).await)
}
