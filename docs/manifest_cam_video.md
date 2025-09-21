# cam.video Manifest Specification

The `cam.video` profile defines a standardized manifest format for trusted video capture archives in the TrustEdge .trst format.

## Schema Fields and Types

```rust
pub struct CamVideoManifest {
    pub trst_version: String,           // Protocol version (e.g., "0.1.0")
    pub profile: String,                // Must be "cam.video"
    pub device: DeviceInfo,             // Device identification and keys
    pub capture: CaptureInfo,           // Capture session metadata
    pub chunk: ChunkInfo,               // Chunking configuration
    pub segments: Vec<SegmentInfo>,     // Per-segment verification data
    pub claims: Vec<String>,            // Additional claims (e.g., "location:unknown")
    pub prev_archive_hash: Option<String>, // Chain to previous archive (optional)
    pub signature: Option<String>,      // Ed25519 signature (excluded from canonical bytes)
}

pub struct DeviceInfo {
    pub id: String,                     // Device identifier (e.g., "te:cam:a1b2c3")
    pub model: String,                  // Device model name
    pub firmware_version: String,       // Firmware version
    pub public_key: String,             // Ed25519 public key ("ed25519:BASE64")
}

pub struct CaptureInfo {
    pub started_at: String,             // RFC3339 timestamp
    pub ended_at: String,               // RFC3339 timestamp
    pub timezone: String,               // Timezone (e.g., "UTC")
    pub fps: f64,                       // Frames per second
    pub resolution: String,             // Resolution (e.g., "1920x1080")
    pub codec: String,                  // Codec (e.g., "raw", "h264")
}

pub struct ChunkInfo {
    pub size_bytes: u64,                // Target chunk size in bytes
    pub duration_seconds: f64,          // Target chunk duration
}

pub struct SegmentInfo {
    pub chunk_file: String,             // Chunk filename (e.g., "00001.bin")
    pub blake3_hash: String,            // BLAKE3 hash of encrypted chunk ("b3:HEX")
    pub start_time: String,             // Relative start time (e.g., "1.000s")
    pub duration_seconds: f64,          // Segment duration
    pub continuity_hash: String,        // Continuity chain state ("b3:HEX")
}
```

## Canonicalization Rules

The manifest must be canonicalized before signing to ensure consistent verification:

### 1. Fixed Object Key Order
All JSON objects must have keys in alphabetical order:
```json
{
  "capture": {...},
  "chunk": {...},
  "claims": [...],
  "device": {...},
  "prev_archive_hash": null,
  "profile": "cam.video",
  "segments": [...],
  "trst_version": "0.1.0"
}
```

### 2. Sorted Maps
All maps and arrays of objects must be sorted by their keys.

### 3. Timestamp Precision
Timestamps (`started_at`, `ended_at`) must be truncated to maximum 3 decimal places for seconds.

### 4. UTF-8 Encoding
All strings must be valid UTF-8 without byte order mark (BOM).

### 5. Signature Exclusion
The `signature` field must be excluded from the canonical bytes used for signing.

## Signature Format

### Ed25519 Signatures
- Algorithm: Ed25519 digital signatures
- Format: `"ed25519:BASE64"`
- Input: Canonical JSON bytes (UTF-8 encoded)
- Verification: Public key from `device.public_key`

Example:
```json
{
  "signature": "ed25519:MEUCIQDx1234...base64signature...5678=="
}
```

## Continuity Chain

The continuity chain provides cryptographic linking between segments to detect tampering or reordering.

### Genesis State
```rust
genesis = blake3("trustedge:genesis")
```

### Chain Progression
```rust
chain_i = blake3(chain_{i-1} || hash_i)
where hash_i = blake3(ciphertext_i)
```

### Hash Prefixes
- Segment hashes: `"b3:HEX"` (BLAKE3 of encrypted chunk data)
- Continuity hashes: `"b3:HEX"` (BLAKE3 of chain state)
- Encryption: `"xchacha20:NONCE"` (XChaCha20-Poly1305 nonce)

## Example Manifest (Redacted)

```json
{
  "trst_version": "0.1.0",
  "profile": "cam.video",
  "device": {
    "id": "te:cam:a1b2c3",
    "model": "TrustEdgeRefCam",
    "firmware_version": "1.0.0",
    "public_key": "ed25519:GAUpGXoor5gP..."
  },
  "capture": {
    "started_at": "2025-01-15T10:30:00Z",
    "ended_at": "2025-01-15T10:32:00Z",
    "timezone": "UTC",
    "fps": 30.0,
    "resolution": "1920x1080",
    "codec": "raw"
  },
  "chunk": {
    "size_bytes": 1048576,
    "duration_seconds": 2.0
  },
  "segments": [
    {
      "chunk_file": "00001.bin",
      "blake3_hash": "b3:abc123...",
      "start_time": "0.000s",
      "duration_seconds": 2.0,
      "continuity_hash": "b3:def456..."
    },
    {
      "chunk_file": "00002.bin",
      "blake3_hash": "b3:ghi789...",
      "start_time": "2.000s",
      "duration_seconds": 2.0,
      "continuity_hash": "b3:jkl012..."
    }
  ],
  "claims": ["location:unknown"],
  "prev_archive_hash": null,
  "signature": "ed25519:MEUCIQDx1234..."
}
```

## Verification Order of Operations

### 1. Parse
- Parse JSON manifest from `manifest.json`
- Validate all required fields are present
- Check data types match schema

### 2. Canonicalize
- Apply canonicalization rules
- Generate canonical JSON bytes
- Exclude signature field from canonical representation

### 3. Signature Verification
- Extract signature from manifest
- Verify Ed25519 signature against canonical bytes
- Use device public key for verification

### 4. Continuity Verification
- Load encrypted chunk files referenced in segments
- Compute BLAKE3 hash of each encrypted chunk
- Verify continuity chain from genesis through all segments
- Ensure no gaps, reordering, or tampering

### Exit Codes
- **0**: All verifications pass
- **10**: Signature verification failed
- **11**: Continuity chain verification failed
- **12**: IO error or malformed archive
- **13**: Invalid CLI arguments
- **14**: Internal error

## Related Documentation

- [CLI Reference](../README.md#p0-golden-path-2-minutes)
- [Acceptance Tests](../trst-cli/tests/acceptance.rs)
- [P0 Test Script](../p0_acceptance.sh)