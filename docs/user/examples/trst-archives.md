<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# .trst Archive System Examples

The TrustEdge .trst archive system provides secure archival with Ed25519 digital signatures and cryptographic chunk verification, ideal for evidence collection, security camera footage, and tamper-evident data storage.

## Basic Archive Creation and Verification

**Create a basic .trst archive:**
```bash
# Create sample data
echo "This is sensitive evidence data" > evidence.txt

# Create a .trst archive with digital signature
./target/release/trst wrap --in evidence.txt --out evidence.trst --device-id "DEVICE001"

# The archive directory structure:
ls -la evidence.trst/
# drwxr-xr-x  3 user user 4096 evidence.trst/
# -rw-r--r--  1 user user  512 manifest.json    # Signed manifest
# drwxr-xr-x  2 user user 4096 chunks/           # Chunk directory
# -rw-r--r--  1 user user   32 chunks/00000.bin  # Data chunks

# Extract the device public key for verification
cat evidence.trst/manifest.json | grep -o '"device_pub_key":"[^"]*"' | cut -d'"' -f4 > device.pub

# Verify the archive integrity
./target/release/trst verify evidence.trst --device-pub "$(cat device.pub)"
```

**Expected verification output:**
```
Signature: PASS
Continuity: PASS
Segments: 1  Duration(s): 0.0  Chunk(s): 1.0
```

## Security Camera Archive Workflow

**High-quality video evidence archival:**
```bash
# Create a video evidence archive with detailed metadata
./target/release/trst wrap \
  --in security_footage.bin \
  --out court_evidence.trst \
  --profile cam.video \
  --fps 60 \
  --resolution 3840x2160 \
  --device-model "SecureCam Pro X1" \
  --device-id "CAM-COURTROOM-01" \
  --started-at "2025-01-15T14:30:00Z" \
  --tz "America/New_York"

# Verify with stored device certificate
./target/release/trst verify court_evidence.trst --device-pub "ed25519:GAUpGXoor5gP6JDkeVtj/PV4quuyLlZlojizplendEUlSU="

# Example successful verification:
# Signature: PASS
# Continuity: PASS
# Segments: 16  Duration(s): 32.0  Chunk(s): 2.0
```

## Continuous Evidence Chain

**Link multiple archives for tamper-evident chain:**
```bash
# Create first archive in chain
./target/release/trst wrap \
  --in segment_001.bin \
  --out segment_001.trst \
  --device-id "CAM-LOBBY-01"

# Get hash for chain linking
HASH_001=$(blake3sum segment_001.trst/manifest.json | cut -d' ' -f1)

# Create linked archive
./target/release/trst wrap \
  --in segment_002.bin \
  --out segment_002.trst \
  --device-id "CAM-LOBBY-01" \
  --prev-archive-hash "$HASH_001"

# Verify the complete chain
./target/release/trst verify segment_001.trst --device-pub "$(cat device.pub)"
./target/release/trst verify segment_002.trst --device-pub "$(cat device.pub)"

# Check chain linkage
echo "Previous hash in segment_002: $(cat segment_002.trst/manifest.json | grep prev_archive_hash)"
echo "Actual hash of segment_001: $HASH_001"
```

## Large File Chunked Archival

**Efficient handling of large files with custom chunk sizes:**
```bash
# Archive large file with 4MB chunks for efficiency
./target/release/trst wrap \
  --in large_dataset.bin \
  --out dataset.trst \
  --chunk-size 4194304 \
  --device-model "DataLogger V2" \
  --device-id "SENSOR-LAB-03"

# Archive with time-based chunking (for streaming data)
./target/release/trst wrap \
  --in audio_stream.bin \
  --out audio.trst \
  --chunk-seconds 5.0 \
  --profile cam.video \
  --fps 48000 \
  --codec "pcm_s16le"

# Verify large archive
./target/release/trst verify dataset.trst --device-pub "$(cat device.pub)"
```

## Hybrid Encryption + Archive Workflow

**Combine envelope encryption with archive format for maximum security:**
```bash
# Step 1: Encrypt sensitive data with envelope encryption
./target/release/trustedge-core \
  --input confidential.pdf \
  --envelope encrypted.trst \
  --key-out secret.key

# Step 2: Archive the encrypted envelope with digital signatures
./target/release/trst wrap \
  --in encrypted.trst \
  --out archived.trst \
  --profile data.secure \
  --device-id "VAULT-001"

# Step 3: Verify archive integrity
./target/release/trst verify archived.trst --device-pub "$(cat device.pub)"

# Step 4: Recovery process
# First extract the encrypted envelope (not shown here, requires archive extraction)
# Then decrypt the envelope
./target/release/trustedge-core \
  --decrypt \
  --input encrypted.trst \
  --out recovered.pdf \
  --key-hex $(cat secret.key)
```

## Archive Metadata Inspection

**Examine archive contents without verification:**
```bash
# Inspect manifest without full verification
cat evidence.trst/manifest.json | jq .

# Check archive structure
find evidence.trst -type f -exec ls -lh {} \;

# Verify individual chunk hashes manually
cd evidence.trst/chunks
for chunk in *.bin; do
  echo -n "$chunk: "
  blake3sum "$chunk" | cut -d' ' -f1
done
```

---


[← Back to Examples Index](README.md)

---

*This document is part of the TrustEdge project documentation.*

**📖 Links:**
- **[TrustEdge Home](https://github.com/TrustEdge-Labs/trustedge)** - Main repository
- **[TrustEdge Labs](https://github.com/TrustEdge-Labs)** - Organization profile
- **[Documentation](https://github.com/TrustEdge-Labs/trustedge/tree/main/docs)** - Complete docs
- **[Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** - Bug reports & features

**⚖️ Legal:**
- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0 ([MPL-2.0](https://mozilla.org/MPL/2.0/))
- **Commercial**: [Enterprise licensing available](mailto:enterprise@trustedgelabs.com)
