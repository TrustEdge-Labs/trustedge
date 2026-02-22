//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Certificate Authority module — library-only, not wired into the HTTP router.
//!
//! This module provides enterprise-grade PKI services using TrustEdge's Universal Backend system.
//! It is feature-gated behind the `ca` feature flag and currently used only as a library
//! (imported directly by consumers, not exposed via HTTP endpoints in `create_router()`).
//!
//! **Future:** CA routes may be exposed via the platform HTTP layer behind the `ca` feature flag.
//! When that happens, `api.rs` service functions will be wrapped with Axum handler shims.

// Library-only module: functions are public API but not called from HTTP handlers yet
#![allow(dead_code)]

pub mod auth;
pub mod database;
pub mod error;
pub mod models;
pub mod service;

pub mod api;

use std::fmt;
use trustedge_core::Secret;

/// Configuration for the CA service.
///
/// `jwt_secret` is a `Secret<String>` — it cannot be serialized or printed accidentally.
/// Use `CAConfig::builder()` to construct instances.
pub struct CAConfig {
    pub database_url: String,
    jwt_secret: Secret<String>,
    pub ca_name: String,
    pub ca_organization: String,
    pub ca_country: String,
    pub certificate_validity_days: u32,
}

impl CAConfig {
    /// Access the JWT secret as a `&str`.
    ///
    /// The caller is responsible for not logging or persisting the returned value.
    pub fn jwt_secret(&self) -> &str {
        self.jwt_secret.expose_secret().as_str()
    }

    /// Create a `CAConfigBuilder` for fluent construction.
    pub fn builder() -> CAConfigBuilder {
        CAConfigBuilder::default()
    }
}

impl fmt::Debug for CAConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CAConfig")
            .field("database_url", &self.database_url)
            .field("jwt_secret", &"[REDACTED]")
            .field("ca_name", &self.ca_name)
            .field("ca_organization", &self.ca_organization)
            .field("ca_country", &self.ca_country)
            .field("certificate_validity_days", &self.certificate_validity_days)
            .finish()
    }
}

impl Clone for CAConfig {
    fn clone(&self) -> Self {
        Self {
            database_url: self.database_url.clone(),
            jwt_secret: Secret::new(self.jwt_secret.expose_secret().clone()),
            ca_name: self.ca_name.clone(),
            ca_organization: self.ca_organization.clone(),
            ca_country: self.ca_country.clone(),
            certificate_validity_days: self.certificate_validity_days,
        }
    }
}

impl Default for CAConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost/trustedge_ca".to_string(),
            jwt_secret: Secret::new("your-secret-key".to_string()),
            ca_name: "TrustEdge Enterprise CA".to_string(),
            ca_organization: "TrustEdge Labs LLC".to_string(),
            ca_country: "US".to_string(),
            certificate_validity_days: 365,
        }
    }
}

/// Builder for `CAConfig`.
pub struct CAConfigBuilder {
    database_url: String,
    jwt_secret: String,
    ca_name: String,
    ca_organization: String,
    ca_country: String,
    certificate_validity_days: u32,
}

impl Default for CAConfigBuilder {
    fn default() -> Self {
        let defaults = CAConfig::default();
        Self {
            database_url: defaults.database_url,
            jwt_secret: defaults.jwt_secret.expose_secret().clone(),
            ca_name: defaults.ca_name,
            ca_organization: defaults.ca_organization,
            ca_country: defaults.ca_country,
            certificate_validity_days: defaults.certificate_validity_days,
        }
    }
}

impl CAConfigBuilder {
    pub fn jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = secret;
        self
    }

    pub fn database_url(mut self, url: String) -> Self {
        self.database_url = url;
        self
    }

    pub fn ca_name(mut self, name: String) -> Self {
        self.ca_name = name;
        self
    }

    pub fn ca_organization(mut self, org: String) -> Self {
        self.ca_organization = org;
        self
    }

    pub fn ca_country(mut self, country: String) -> Self {
        self.ca_country = country;
        self
    }

    pub fn certificate_validity_days(mut self, days: u32) -> Self {
        self.certificate_validity_days = days;
        self
    }

    pub fn build(self) -> CAConfig {
        CAConfig {
            database_url: self.database_url,
            jwt_secret: Secret::new(self.jwt_secret),
            ca_name: self.ca_name,
            ca_organization: self.ca_organization,
            ca_country: self.ca_country,
            certificate_validity_days: self.certificate_validity_days,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caconfig_debug_redacts_jwt_secret() {
        let config = CAConfig::default();
        let debug_output = format!("{:?}", config);
        assert!(
            debug_output.contains("[REDACTED]"),
            "Debug output must contain [REDACTED], got: {debug_output}"
        );
        assert!(
            !debug_output.contains("your-secret-key"),
            "Debug output must NOT contain the actual JWT secret, got: {debug_output}"
        );
    }

    #[test]
    fn test_caconfig_builder_sets_fields() {
        let config = CAConfig::builder()
            .jwt_secret("production-secret".to_string())
            .database_url("postgresql://prod/ca".to_string())
            .ca_name("My CA".to_string())
            .build();
        assert_eq!(config.jwt_secret(), "production-secret");
        assert_eq!(config.database_url, "postgresql://prod/ca");
        assert_eq!(config.ca_name, "My CA");
    }

    #[test]
    fn test_caconfig_builder_defaults() {
        let config = CAConfig::builder().build();
        assert_eq!(config.jwt_secret(), "your-secret-key");
        assert_eq!(config.ca_country, "US");
        assert_eq!(config.certificate_validity_days, 365);
    }

    #[test]
    fn test_caconfig_clone_redacts_in_debug() {
        let original = CAConfig::builder()
            .jwt_secret("clone-secret".to_string())
            .build();
        let cloned = original.clone();
        assert_eq!(cloned.jwt_secret(), "clone-secret");
        let debug_output = format!("{:?}", cloned);
        assert!(
            !debug_output.contains("clone-secret"),
            "Cloned config debug must not expose secret, got: {debug_output}"
        );
    }
}
