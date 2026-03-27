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
[![Version](https://img.shields.io/badge/version-3.0-blue.svg)](https://github.com/TrustEdge-Labs/trustedge/releases/tag/v3.0)
[![YubiKey](https://img.shields.io/badge/YubiKey-Hardware%20Supported-green.svg)](https://www.yubico.com/)

# TrustEdge

Cryptographic provenance for edge device data.

## Problem

Edge devices generate data -- video, sensor readings, audio, logs -- but there is no way to
prove that data has not been tampered with between capture and consumption. TrustEdge provides
a cryptographic chain of custody: sign at the source, encrypt, wrap into a tamper-evident
archive, verify with an independent service, and receive a cryptographic receipt as proof.

## Quick Start

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

**Security Posture (v3.0):** TrustEdge uses RSA OAEP-SHA256 for all asymmetric operations. Envelopes are v2-only format with HKDF-SHA256 key derivation. Device private keys are encrypted at rest using TRUSTEDGE-KEY-V1 format (PBKDF2-HMAC-SHA256 600k + AES-256-GCM, versioned metadata); a passphrase is prompted at runtime. Key-holding structs (`PrivateKey`, `SessionInfo`, `ClientAuthResult`, `SymmetricKey`) zeroize memory on drop. Auth timestamps use asymmetric validation (5s future / 300s past tolerance). Generated key files are restricted to owner-only permissions (0600) on Unix. Platform HTTP endpoints enforce a 2 MB body limit and per-IP rate limiting on `/v1/verify`. CORS origins are configurable via `CORS_ORIGINS` env var. JWKS signing key path is configurable via `JWKS_KEY_PATH` (no plaintext in build directories). Receipt TTL is configurable via `RECEIPT_TTL_SECS` (default 3600s). PORT parsing fails fast with a clear error on invalid values. nginx supports conditional TLS termination. The CLI requires `--show-key` or `--key-out` to display encryption keys. 406 tests across 9 workspace crates.

1. **Sign** -- Device generates an Ed25519 keypair (or uses YubiKey ECDSA P-256) and signs data at the point of capture; private keys are encrypted at rest with TRUSTEDGE-KEY-V1 format (PBKDF2 + AES-GCM), passphrase prompted at runtime
2. **Encrypt** -- Data is chunked and each chunk is AES-256-GCM encrypted using HKDF-derived keys (v2 envelope format)
3. **Wrap** -- Chunks, manifest, and signature are packaged into a `.trst` archive with a BLAKE3 continuity chain
4. **Verify** -- An independent verification service checks the signature, chain integrity, and manifest
5. **Unwrap** -- Original data is recovered after mandatory signature and chain verification
6. **Receipt** -- A cryptographic receipt is issued proving the data was verified at a specific time

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
