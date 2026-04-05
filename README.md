<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

[![CI Status](https://github.com/TrustEdge-Labs/trustedge/workflows/CI/badge.svg)](https://github.com/TrustEdge-Labs/trustedge/actions)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Commercial License](https://img.shields.io/badge/Commercial-License%20Available-blue.svg)](mailto:enterprise@trustedgelabs.com)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-5.0-blue.svg)](https://github.com/TrustEdge-Labs/trustedge/releases/tag/v5.0)
[![YubiKey](https://img.shields.io/badge/YubiKey-Hardware%20Supported-green.svg)](https://www.yubico.com/)

# TrustEdge

Cryptographic provenance for edge device data and software supply chains.

## Problem

Edge devices generate data, video, sensor readings, audio, logs, but there is no way to
prove that data has not been tampered with between capture and consumption. Software teams
generate SBOMs but can't cryptographically prove the SBOM matches the actual binary.
TrustEdge provides cryptographic chain of custody: sign at the source, verify with an
independent service, and receive a cryptographic receipt as proof.

## Quick Start: SBOM Attestation

Cryptographically bind an SBOM to a binary artifact and verify it:

```bash
# Generate a signing key
trst keygen --out-key build.key --out-pub build.pub --unencrypted

# Attest: bind SBOM to binary
trst attest-sbom --binary target/release/myapp --sbom bom.cdx.json \
  --device-key build.key --device-pub build.pub --out attestation.te-attestation.json

# Verify locally
trst verify-attestation attestation.te-attestation.json --device-pub "$(cat build.pub)"
```

Or use the [GitHub Action](https://github.com/TrustEdge-Labs/attest-sbom-action) for one-line CI integration (verifies `trst` binary SHA256 before executing):

```yaml
- uses: TrustEdge-Labs/attest-sbom-action@v1
  with:
    binary: target/release/myapp
    sbom: bom.cdx.json
```

TrustEdge self-attests its own releases: every GitHub release includes `trst.te-attestation.json` and `build.pub` as downloadable assets. Verify with `trst verify-attestation trst.te-attestation.json --device-pub "$(cat build.pub)"`.

See the [third-party attestation guide](docs/third-party-attestation-guide.md) for complete manual and CI workflows.

## Quick Start: Archive Verification

For continuous data streams (video, audio, sensor data), use `.trst` archives:

```bash
git clone https://github.com/TrustEdge-Labs/trustedge.git && cd trustedge
cp deploy/.env.example deploy/.env
docker compose -f deploy/docker-compose.yml up -d --build
./scripts/demo.sh
```

This starts the full TrustEdge stack (platform server, PostgreSQL, dashboard) and runs an
end-to-end demo: key generation, archive wrapping, server-side verification, and receipt
retrieval.

No Docker? Run `./scripts/demo.sh --local` with just Rust installed for local-only
verification.

## Use Cases

### Firmware SBOM Attestation

An IoT manufacturer ships firmware updates and needs to prove the SBOM matches the actual
binary for EU CRA compliance. TrustEdge cryptographically binds the two artifacts together.

```bash
trst attest-sbom --binary firmware-v2.3.bin --sbom firmware-v2.3.cdx.json \
  --device-key build.key --device-pub build.pub --out firmware-v2.3.te-attestation.json
trst verify-attestation firmware-v2.3.te-attestation.json --device-pub "$(cat build.pub)" \
  --binary firmware-v2.3.bin --sbom firmware-v2.3.cdx.json
```

The `.te-attestation.json` is a lightweight JSON document with Ed25519 signature over
BLAKE3 hashes of both files, a random nonce, and timestamp. Any third party can verify
it using only the attestation document and the embedded public key.

### Drone Inspection

A drone captures inspection footage of infrastructure. The operator needs to prove the video
has not been edited between capture and submission to the client.

```bash
trst keygen --out-key drone.key --out-pub drone.pub
# For CI/automation (no passphrase prompt):
# trst keygen --out-key drone.key --out-pub drone.pub --unencrypted
trst wrap --in flight-recording.bin --out inspection.trst \
  --data-type video --source "DJI-Mavic-3E" --description "Bridge inspection flight 2024-03-15" \
  --device-key drone.key --device-pub drone.pub
trst verify inspection.trst --device-pub "$(cat drone.pub)"
```

### Sensor Logs

Industrial sensors produce continuous readings. Regulators need assurance that the submitted
logs match what was actually recorded.

```bash
trst wrap --in sensor-readings.csv --out telemetry.trst \
  --data-type sensor --source "Modbus-RTU-Unit-7" --description "Temperature readings Q1 2024" \
  --device-key sensor.key --device-pub sensor.pub
```

### Body Camera

Law enforcement body cameras record interactions. The footage must be verifiably unaltered
for evidentiary use.

```bash
trst wrap --in bodycam-clip.mp4 --out evidence.trst \
  --data-type video --source "Axon-Body-4" --description "Incident report 2024-0847" \
  --device-key officer.key --device-pub officer.pub
```

### Audio Capture

A journalist records an interview. The publication needs to prove the audio is the original
unedited recording.

```bash
trst wrap --in interview.wav --out recording.trst \
  --data-type audio --source "Zoom-H6" --description "Interview with source, 2024-03-15" \
  --device-key recorder.key --device-pub recorder.pub
```

Named profiles are also available for use-case-specific metadata:

```bash
# Sensor data with geo-tagging
trst wrap --profile sensor --in readings.csv --out telemetry.trst \
  --sample-rate 100 --unit celsius --sensor-model DHT22 \
  --latitude 40.7128 --longitude=-74.0060 \
  --device-key sensor.key --device-pub sensor.pub

# Audio with codec metadata
trst wrap --profile audio --in call.wav --out recording.trst \
  --sample-rate 44100 --bit-depth 16 --channels 2 --codec pcm \
  --device-key mic.key --device-pub mic.pub

# Application logs
trst wrap --profile log --in access.log --out logs.trst \
  --application nginx --host web-01 --log-level info --log-format json \
  --device-key server.key --device-pub server.pub
```

To decrypt and recover original data:

```bash
trst unwrap recording.trst --device-key mic.key --out recovered.wav
```

For cam.video-specific archives with frame rate and segment duration, see [examples/cam.video](examples/cam.video/).

## How It Works

**Security Posture (v5.0):** TrustEdge uses RSA OAEP-SHA256 for all asymmetric operations. Envelopes are v2-only format with HKDF-SHA256 key derivation. Point attestations use Ed25519 signing over BLAKE3 hashes with random nonces (`.te-attestation.json`). Device private keys are encrypted at rest using TRUSTEDGE-KEY-V1 format (PBKDF2-HMAC-SHA256 600k + AES-256-GCM, versioned metadata); a passphrase is prompted at runtime. Key-holding structs zeroize memory on drop. Platform HTTP endpoints enforce a 2 MB body limit and per-IP rate limiting on `/v1/verify` and `/v1/verify-attestation`. JWKS signing key path is configurable via `JWKS_KEY_PATH`. Receipt TTL is configurable via `RECEIPT_TTL_SECS` (default 3600s). 471 tests across 9 workspace crates.

**Two attestation modes:**

**Point Attestation** (for single artifacts: SBOMs, firmware, documents):
1. **Hash** -- BLAKE3 hash of the binary and SBOM (or any two artifacts)
2. **Sign** -- Ed25519 signature over the hashes, timestamp, and random nonce
3. **Verify** -- Any third party verifies using the embedded public key
4. **Receipt** -- Platform issues a JWS receipt proving verification at a specific time

**Stream Attestation** (for continuous data: video, audio, sensor readings):
1. **Sign** -- Ed25519 keypair (or YubiKey ECDSA P-256) signs data at capture
2. **Encrypt** -- Chunked AES-256-GCM encryption with HKDF-derived keys
3. **Wrap** -- Chunks, manifest, and signature packaged into a `.trst` archive with BLAKE3 continuity chain
4. **Verify** -- Independent verification service checks signature, chain integrity, and manifest
5. **Unwrap** -- Original data recovered after mandatory signature and chain verification
6. **Receipt** -- Cryptographic receipt proving verification at a specific time

Hardware-backed signing is supported via YubiKey PIV (`trst wrap --backend yubikey`). See [docs/yubikey-guide.md](docs/yubikey-guide.md).

## Architecture

TrustEdge is a Rust workspace with 9 crates organized as a monolithic core library with thin
CLI and WASM shells, plus a platform verification service with PostgreSQL backend and
SvelteKit dashboard.

For crate breakdown, module hierarchy, data flow, and testing details, see
[docs/architecture.md](docs/architecture.md).

For hardware-backed signing with YubiKey PIV, see [docs/yubikey-guide.md](docs/yubikey-guide.md).

## Commercial Support

Building edge devices that need cryptographic provenance? We offer commercial SDK, custom
hardware integration, fleet management, and compliance consulting.

Contact: pilot@trustedgelabs.com

## License

Mozilla Public License 2.0. See [LICENSE](LICENSE) for details.

Commercial licenses available for enterprise use. Contact: enterprise@trustedgelabs.com

(c) 2025 TrustEdge Labs LLC

---

*TrustEdge -- Privacy and trust at the edge.*
