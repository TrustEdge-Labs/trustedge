#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: sealedge — Privacy and trust at the edge.
#
# End-to-end sealedge demo: keygen, wrap, local verify, server verify (optional)
#
# Usage:
#   ./scripts/demo.sh           # Auto-detect: full demo if server running, local-only otherwise
#   ./scripts/demo.sh --local   # Force local-only mode (skip server verification)
#   ./scripts/demo.sh --docker  # Force docker mode (error if server not running)

set -uo pipefail

cd "$(dirname "$0")/.."

# ── ANSI colors ───────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# ── Argument parsing ──────────────────────────────────────────────────────────
FORCE_LOCAL=false
FORCE_DOCKER=false
for arg in "$@"; do
    case "$arg" in
        --local)  FORCE_LOCAL=true ;;
        --docker) FORCE_DOCKER=true ;;
        *)
            printf "${RED}Unknown flag: %s${NC}\n" "$arg"
            printf "Usage: %s [--local|--docker]\n" "$0"
            exit 1
            ;;
    esac
done

if $FORCE_LOCAL && $FORCE_DOCKER; then
    printf "${RED}Cannot specify both --local and --docker${NC}\n"
    exit 1
fi

# ── Mode detection ────────────────────────────────────────────────────────────
SEAL="cargo run -q -p sealedge-seal-cli --"

SERVER_AVAILABLE=false
if ! $FORCE_LOCAL; then
    if curl -sf http://localhost:3001/healthz > /dev/null 2>&1; then
        SERVER_AVAILABLE=true
    elif $FORCE_DOCKER; then
        printf "${RED}✖ Platform server not reachable at http://localhost:3001${NC}\n"
        printf "  Start the stack: docker compose -f deploy/docker-compose.yml up --build\n"
        exit 1
    fi
fi

# ── YubiKey detection ─────────────────────────────────────────────────────────
YUBIKEY_AVAILABLE=false
if command -v ykman &>/dev/null; then
    if ykman list 2>/dev/null | grep -q "YubiKey"; then
        YUBIKEY_AVAILABLE=true
    fi
fi

# ── Output directory ──────────────────────────────────────────────────────────
DEMO_DIR="demo-output"
rm -rf "$DEMO_DIR" && mkdir -p "$DEMO_DIR"

# ── Step state ────────────────────────────────────────────────────────────────
STEP=0
FAILURES=0
TOTAL_STEPS=5
if $SERVER_AVAILABLE; then TOTAL_STEPS=$((TOTAL_STEPS + 1)); fi
if $YUBIKEY_AVAILABLE; then TOTAL_STEPS=$((TOTAL_STEPS + 1)); fi

# ── Step helpers ──────────────────────────────────────────────────────────────
step_banner() {
    STEP=$((STEP + 1))
    printf "\n${BOLD}${BLUE}[Step %d/%d] %s${NC}\n" "$STEP" "$TOTAL_STEPS" "$1"
}

pass() { printf "  ${GREEN}✔ %s${NC}\n" "$1"; }
fail() { printf "  ${RED}✖ %s${NC}\n" "$1"; FAILURES=$((FAILURES + 1)); }

# ── Header ────────────────────────────────────────────────────────────────────
printf "\n${BOLD}● Sealedge End-to-End Demo${NC}\n"
if $SERVER_AVAILABLE; then
    printf "  Mode: full (keygen + wrap + local verify + server verify)\n"
else
    printf "  Mode: local-only (keygen + wrap + local verify)\n"
    printf "  Tip:  Start docker stack for server verification:\n"
    printf "        docker compose -f deploy/docker-compose.yml up --build\n"
fi

# ── Step 1: Generate Ed25519 device key pair ──────────────────────────────────
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

# ── Step 2: Generate sample data ─────────────────────────────────────────────
step_banner "Generate sample data"
if dd if=/dev/urandom of="$DEMO_DIR/sample.bin" bs=1K count=10 2>/dev/null; then
    pass "Created $DEMO_DIR/sample.bin (10 KB random data)"
else
    fail "Failed to generate sample data"
fi

# ── Step 3: Wrap data into .seal archive ──────────────────────────────────────
step_banner "Wrap data into .seal archive"
if $SEAL wrap \
        --profile generic \
        --in "$DEMO_DIR/sample.bin" \
        --out "$DEMO_DIR/sample.seal" \
        --device-key "$DEMO_DIR/device.key" \
        --device-pub "$DEMO_DIR/device.pub" \
        --data-type "sensor" \
        --source "demo-device-01" \
        --description "Demo sensor data capture" \
        --unencrypted 2>&1; then
    pass "Created $DEMO_DIR/sample.seal archive"
else
    fail "Wrap failed"
fi

# ── Step 4: Local verification ────────────────────────────────────────────────
step_banner "Verify archive locally"
if [ -n "${DEVICE_PUB:-}" ] && [ -f "$DEMO_DIR/sample.seal/manifest.json" ]; then
    if $SEAL verify "$DEMO_DIR/sample.seal" --device-pub "$DEVICE_PUB" 2>&1; then
        pass "Local verification PASSED"
    else
        fail "Local verification FAILED"
    fi
else
    fail "Cannot verify — keygen or wrap step failed"
fi

# ── Step 5: YubiKey hardware signing (only if YubiKey detected) ──────────────
if $YUBIKEY_AVAILABLE; then
    step_banner "Sign archive with YubiKey (ECDSA P-256)"
    SEAL_YUBIKEY="cargo run -q -p sealedge-seal-cli --features yubikey --"
    if $SEAL_YUBIKEY wrap \
            --backend yubikey \
            --profile generic \
            --in "$DEMO_DIR/sample.bin" \
            --out "$DEMO_DIR/sample-yubikey.seal" \
            --device-key "$DEMO_DIR/device.key" \
            --data-type "sensor" \
            --source "demo-yubikey" \
            --description "YubiKey hardware-signed demo" \
            --unencrypted 2>&1; then
        pass "Created $DEMO_DIR/sample-yubikey.seal (YubiKey ECDSA P-256 signed)"
        # Verify the YubiKey-signed archive
        YUBIKEY_PUB=$(jq -r '.device.public_key' "$DEMO_DIR/sample-yubikey.seal/manifest.json")
        if $SEAL verify "$DEMO_DIR/sample-yubikey.seal" --device-pub "$YUBIKEY_PUB" 2>&1; then
            pass "YubiKey archive verification PASSED"
        else
            fail "YubiKey archive verification FAILED"
        fi
    else
        fail "YubiKey wrap failed"
    fi
else
    printf "\n  ${BOLD}[Skipped]${NC} YubiKey hardware signing (no YubiKey detected)\n"
    printf "  Insert a YubiKey to see hardware signing.\n"
fi

# ── Step N: Server verification (only if server available) ────────────────────
if $SERVER_AVAILABLE; then
    step_banner "Submit to platform verification server"
    if $SEAL emit-request \
            --archive "$DEMO_DIR/sample.seal" \
            --device-pub "$DEMO_DIR/device.pub" \
            --out "$DEMO_DIR/verify-request.json" \
            --post http://localhost:3001/v1/verify 2>&1; then
        pass "Server verification complete — receipt saved to $DEMO_DIR/verify-request.json"
    else
        fail "Server verification failed"
    fi
else
    printf "\n  ${BOLD}[Skipped]${NC} Server verification (platform not running)\n"
    printf "  Start the stack: docker compose -f deploy/docker-compose.yml up --build\n"
fi

# ── Step N: Summary ───────────────────────────────────────────────────────────
step_banner "Summary"
printf "\n  Artifacts in %s/:\n" "$DEMO_DIR"
ls -la "$DEMO_DIR/" 2>/dev/null | tail -n +2 | while IFS= read -r line; do
    printf "    %s\n" "$line"
done

# ── Final banner ──────────────────────────────────────────────────────────────
if [ "$FAILURES" -eq 0 ]; then
    printf "\n${BOLD}${GREEN}DEMO COMPLETE — ALL PASSED${NC}\n\n"
    exit 0
else
    printf "\n${BOLD}${RED}DEMO FAILED — %d step(s) failed${NC}\n\n" "$FAILURES"
    exit 1
fi
