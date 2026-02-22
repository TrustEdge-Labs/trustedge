//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Request/response types for the verification service.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::engine::{SegmentDigest, VerifyReport};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerifyRequest {
    pub device_pub: String,
    pub manifest: Value,
    pub segments: Vec<SegmentDigest>,
    pub options: Option<VerifyOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerifyOptions {
    pub return_receipt: Option<bool>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerifyResponse {
    pub verification_id: String,
    pub result: VerifyReport,
    pub receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}
