//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::verify_report::VerifyReport;

/// A reference to a single verified segment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SegmentRef {
    pub index: u32,
    pub hash: String,
}

/// Options controlling verification behavior.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyOptions {
    #[serde(default)]
    pub return_receipt: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// Request payload sent to the verification service.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyRequest {
    pub device_pub: String,
    pub manifest: serde_json::Value,
    pub segments: Vec<SegmentRef>,
    #[serde(default)]
    pub options: VerifyOptions,
}

/// Response payload returned by the verification service.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyResponse {
    pub verification_id: String,
    pub result: VerifyReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}
