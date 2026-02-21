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
pub struct PolicyV0 {
    pub required_profile: Option<String>,
    pub min_segments: Option<u32>,
    pub chunk_seconds_range: Option<(f32, f32)>,
    pub allowed_codecs: Option<Vec<String>>,
}
