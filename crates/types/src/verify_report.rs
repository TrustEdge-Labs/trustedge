//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyReport {
    pub signature: String,
    pub continuity: String,
    pub segments: u32,
    pub duration_s: f32,
    pub profile: String,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_gap_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_order: Option<OutOfOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub verify_time_ms: u64,
    #[serde(default)]
    pub chain_tip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct OutOfOrder {
    pub expected: u32,
    pub found: u32,
}
