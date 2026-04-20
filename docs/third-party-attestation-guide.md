<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
This source code is subject to the terms of the Mozilla Public License, v. 2.0.
If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

Project: sealedge — Privacy and trust at the edge.
-->

# Third-Party SBOM Attestation Guide

This guide shows how to add SBOM attestation to any project using Sealedge. The result is a signed JSON document (`.se-attestation.json`) that cryptographically binds your binary to its software bill of materials — verifiable by anyone without contacting Sealedge infrastructure.

**Why:** The EU Cyber Resilience Act requires demonstrable software component provenance. SBOM attestation gives you a tamper-evident record of what your software contains at build time.

## Prerequisites

- **seal binary** — download from [TrustEdge-Labs/sealedge releases](https://github.com/TrustEdge-Labs/sealedge/releases/latest) or build from source:
  ```bash
  cargo install --git https://github.com/TrustEdge-Labs/sealedge sealedge-seal-cli
  ```

- **syft** — SBOM generator from Anchore:
  ```bash
  curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin
  ```

## Manual Workflow

Complete copy-paste flow from zero to verified attestation:

```bash
# 1. Generate Ed25519 device keypair
#    (use encrypted keys in production: omit --unencrypted and set a passphrase)
seal keygen --out-key device.key --out-pub device.pub --unencrypted

# 2. Generate CycloneDX SBOM with syft
syft ./my-binary -o cyclonedx-json > sbom.cdx.json

# 3. Create attestation — binds binary hash + SBOM hash + full SBOM under Ed25519 signature
seal attest-sbom \
  --binary ./my-binary \
  --sbom sbom.cdx.json \
  --device-key device.key \
  --device-pub device.pub \
  --out my-binary.se-attestation.json \
  --unencrypted

# 4. Verify locally (confirms signature and hash integrity before publishing)
seal verify-attestation my-binary.se-attestation.json \
  --device-pub "$(cat device.pub)"

# 5. Upload attestation alongside your release binary
gh release upload v1.0.0 my-binary.se-attestation.json
```

The attestation file is self-contained. Recipients can verify it offline using your public key or via the [public verifier](https://verify.sealedge.dev/verify).

## CI Workflow

### Option A: GitHub Action (recommended)

One line in your workflow:

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build-and-attest:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: cargo build --release

      - name: Generate SBOM
        run: |
          curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin
          syft ./target/release/my-app -o cyclonedx-json > sbom.cdx.json

      - uses: TrustEdge-Labs/attest-sbom-action@v1
        with:
          binary: ./target/release/my-app
          sbom: sbom.cdx.json

      - name: Upload release assets
        run: |
          gh release upload ${{ github.ref_name }} \
            ./target/release/my-app \
            my-app.se-attestation.json
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

The action generates an ephemeral keypair, runs `seal attest-sbom`, and produces `<binary-name>.se-attestation.json` as output.

### Option B: Manual CI steps (any CI provider)

For GitLab CI, CircleCI, Jenkins, or any system where the GitHub Action is unavailable:

```yaml
# GitHub Actions equivalent — adapt syntax for your CI provider
- name: Install seal
  run: |
    curl -L https://github.com/TrustEdge-Labs/sealedge/releases/latest/download/seal-linux-amd64 \
      -o seal
    chmod +x seal

- name: Install syft
  run: |
    curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh \
      | sh -s -- -b /usr/local/bin

- name: Generate SBOM
  run: syft ./target/release/my-app -o cyclonedx-json > sbom.cdx.json

- name: Attest SBOM
  run: |
    ./seal keygen --out-key key.key --out-pub key.pub --unencrypted
    ./seal attest-sbom \
      --binary ./target/release/my-app \
      --sbom sbom.cdx.json \
      --device-key key.key \
      --device-pub key.pub \
      --out attestation.se-attestation.json \
      --unencrypted

- name: Verify attestation
  run: |
    ./seal verify-attestation attestation.se-attestation.json \
      --device-pub "$(cat key.pub)"
```

**Key management note:** Ephemeral keys (generated fresh each build) are the simplest and most secure approach for CI — no secrets to rotate. For long-lived device identity (e.g., firmware signing), generate a persistent key, store the private key as a CI secret, and distribute the public key in your project repository.

## Verification

Recipients of your attestation can verify it three ways:

**1. Local CLI verification:**
```bash
seal verify-attestation my-binary.se-attestation.json --device-pub "ed25519:<your-public-key>"
```

Exit code 0 = valid signature and hash integrity confirmed. Exit code non-zero = failure (see stderr for details).

**2. Public verifier (web UI):**

Visit [https://verify.sealedge.dev/verify](https://verify.sealedge.dev/verify) and paste the contents of your `.se-attestation.json` file. The verifier checks the Ed25519 signature and BLAKE3 hashes and returns a timestamped receipt.

**3. Direct API verification:**
```bash
curl -X POST https://verify.sealedge.dev/v1/verify-attestation \
  -H "Content-Type: application/json" \
  -d @my-binary.se-attestation.json
```

Returns a JSON receipt with `status`, `verified_at`, and `receipt_id`.

## What's in an Attestation

Each `.se-attestation.json` file is a JSON document with these fields:

| Field | Description |
|-------|-------------|
| `format` | `te-point-attestation-v1` — format version discriminant |
| `binary_hash` | BLAKE3 hash of the binary artifact (hex) |
| `sbom_hash` | BLAKE3 hash of the CycloneDX SBOM (hex) |
| `sbom_content` | Full SBOM contents (embedded, base64-encoded) |
| `device_pub` | Ed25519 public key used to sign this attestation |
| `signature` | Ed25519 signature over canonical JSON (base64) |
| `nonce` | Random 16-byte nonce (replay prevention) |
| `timestamp` | RFC 3339 creation timestamp |

The signature covers all fields except `signature` itself — canonical serialization uses stable struct field order. Verifiers need only the attestation file and the expected public key.

## Encrypted Keys for Production

The `--unencrypted` flag is an explicit escape hatch for CI/automation. For production device signing where keys persist across builds:

```bash
# Generate encrypted keypair (passphrase prompted at creation)
seal keygen --out-key device.key --out-pub device.pub

# Sign with encrypted key (passphrase prompted at signing time)
seal attest-sbom --binary ./firmware.bin --sbom sbom.cdx.json \
  --device-key device.key --device-pub device.pub \
  --out firmware.se-attestation.json
```

For hardware-backed keys, the YubiKey PIV backend is available:

```bash
seal attest-sbom --binary ./firmware.bin --sbom sbom.cdx.json \
  --backend yubikey \
  --device-key device.key --device-pub device.pub \
  --out firmware.se-attestation.json
```

Hardware keys keep the private key material inside the YubiKey — it never touches the host filesystem.
