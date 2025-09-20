//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: u32,
    pub t0: f64,
    pub t1: f64,
    pub hash: String,
    pub prev_hash: String,
    pub bytes: u64,
    pub nonce: String,
}

impl Segment {
    pub fn new(
        id: u32,
        t0: f64,
        t1: f64,
        hash: &[u8; 32],
        prev_hash: &[u8; 32],
        bytes: u64,
        nonce: &[u8; 24],
    ) -> Self {
        Self {
            id,
            t0,
            t1,
            hash: hex::encode(hash),
            prev_hash: hex::encode(prev_hash),
            bytes,
            nonce: hex::encode(nonce),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestChunk {
    pub approx_duration_s: f64,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub trst_version: String,
    pub profile: String,
    pub device: crate::DeviceInfo,
    pub capture: crate::ManifestCapture,
    pub chunk: ManifestChunk,
    pub segments: Vec<Segment>,
    pub claims: serde_json::Value,
    pub prev_archive_hash: Option<String>,
    pub signature: Option<String>,
}

impl Manifest {
    pub fn to_canonical_bytes(&self, include_signature: bool) -> Result<Vec<u8>> {
        let mut manifest = self.clone();

        if !include_signature {
            manifest.signature = None;
        }

        // Sort claims alphabetically for canonical representation
        if let serde_json::Value::Object(ref mut map) = manifest.claims {
            let sorted: serde_json::Map<String, serde_json::Value> =
                map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            manifest.claims = serde_json::Value::Object(sorted);
        }

        Ok(serde_json::to_vec_pretty(&manifest)?)
    }

    pub fn with_signature(mut self, signature: String) -> Self {
        self.signature = Some(signature);
        self
    }
}
