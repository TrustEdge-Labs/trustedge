#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: sealedge — Privacy and trust at the edge.
#
# End-to-end SBOM attestation demo: keygen, syft SBOM, attest, local verify, remote verify (optional)
#
# Usage:
#   ./scripts/demo-attestation.sh                      # Auto-detect: full demo, skip remote if unreachable
#   ./scripts/demo-attestation.sh --local              # Force local-only (skip remote verification)
#   ./scripts/demo-attestation.sh --endpoint <url>     # Override remote endpoint URL

set -uo pipefail

cd "$(dirname "$0")/.."

# ── Timing ────────────────────────────────────────────────────────────────────
START_TIME=$(date +%s)

# ── ANSI colors ───────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
BOLD='\033[1m'
NC='\033[0m'

# ── Argument parsing ──────────────────────────────────────────────────────────
FORCE_LOCAL=false
ENDPOINT="https://verify.trustedge.dev"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --local)
            FORCE_LOCAL=true
            shift
            ;;
        --endpoint)
            if [[ $# -lt 2 ]]; then
                printf "${RED}--endpoint requires a URL argument${NC}\n"
                exit 1
            fi
            ENDPOINT="$2"
            shift 2
            ;;
        *)
            printf "${RED}Unknown flag: %s${NC}\n" "$1"
            printf "Usage: %s [--local] [--endpoint <url>]\n" "$0"
            exit 1
            ;;
    esac
done

# ── Step state ────────────────────────────────────────────────────────────────
STEP=0
FAILURES=0
TOTAL_STEPS=7

# ── Step helpers ──────────────────────────────────────────────────────────────
step_banner() {
    STEP=$((STEP + 1))
    printf "\n${BOLD}${BLUE}[Step %d/%d] %s${NC}\n" "$STEP" "$TOTAL_STEPS" "$1"
}

pass() { printf "  ${GREEN}✔ %s${NC}\n" "$1"; }
fail() { printf "  ${RED}✖ %s${NC}\n" "$1"; FAILURES=$((FAILURES + 1)); }
warn() { printf "  ${YELLOW}⚠ %s${NC}\n" "$1"; }

# ── Header ────────────────────────────────────────────────────────────────────
printf "\n${BOLD}● TrustEdge SBOM Attestation Demo${NC}\n"
printf "  Flow: keygen → syft SBOM → attest → local verify → remote verify (optional)\n"
if $FORCE_LOCAL; then
    printf "  Mode: local-only (remote verification skipped)\n"
else
    printf "  Remote endpoint: %s\n" "$ENDPOINT"
fi

# ── Output directory ──────────────────────────────────────────────────────────
DEMO_DIR="demo-attestation-output"
rm -rf "$DEMO_DIR" && mkdir -p "$DEMO_DIR"

# ── seal runner ───────────────────────────────────────────────────────────────
SEAL="cargo run -q -p sealedge-seal-cli --"

# ── Step 1: Check prerequisites ───────────────────────────────────────────────
step_banner "Check prerequisites"

PREREQS_OK=true

if command -v cargo &>/dev/null; then
    CARGO_VERSION=$(cargo --version 2>/dev/null || echo "unknown")
    pass "cargo found: $CARGO_VERSION"
else
    fail "cargo not found — install Rust: https://rustup.rs/"
    PREREQS_OK=false
fi

if command -v syft &>/dev/null; then
    SYFT_VERSION=$(syft --version 2>/dev/null || echo "unknown")
    pass "syft found: $SYFT_VERSION"
else
    fail "syft not found — install syft:"
    printf "        curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin\n"
    PREREQS_OK=false
fi

if ! $PREREQS_OK; then
    printf "\n${RED}Prerequisites check failed. Install missing tools and re-run.${NC}\n\n"
    exit 1
fi

# ── Step 2: Generate Ed25519 device key pair ──────────────────────────────────
step_banner "Generate Ed25519 device key pair"

if $SEAL keygen \
        --out-key "$DEMO_DIR/device.key" \
        --out-pub "$DEMO_DIR/device.pub" \
        --unencrypted 2>&1; then
    pass "Created $DEMO_DIR/device.key and $DEMO_DIR/device.pub"
    DEVICE_PUB=$(cat "$DEMO_DIR/device.pub" | tr -d '\n')
    printf "  Public key: %s\n" "$DEVICE_PUB"
else
    fail "Key generation failed"
    DEVICE_PUB=""
fi

# ── Step 3: Build the seal binary (for self-attestation) ─────────────────────
step_banner "Build seal binary (self-attestation target)"

printf "  Building sealedge-seal-cli (this may take a moment on first build)...\n"
if cargo build -p sealedge-seal-cli --release 2>&1; then
    SEAL_BINARY="target/release/seal"
    if [ -f "$SEAL_BINARY" ]; then
        BINARY_SIZE=$(du -sh "$SEAL_BINARY" 2>/dev/null | cut -f1)
        pass "Built $SEAL_BINARY ($BINARY_SIZE)"
    else
        fail "Build succeeded but binary not found at $SEAL_BINARY"
        SEAL_BINARY=""
    fi
else
    fail "Build failed"
    SEAL_BINARY=""
fi

# ── Step 4: Generate SBOM with syft ──────────────────────────────────────────
step_banner "Generate SBOM with syft (CycloneDX JSON)"

SBOM_PATH="$DEMO_DIR/sbom.cdx.json"
if [ -n "${SEAL_BINARY:-}" ] && [ -f "${SEAL_BINARY:-}" ]; then
    printf "  Running syft on %s (may take 10-20 seconds)...\n" "$SEAL_BINARY"
    if syft "$SEAL_BINARY" -o cyclonedx-json > "$SBOM_PATH" 2>/dev/null; then
        SBOM_SIZE=$(wc -c < "$SBOM_PATH" 2>/dev/null || echo "0")
        COMPONENT_COUNT=$(grep -c '"type"' "$SBOM_PATH" 2>/dev/null || echo "?")
        pass "Generated $SBOM_PATH (${SBOM_SIZE} bytes, ~${COMPONENT_COUNT} type entries)"
    else
        fail "syft SBOM generation failed"
        SBOM_PATH=""
    fi
else
    fail "Cannot generate SBOM — build step failed"
    SBOM_PATH=""
fi

# ── Step 5: Create attestation ────────────────────────────────────────────────
step_banner "Create SBOM attestation"

ATTESTATION_PATH="$DEMO_DIR/attestation.se-attestation.json"
if [ -n "${SEAL_BINARY:-}" ] && [ -f "${SEAL_BINARY:-}" ] && \
   [ -n "${SBOM_PATH:-}" ] && [ -f "${SBOM_PATH:-}" ] && \
   [ -n "${DEVICE_PUB:-}" ]; then
    if $SEAL attest-sbom \
            --binary "$SEAL_BINARY" \
            --sbom "$SBOM_PATH" \
            --device-key "$DEMO_DIR/device.key" \
            --device-pub "$DEMO_DIR/device.pub" \
            --out "$ATTESTATION_PATH" \
            --unencrypted 2>&1; then
        ATTEST_SIZE=$(wc -c < "$ATTESTATION_PATH" 2>/dev/null || echo "0")
        pass "Created $ATTESTATION_PATH (${ATTEST_SIZE} bytes)"
    else
        fail "attest-sbom failed"
        ATTESTATION_PATH=""
    fi
else
    fail "Cannot attest — one or more prior steps failed (build, SBOM, or keygen)"
    ATTESTATION_PATH=""
fi

# ── Step 6: Verify attestation locally ───────────────────────────────────────
step_banner "Verify attestation locally"

if [ -n "${ATTESTATION_PATH:-}" ] && [ -f "${ATTESTATION_PATH:-}" ] && [ -n "${DEVICE_PUB:-}" ]; then
    if $SEAL verify-attestation "$ATTESTATION_PATH" --device-pub "$DEVICE_PUB" 2>&1; then
        pass "Local verification PASSED — signature valid, hashes match"
    else
        fail "Local verification FAILED"
    fi
else
    fail "Cannot verify — attestation step failed"
fi

# ── Step 7: Remote verification (optional) ────────────────────────────────────
step_banner "Remote verification (optional)"

if $FORCE_LOCAL; then
    warn "[Skipped] Remote verification (--local flag set)"
elif [ -z "${ATTESTATION_PATH:-}" ] || [ ! -f "${ATTESTATION_PATH:-}" ]; then
    warn "[Skipped] Remote verification (no attestation file — prior step failed)"
else
    # Auto-detect endpoint availability
    if curl -sf "${ENDPOINT}/healthz" > /dev/null 2>&1; then
        printf "  Endpoint reachable at %s\n" "$ENDPOINT"
        REMOTE_RESULT=$(curl -sf -X POST "${ENDPOINT}/v1/verify-attestation" \
            -H "Content-Type: application/json" \
            -d @"$ATTESTATION_PATH" 2>&1) || true
        if [ $? -eq 0 ] && [ -n "$REMOTE_RESULT" ]; then
            pass "Remote verification PASSED"
            printf "  Receipt: %s\n" "$(echo "$REMOTE_RESULT" | head -c 200)"
        else
            fail "Remote verification request failed"
        fi
    else
        warn "[Skipped] Remote verification (endpoint not reachable at $ENDPOINT)"
        printf "  To deploy your own verifier: see deploy/digitalocean/\n"
    fi
fi

# ── Step 8: Summary ───────────────────────────────────────────────────────────
STEP=$((STEP + 1))
printf "\n${BOLD}${BLUE}[Step %d/%d] Summary${NC}\n" "$STEP" "$((TOTAL_STEPS + 1))"

printf "\n  Artifacts in %s/:\n" "$DEMO_DIR"
ls -la "$DEMO_DIR/" 2>/dev/null | tail -n +2 | while IFS= read -r line; do
    printf "    %s\n" "$line"
done

# ── Timing ────────────────────────────────────────────────────────────────────
END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

printf "\n  Elapsed time: %ds\n" "$ELAPSED"

# ── Final banner ──────────────────────────────────────────────────────────────
if [ "$FAILURES" -eq 0 ]; then
    printf "\n${BOLD}${GREEN}DEMO COMPLETE — ALL PASSED${NC} (${ELAPSED}s)\n\n"
    exit 0
else
    printf "\n${BOLD}${RED}DEMO FAILED — %d step(s) failed${NC} (${ELAPSED}s)\n\n" "$FAILURES"
    exit 1
fi
