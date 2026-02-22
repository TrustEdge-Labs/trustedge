//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! AppState — unified shared state for the consolidated platform service.
//!
//! The `verify_core_url` field from the original platform-api AppState has been
//! removed: verification is now performed inline via direct function calls.

use crate::verify::jwks::KeyManager;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state for the HTTP layer.
///
/// This is the consolidated state combining both platform-api and verify-service
/// state. Verification is performed directly — no HTTP forwarding to a separate
/// verify-core service.
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "postgres")]
    pub db_pool: sqlx::PgPool,
    pub keys: Arc<RwLock<KeyManager>>,
}
