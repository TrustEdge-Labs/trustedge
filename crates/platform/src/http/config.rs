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
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:password@localhost:5432/trustedge".to_string()
        });

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
