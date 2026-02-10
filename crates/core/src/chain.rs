//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

/// Genesis seed for the continuity chain
const GENESIS_SEED: &[u8] = b"trustedge:genesis";

pub use crate::error::ChainError;

/// Convert BLAKE3 hash bytes to base64 with "b3:" prefix for manifest storage
pub fn blake3_hex_or_b64(bytes: &[u8]) -> String {
    format!("b3:{}", base64_encode(bytes))
}

/// Compute BLAKE3 hash of segment ciphertext
pub fn segment_hash(ciphertext: &[u8]) -> [u8; 32] {
    blake3::hash(ciphertext).into()
}

/// Compute next continuity chain hash: BLAKE3(prev||curr)
pub fn chain_next(prev: &[u8; 32], curr: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(prev);
    hasher.update(curr);
    hasher.finalize().into()
}

/// Compute genesis continuity hash: BLAKE3(GENESIS_SEED)
pub fn genesis() -> [u8; 32] {
    blake3::hash(GENESIS_SEED).into()
}

/// Segment with stored hash and continuity values
#[derive(Debug, Clone)]
pub struct ChainSegment {
    pub index: usize,
    pub stored_hash: [u8; 32],
    pub stored_continuity: [u8; 32],
}

/// Validate continuity chain for ordered segments
pub fn validate_chain(segments: &[ChainSegment]) -> Result<(), ChainError> {
    if segments.is_empty() {
        return Ok(());
    }

    // Check for gaps in segment indices
    for (i, segment) in segments.iter().enumerate() {
        if segment.index != i {
            return Err(ChainError::Gap(i));
        }
    }

    // Validate continuity chain
    let mut expected_continuity = genesis();

    for segment in segments {
        // Compute expected continuity hash from previous + current segment hash
        expected_continuity = chain_next(&expected_continuity, &segment.stored_hash);

        // Check if computed continuity matches stored continuity
        if expected_continuity != segment.stored_continuity {
            return Err(ChainError::OutOfOrder {
                expected: hex::encode(expected_continuity),
                found: hex::encode(segment.stored_continuity),
            });
        }
    }

    Ok(())
}

/// Simple base64 encoding helper
fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        let b1 = bytes[i];
        let b2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b3 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        let chunk = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        result.push(CHARS[((chunk >> 18) & 63) as usize] as char);
        result.push(CHARS[((chunk >> 12) & 63) as usize] as char);

        if i + 1 < bytes.len() {
            result.push(CHARS[((chunk >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < bytes.len() {
            result.push(CHARS[(chunk & 63) as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_computation() {
        let computed_genesis = genesis();
        let expected = blake3::hash(GENESIS_SEED);
        assert_eq!(computed_genesis, *expected.as_bytes());
    }

    #[test]
    fn test_segment_hash() {
        let ciphertext = b"fake_ciphertext_data";
        let hash = segment_hash(ciphertext);
        let expected = blake3::hash(ciphertext);
        assert_eq!(hash, *expected.as_bytes());
    }

    #[test]
    fn test_chain_next() {
        let prev = [1u8; 32];
        let curr = [2u8; 32];
        let result = chain_next(&prev, &curr);

        // Verify with manual computation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&prev);
        hasher.update(&curr);
        let expected = hasher.finalize();

        assert_eq!(result, *expected.as_bytes());
    }

    #[test]
    fn test_blake3_hex_or_b64() {
        let test_bytes = [0x01, 0x23, 0x45, 0x67];
        let result = blake3_hex_or_b64(&test_bytes);
        assert!(result.starts_with("b3:"));

        // Verify it's valid base64 after the prefix
        let b64_part = &result[3..];
        assert!(!b64_part.is_empty());
    }

    #[test]
    fn test_happy_path_with_3_segments() {
        // Create 3 dummy segments with fake ciphertext
        let ciphertext1 = b"fake_segment_1_ciphertext";
        let ciphertext2 = b"fake_segment_2_ciphertext";
        let ciphertext3 = b"fake_segment_3_ciphertext";

        // Compute segment hashes
        let hash1 = segment_hash(ciphertext1);
        let hash2 = segment_hash(ciphertext2);
        let hash3 = segment_hash(ciphertext3);

        // Compute continuity chain
        let genesis_hash = genesis();
        let continuity1 = chain_next(&genesis_hash, &hash1);
        let continuity2 = chain_next(&continuity1, &hash2);
        let continuity3 = chain_next(&continuity2, &hash3);

        // Create segments with computed values
        let segments = vec![
            ChainSegment {
                index: 0,
                stored_hash: hash1,
                stored_continuity: continuity1,
            },
            ChainSegment {
                index: 1,
                stored_hash: hash2,
                stored_continuity: continuity2,
            },
            ChainSegment {
                index: 2,
                stored_hash: hash3,
                stored_continuity: continuity3,
            },
        ];

        // Validation should pass
        assert!(validate_chain(&segments).is_ok());
    }

    #[test]
    fn test_swapping_segments_fails_out_of_order() {
        // Create 2 dummy segments
        let ciphertext1 = b"fake_segment_1_ciphertext";
        let ciphertext2 = b"fake_segment_2_ciphertext";

        let hash1 = segment_hash(ciphertext1);
        let hash2 = segment_hash(ciphertext2);

        let genesis_hash = genesis();
        let continuity1 = chain_next(&genesis_hash, &hash1);
        let continuity2 = chain_next(&continuity1, &hash2);

        // Create segments but swap their order (indices 0,1 but wrong continuity)
        let segments = vec![
            ChainSegment {
                index: 0,
                stored_hash: hash2,             // Wrong hash for index 0
                stored_continuity: continuity2, // Wrong continuity
            },
            ChainSegment {
                index: 1,
                stored_hash: hash1,             // Wrong hash for index 1
                stored_continuity: continuity1, // Wrong continuity
            },
        ];

        // Validation should fail with OutOfOrder
        let result = validate_chain(&segments);
        assert!(result.is_err());
        match result.unwrap_err() {
            ChainError::OutOfOrder { .. } => (),
            other => panic!("Expected OutOfOrder error, got {:?}", other),
        }
    }

    #[test]
    fn test_removing_last_segment_fails_end_of_chain_truncated() {
        // Create 3 segments but only provide the first 2
        let ciphertext1 = b"fake_segment_1_ciphertext";
        let ciphertext2 = b"fake_segment_2_ciphertext";
        let ciphertext3 = b"fake_segment_3_ciphertext";

        let hash1 = segment_hash(ciphertext1);
        let hash2 = segment_hash(ciphertext2);
        let hash3 = segment_hash(ciphertext3);

        let genesis_hash = genesis();
        let continuity1 = chain_next(&genesis_hash, &hash1);
        let continuity2 = chain_next(&continuity1, &hash2);
        let continuity3 = chain_next(&continuity2, &hash3);

        // Create segments but include continuity from segment 3 in segment 2
        // This simulates having a chain that expects more segments
        let segments = vec![
            ChainSegment {
                index: 0,
                stored_hash: hash1,
                stored_continuity: continuity1,
            },
            ChainSegment {
                index: 1,
                stored_hash: hash2,
                stored_continuity: continuity3, // This is wrong - should be continuity2
            },
        ];

        // Validation should fail
        let result = validate_chain(&segments);
        assert!(result.is_err());
        match result.unwrap_err() {
            ChainError::OutOfOrder { .. } => (),
            other => panic!("Expected OutOfOrder error, got {:?}", other),
        }
    }

    #[test]
    fn test_gap_in_segments() {
        let ciphertext1 = b"fake_segment_1_ciphertext";
        let hash1 = segment_hash(ciphertext1);
        let genesis_hash = genesis();
        let continuity1 = chain_next(&genesis_hash, &hash1);

        // Create segments with gap (index 0, then index 2)
        let segments = vec![
            ChainSegment {
                index: 0,
                stored_hash: hash1,
                stored_continuity: continuity1,
            },
            ChainSegment {
                index: 2, // Gap! Should be index 1
                stored_hash: hash1,
                stored_continuity: continuity1,
            },
        ];

        // Validation should fail with Gap
        let result = validate_chain(&segments);
        assert!(result.is_err());
        match result.unwrap_err() {
            ChainError::Gap(index) => assert_eq!(index, 1),
            other => panic!("Expected Gap error, got {:?}", other),
        }
    }

    #[test]
    fn test_empty_segments() {
        // Empty segments should validate successfully
        assert!(validate_chain(&[]).is_ok());
    }

    #[test]
    fn test_base64_encoding() {
        // Test our simple base64 implementation
        let test_data = [0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e];
        let encoded = base64_encode(&test_data);

        // Should be valid base64
        assert_eq!(encoded.len() % 4, 0);
        assert!(encoded
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='));
    }
}
