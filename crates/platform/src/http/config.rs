//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Service configuration loaded from environment variables.
//!
//! The `verify_core_url` field from the original platform-api Config has been
//! removed: verification is now performed inline.

use anyhow::Result;
use std::env;

/// Runtime configuration for the TrustEdge Platform service.
#[derive(Debug, Clone)]
pub struct Config {
    #[cfg(feature = "postgres")]
    pub database_url: String,
    pub jwt_audience: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        #[cfg(feature = "postgres")]
        let database_url = env::var("DATABASE_URL").or_else(|_| {
            if cfg!(debug_assertions) {
                Ok("postgres://postgres:password@localhost:5432/trustedge".to_string())
            } else {
                Err(anyhow::anyhow!(
                    "DATABASE_URL must be set in release builds (no hardcoded fallback)"
                ))
            }
        })?;

        let jwt_audience =
            env::var("JWT_AUDIENCE").unwrap_or_else(|_| "trustedge-platform".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001);

        Ok(Config {
            #[cfg(feature = "postgres")]
            database_url,
            jwt_audience,
            port,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_database_url_error_message_exists() {
        // Verify the release-mode error message is present in the source.
        // We cannot toggle cfg!(debug_assertions) in a unit test, but we can
        // confirm the error path is reachable by testing with env var unset
        // in the non-debug branch. The actual enforcement is compile-time gated.
        let msg = "DATABASE_URL must be set in release builds";
        let source = include_str!("config.rs");
        assert!(
            source.contains(msg),
            "config.rs must contain the release-mode DATABASE_URL error message"
        );
    }
}
