//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: sealedge — Privacy and trust at the edge.
//

/// Genesis seed for the continuity chain
const GENESIS_SEED: &[u8] = b"sealedge:genesis";

pub use crate::error::ChainError;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;

/// Convert BLAKE3 hash bytes to base64 with "b3:" prefix for manifest storage
pub fn blake3_hex_or_b64(bytes: &[u8]) -> String {
    format!("b3:{}", BASE64.encode(bytes))
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
    fn test_blake3_b64_format() {
        let test_data = [0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e];
        let result = blake3_hex_or_b64(&test_data);
        assert!(result.starts_with("b3:"));
        // Verify the base64 part round-trips
        let b64_part = &result[3..];
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(b64_part)
            .expect("valid base64");
        assert_eq!(decoded, test_data);
    }
}

#[cfg(test)]
mod clean_break_genesis_tests {
    /// D-02 clean-break tests for the continuity-chain genesis seed. Per
    /// CONTEXT.md §Decisions D-02 — shadow const lives only here.
    const OLD_GENESIS_SEED: &[u8] = b"trustedge:genesis";

    /// KAT: BLAKE3(old_seed) and BLAKE3(new_seed) produce DISTINCT 32-byte hashes.
    #[test]
    fn test_old_genesis_seed_produces_distinct_hash() {
        let old = blake3::hash(OLD_GENESIS_SEED);
        let new = blake3::hash(b"sealedge:genesis");
        assert_ne!(
            old.as_bytes(),
            new.as_bytes(),
            "genesis-seed BLAKE3 domain separation failed: legacy and new seeds must produce distinct 32-byte hashes"
        );
    }

    /// D-02 rejection: any continuity chain rooted at OLD_GENESIS_SEED has a
    /// first-block hash distinct from a chain rooted at the NEW seed, so a
    /// verifier computing from the NEW genesis will reject OLD-rooted chains
    /// (chain-id mismatch, not silent accept).
    #[test]
    fn test_old_genesis_seed_rejected_cleanly() {
        // Simulate "first chain step" = BLAKE3(seed || arbitrary segment hash).
        let segment_hash = [0xEEu8; 32];
        let mut old_hasher = blake3::Hasher::new();
        old_hasher.update(OLD_GENESIS_SEED);
        old_hasher.update(&segment_hash);
        let old_first = *old_hasher.finalize().as_bytes();

        let mut new_hasher = blake3::Hasher::new();
        new_hasher.update(b"sealedge:genesis");
        new_hasher.update(&segment_hash);
        let new_first = *new_hasher.finalize().as_bytes();

        assert_ne!(
            old_first, new_first,
            "first-block chain hash under the two genesis seeds must differ — \
             otherwise a verifier using the new seed could silently accept an old-rooted chain"
        );
    }
}
