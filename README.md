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
[![Version](https://img.shields.io/badge/version-2.0-blue.svg)](https://github.com/TrustEdge-Labs/trustedge/releases/tag/v2.0)
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

All examples use the generic profile (default). For cam.video-specific archives with frame
rate and segment duration, see [examples/cam.video](examples/cam.video/).

## How It Works

1. **Sign** -- Device generates an Ed25519 keypair and signs data at the point of capture
2. **Encrypt** -- Data is chunked and each chunk is encrypted with AES-256-GCM
3. **Wrap** -- Chunks, manifest, and signature are packaged into a `.trst` archive with a BLAKE3 continuity chain
4. **Verify** -- An independent verification service checks the signature, chain integrity, and manifest
5. **Receipt** -- A cryptographic receipt is issued proving the data was verified at a specific time

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
