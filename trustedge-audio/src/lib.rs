//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
// lib.rs - Core library for privacy-preserving edge data processing
//
//! TrustEdge - Privacy and trust at the edge
//! 
//! Core library for privacy-preserving edge data processing

use serde::{Serialize, Deserialize};

// set up nonce length
pub const NONCE_LEN: usize = 12;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkChunk {
    pub sequence: u64,
    pub data: Vec<u8>,  
    pub manifest: Vec<u8>, 
    pub nonce: [u8; NONCE_LEN],  // Include the actual nonce used for encryption
    pub timestamp: u64,  
}

impl NetworkChunk {
    // Create a new NetworkChunk
    pub fn new(seq: u64, encrypted_data: Vec<u8>, manifest_bytes: Vec<u8>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Self {
            sequence: seq,
            data: encrypted_data,
            manifest: manifest_bytes,
            nonce: [0; NONCE_LEN], // Default nonce - should be set explicitly
            timestamp,
        }
    }
    
    // Create a new NetworkChunk with explicit nonce
    pub fn new_with_nonce(seq: u64, encrypted_data: Vec<u8>, manifest_bytes: Vec<u8>, nonce: [u8; NONCE_LEN]) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Self {
            sequence: seq,
            data: encrypted_data,
            manifest: manifest_bytes,
            nonce,
            timestamp,
        }
    }
    
    // Simple validation of the network chunk
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.data.is_empty() {
            return Err(anyhow::anyhow!("Chunk data is empty"));
        }
        if self.manifest.is_empty() {
            return Err(anyhow::anyhow!("Manifest is empty"));
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("Time error: {}", e))?
            .as_secs();
            
        if self.timestamp > now + 300 { // Not more than 5 minutes in future
            return Err(anyhow::anyhow!("Chunk timestamp is too far in the future"));
        }
        
        Ok(())
    }
}
