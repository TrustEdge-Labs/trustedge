<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Software Attestation Examples

Create and verify cryptographically signed software "birth certificates" for supply chain security.

## Basic Attestation Workflow

Create and verify cryptographically signed software "birth certificates":

```bash
# Build your application
cargo build --release

# Create an attestation for the binary
trustedge-attest --file target/release/my-app \
                 --builder-id "developer@company.com" \
                 --output my-app.trst \
                 --verbose

# Example output:
# ● TrustEdge Software Attestation Tool
# =====================================
# ● Artifact: target/release/my-app
# ● Builder: developer@company.com
# ● Output: my-app.trst
# ● Analyzing artifact and repository...
# ✔ Attestation created:
#    ● Artifact: my-app
#    ● Hash: a1b2c3d4e5f6789a...
#    ● Commit: 0a9a9c9fa2e8b1c4...
#    ● Timestamp: 2025-09-19T14:30:00Z
# ✔ Sealed attestation created: my-app.trst

# Verify the attestation
trustedge-verify --artifact target/release/my-app \
                 --attestation-file my-app.trst \
                 --verbose

# Example output:
# ● TrustEdge Attestation Verification Tool
# ==========================================
# ● Artifact: target/release/my-app
# ● Attestation: my-app.trst
# ● Reading attestation (trying envelope first, JSON fallback)...
# ● Computing artifact hash...
# ✔ VERIFICATION SUCCESSFUL
# ● Artifact Details:
#    • Name: my-app
#    • Hash: a1b2c3d4e5f6789a...
#    • Size: 2456789 bytes
# ● Provenance Information:
#    • Source Commit: 0a9a9c9fa2e8b1c4d8f2e1a6b9c5...
#    • Builder ID: developer@company.com
#    • Created: 2025-09-19T14:30:00Z
# ✔ This software artifact is AUTHENTICATED and VERIFIED
```

## CI/CD Integration Example

Integrate attestation into your CI/CD pipeline:

```bash
#!/bin/bash
# .github/workflows/release.yml or similar

# Build the release
cargo build --release

# Get CI environment info
CI_JOB_ID="${GITHUB_RUN_ID}-${GITHUB_RUN_NUMBER}"
ARTIFACT_NAME="my-app-${GITHUB_REF_NAME}"

# Create attestation with CI context
trustedge-attest --file "target/release/my-app" \
                 --builder-id "ci-job-${CI_JOB_ID}" \
                 --output "${ARTIFACT_NAME}.trst" \
                 --verbose

# Upload both artifact and attestation
aws s3 cp "target/release/my-app" "s3://releases/${ARTIFACT_NAME}"
aws s3 cp "${ARTIFACT_NAME}.trst" "s3://releases/${ARTIFACT_NAME}.trst"

echo "✔ Release ${ARTIFACT_NAME} uploaded with attestation"
```

## Supply Chain Verification

Verify software throughout the supply chain:

```bash
# Download artifact and attestation
aws s3 cp "s3://releases/my-app-v1.0.0" ./my-app
aws s3 cp "s3://releases/my-app-v1.0.0.trst" ./my-app.trst

# Verify integrity and provenance
trustedge-verify --artifact my-app \
                 --attestation-file my-app.trst \
                 --verbose

# Check exit code for automation
if [ $? -eq 0 ]; then
    echo "✔ Software verification PASSED - safe to deploy"
    chmod +x my-app
    ./my-app
else
    echo "✖ Software verification FAILED - DO NOT DEPLOY"
    exit 1
fi
```

## JSON Inspection Mode

Create JSON-only attestations for debugging and inspection:

```bash
# Create JSON attestation (no cryptographic envelope)
trustedge-attest --file target/release/my-app \
                 --builder-id "debug-build" \
                 --output attestation.json \
                 --json-only

# Inspect the raw attestation data
cat attestation.json | jq .

# Example output:
# {
#   "artifact_hash": "a1b2c3d4e5f6789ab1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
#   "artifact_name": "my-app",
#   "source_commit_hash": "0a9a9c9fa2e8b1c4d8f2e1a6b9c5a4d7f8e3b2c1",
#   "builder_id": "debug-build",
#   "timestamp": "2025-09-19T14:30:00Z"
# }

# Verify JSON attestation
trustedge-verify --artifact target/release/my-app \
                 --attestation-file attestation.json \
                 --json-input
```

## Multi-Platform Release Attestation

Create attestations for multiple build targets:

```bash
#!/bin/bash
# Multi-platform build and attestation script

TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-pc-windows-gnu")
BUILDER_ID="release-automation-v1.0.0"

for target in "${TARGETS[@]}"; do
    echo "Building for target: $target"

    # Build for specific target
    cargo build --release --target "$target"

    # Create attestation for this target
    trustedge-attest --file "target/${target}/release/my-app" \
                     --builder-id "${BUILDER_ID}-${target}" \
                     --output "releases/my-app-${target}.trst" \
                     --verbose

    echo "✔ Attestation created for $target"
done

echo "✔ All platform attestations created in releases/"
ls -la releases/
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
