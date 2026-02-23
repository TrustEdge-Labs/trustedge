//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VerificationReceipt {
    pub verification_id: String,
    pub profile: String,
    pub device_id: String,
    pub manifest_digest: String,
    pub segments: u32,
    pub duration_s: f32,
    pub signature: String,
    pub continuity: String,
    pub issued_at: String,
    pub service_kid: String,
    pub chain_tip: String,
}
