//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! CA error types.

use thiserror::Error;

pub type CAResult<T> = Result<T, CAError>;

#[derive(Error, Debug)]
pub enum CAError {
    #[cfg(feature = "postgres")]
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[cfg(not(feature = "postgres"))]
    #[error("Database error: {0}")]
    Database(String),

    #[error("TrustEdge backend error: {0}")]
    Backend(#[from] trustedge_core::BackendError),

    #[error("Certificate parsing error: {0}")]
    CertificateParsing(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Tenant not found: {0}")]
    TenantNotFound(String),

    #[error("Certificate not found: {0}")]
    CertificateNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Certificate generation error: {0}")]
    CertificateGeneration(String),
}

impl CAError {
    pub fn status_code(&self) -> u16 {
        match self {
            CAError::Authentication(_) => 401,
            CAError::Authorization(_) => 403,
            CAError::TenantNotFound(_) | CAError::CertificateNotFound(_) => 404,
            CAError::InvalidRequest(_) => 400,
            _ => 500,
        }
    }
}
