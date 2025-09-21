#!/usr/bin/env bash
set -euo pipefail

# TrustEdge P0 Acceptance Runner
# - Builds release CLI if needed
# - Creates sample input + .trst archive
# - Runs A1–A5 acceptance tests with clear PASS/FAIL reporting

# ----------- Config -----------
TRST_BIN="./target/release/trst"
OUT_DIR="./clip.trst"
SAMPLE="./sample.bin"
CHUNK_SIZE=$((1024*1024))   # 1 MiB
CHUNK_SECONDS="2.0"
FPS="30"
PROFILE="cam.video"
DEVICE_PUB_FILE="./device.pub"
# ------------------------------

# Colors
g() { printf "\033[32m%s\033[0m\n" "$*"; }
r() { printf "\033[31m%s\033[0m\n" "$*"; }
y() { printf "\033[33m%s\033[0m\n" "$*"; }

need() {
  command -v "$1" >/dev/null 2>&1 || { r "Missing dependency: $1"; exit 1; }
}

header() {
  printf "\n%s\n" "===================================================="
  printf "%s\n" "$1"
  printf "%s\n\n" "===================================================="
}

pass() { g "✔ $1"; }
fail() { r "✘ $1"; SUMMARY_FAILS+=("$1"); }

SUMMARY_FAILS=()

# ----------- Checks -----------
need cargo
need sed
# jq is optional; we'll fall back to Python if missing
if ! command -v jq >/dev/null 2>&1; then
  y "jq not found — will use Python for JSON edit in A1"
  need python3
fi

# -------- Build CLI if needed --------
header "Build (release)"
if [[ ! -x "$TRST_BIN" ]]; then
  cargo build --release -p cli
fi
pass "Built: $TRST_BIN"

# -------- Create sample input --------
header "Create sample input"
if [[ ! -f "$SAMPLE" ]]; then
  # 32 MiB random file
  dd if=/dev/urandom of="$SAMPLE" bs=1M count=32 status=none
  pass "Created $SAMPLE (32 MiB)"
else
  y "$SAMPLE already exists — reusing"
fi

# -------- Wrap to .trst --------
header "Wrap → .trst"
rm -rf "$OUT_DIR"
"$TRST_BIN" wrap \
  --profile "$PROFILE" \
  --in "$SAMPLE" \
  --out "$OUT_DIR" \
  --chunk-size "$CHUNK_SIZE" \
  --chunk-seconds "$CHUNK_SECONDS" \
  --fps "$FPS"

test -d "$OUT_DIR/chunks" || { fail "Archive not created correctly (no chunks dir)"; exit 1; }
pass "Archive created at $OUT_DIR"

# -------- Verify (Happy path) --------
header "Verify (happy path)"
if "$TRST_BIN" verify "$OUT_DIR" --device-pub "$(cat "$DEVICE_PUB_FILE")"; then
  pass "Happy path: Signature & Continuity PASS"
else
  fail "Happy path verify FAILED"
fi

# Helpers
copy_archive() {
  local dst="$1"
  rm -rf "$dst"
  cp -R "$OUT_DIR" "$dst"
}

swap_two_chunks() {
  local dir="$1"
  local c1="$2"
  local c2="$3"
  mv "$dir/chunks/$c1" "$dir/chunks/.$c1.tmp"
  mv "$dir/chunks/$c2" "$dir/chunks/$c1"
  mv "$dir/chunks/.$c1.tmp" "$dir/chunks/$c2"
}

# List chunks and derive useful indices
mapfile -t CHUNKS < <(ls "$OUT_DIR/chunks" | sort)
CHUNK_COUNT="${#CHUNKS[@]}"
MID_INDEX=$(( CHUNK_COUNT / 2 ))
# Zero-padded names
MID_NAME=$(printf "%05d.bin" "$MID_INDEX")
IDX10=$(printf "%05d.bin" 10)
IDX11=$(printf "%05d.bin" 11)
LAST_NAME="${CHUNKS[-1]}"

# -------- A1: Signature FAIL (keep JSON valid) --------
header "A1: Signature FAIL (JSON remains valid)"
A1_DIR="./clip_A1_sigfail.trst"
copy_archive "$A1_DIR"

MANIFEST="$A1_DIR/manifest.json"
cp "$MANIFEST" "$A1_DIR/manifest.json.bak"

if command -v jq >/dev/null 2>&1; then
  # Change a benign field (device.model) to keep JSON valid, invalidate signature
  tmp="$(mktemp)"
  jq '.device.model = "TamperedCam"' "$MANIFEST" > "$tmp" && mv "$tmp" "$MANIFEST"
else
  # Python fallback: same change
  python3 - <<'PY' "$MANIFEST"
import json,sys
p=sys.argv[1]
with open(p,'r',encoding='utf-8') as f: m=json.load(f)
if 'device' in m and isinstance(m['device'],dict):
    m['device']['model']='TamperedCam'
else:
    m['device']={'id':'te:cam:XYZ123','fw':'1.0.0','model':'TamperedCam','public_key':m.get('device',{}).get('public_key','')}
with open(p,'w',encoding='utf-8') as f: json.dump(m,f,separators=(',',':'))
PY
fi

if "$TRST_BIN" verify "$A1_DIR" --device-pub "$(cat "$DEVICE_PUB_FILE")"; then
  fail "A1 expected SIGNATURE FAIL, but verify passed"
else
  pass "A1 produced clean signature failure (as expected)"
fi

# -------- A2: Gap detection (delete a middle chunk) --------
header "A2: Continuity FAIL (gap in middle)"
A2_DIR="./clip_A2_gap.trst"
copy_archive "$A2_DIR"
TARGET="$A2_DIR/chunks/$MID_NAME"
if [[ -f "$TARGET" ]]; then
  rm "$TARGET"
else
  # Fallback: remove index 7 if present
  TARGET2="$A2_DIR/chunks/00007.bin"
  [[ -f "$TARGET2" ]] && rm "$TARGET2"
fi

if "$TRST_BIN" verify "$A2_DIR" --device-pub "$(cat "$DEVICE_PUB_FILE")"; then
  fail "A2 expected CONTINUITY FAIL, but verify passed"
else
  pass "A2 reported continuity failure due to missing chunk (as expected)"
fi

# -------- A3: Out-of-order (swap two chunks) --------
header "A3: Continuity FAIL (out-of-order chunks)"
A3_DIR="./clip_A3_shuffle.trst"
copy_archive "$A3_DIR"

# Prefer swapping 00010 and 00011 if they exist; else swap last two
if [[ -f "$A3_DIR/chunks/$IDX10" && -f "$A3_DIR/chunks/$IDX11" ]]; then
  swap_two_chunks "$A3_DIR" "$IDX10" "$IDX11"
else
  # last two names
  mapfile -t A3CH < <(ls "$A3_DIR/chunks" | sort)
  L="${#A3CH[@]}"
  if (( L >= 2 )); then
    swap_two_chunks "$A3_DIR" "${A3CH[$((L-2))]}" "${A3CH[$((L-1))]}"
  fi
fi

if "$TRST_BIN" verify "$A3_DIR" --device-pub "$(cat "$DEVICE_PUB_FILE")"; then
  fail "A3 expected CONTINUITY FAIL (out-of-order), but verify passed"
else
  pass "A3 reported out-of-order continuity failure (as expected)"
fi

# -------- A4: End-of-chain truncation (delete last chunk) --------
header "A4: Continuity FAIL (unexpected end of chain)"
A4_DIR="./clip_A4_truncate.trst"
copy_archive "$A4_DIR"
mapfile -t A4CH < <(ls "$A4_DIR/chunks" | sort)
LAST="${A4CH[-1]}"
rm "$A4_DIR/chunks/$LAST"

if "$TRST_BIN" verify "$A4_DIR" --device-pub "$(cat "$DEVICE_PUB_FILE")"; then
  fail "A4 expected END-OF-CHAIN FAIL, but verify passed"
else
  pass "A4 reported end-of-chain/truncation failure (as expected)"
fi

# -------- A5: Wrong key (signature check must fail) --------
header "A5: Signature FAIL (wrong public key)"
if "$TRST_BIN" verify "$OUT_DIR" --device-pub "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"; then
  fail "A5 expected SIGNATURE FAIL with wrong key, but verify passed"
else
  pass "A5 produced signature failure with wrong pubkey (as expected)"
fi

# -------- Summary --------
header "Summary"
if (( ${#SUMMARY_FAILS[@]} == 0 )); then
  g "All tests passed as expected ✅"
  exit 0
else
  r "Some checks did not behave as expected:"
  for m in "${SUMMARY_FAILS[@]}"; do r " - $m"; done
  exit 1
fi

