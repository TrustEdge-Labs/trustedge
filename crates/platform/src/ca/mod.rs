//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Certificate Authority module — feature-gated behind `ca` feature.
//!
//! Enterprise-grade certificate authority built on TrustEdge's Universal Backend system.
//! Provides hardware-backed PKI services with YubiKey and CloudHSM support.
//!
//! Note: This module is private to the crate. Plan 02 will expose it via the HTTP layer.

// Phase 26 will wire up all CA internals; suppress dead_code until then
#![allow(dead_code)]

pub mod auth;
pub mod database;
pub mod error;
pub mod models;
pub mod service;

#[cfg(feature = "http")]
pub mod api;

/// Configuration for the CA service
#[derive(Debug, Clone)]
pub struct CAConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub ca_name: String,
    pub ca_organization: String,
    pub ca_country: String,
    pub certificate_validity_days: u32,
}

impl Default for CAConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost/trustedge_ca".to_string(),
            jwt_secret: "your-secret-key".to_string(),
            ca_name: "TrustEdge Enterprise CA".to_string(),
            ca_organization: "TrustEdge Labs LLC".to_string(),
            ca_country: "US".to_string(),
            certificate_validity_days: 365,
        }
    }
}
