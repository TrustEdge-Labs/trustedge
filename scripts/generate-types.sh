#!/usr/bin/env bash
# scripts/generate-types.sh
#
# Generate TypeScript interfaces from trustedge-types JSON Schema fixtures.
# Output: web/dashboard/src/lib/types.ts
#
# Usage: bash scripts/generate-types.sh
# Re-run whenever Rust types change and snapshot tests have been updated.
#
# Requires:
#   - Node.js + npx
#   - json-schema-to-typescript (installed as devDependency in web/dashboard)
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
FIXTURES_DIR="${REPO_ROOT}/crates/types/tests/fixtures"
DASHBOARD_LIB="${REPO_ROOT}/web/dashboard/src/lib"
OUTPUT_FILE="${DASHBOARD_LIB}/types.ts"
NPX_PREFIX="${REPO_ROOT}/web/dashboard"

# Verify npx is available
if ! command -v npx &>/dev/null; then
  echo "ERROR: npx is required (install Node.js + npm)" >&2
  exit 1
fi

# Verify json-schema-to-typescript is installed in web/dashboard
if [ ! -f "${NPX_PREFIX}/node_modules/.bin/json2ts" ]; then
  echo "ERROR: json-schema-to-typescript not found." >&2
  echo "  Run: cd web/dashboard && npm install" >&2
  exit 1
fi

# Verify fixture files exist
FIXTURE_FILES=(
  "${FIXTURES_DIR}/verify_report.v1.json"
  "${FIXTURES_DIR}/receipt.v1.json"
  "${FIXTURES_DIR}/verify_request.v1.json"
  "${FIXTURES_DIR}/verify_response.v1.json"
)

for f in "${FIXTURE_FILES[@]}"; do
  if [ ! -f "${f}" ]; then
    echo "ERROR: Missing fixture file: ${f}" >&2
    exit 1
  fi
done

echo "Generating TypeScript types from trustedge-types JSON Schema fixtures..."

# Generate TypeScript for each schema, collect raw output into a temp file
TMPFILE="$(mktemp /tmp/ts-raw-XXXXXX.ts)"
trap 'rm -f "${TMPFILE}"' EXIT

for schema in "${FIXTURE_FILES[@]}"; do
  fname="$(basename "${schema}")"
  echo "  Processing ${fname}..."
  npx --prefix "${NPX_PREFIX}" json2ts \
    -i "${schema}" \
    --bannerComment "" \
    2>/dev/null >> "${TMPFILE}"
  echo "" >> "${TMPFILE}"
done

# Deduplicate interfaces using Node.js.
# Some definitions (e.g. VerifyReport, OutOfOrder) appear in multiple schemas
# because json-schema-to-typescript inlines $ref definitions per-schema.
# This deduplicates by keeping the first occurrence of each interface.
DEDUP_SCRIPT="$(mktemp /tmp/dedup-XXXXXX.mjs)"
trap 'rm -f "${TMPFILE}" "${DEDUP_SCRIPT}"' EXIT

cat > "${DEDUP_SCRIPT}" << 'NODESCRIPT'
import { readFileSync } from "fs";

const inputFile = process.argv[2];
const raw = readFileSync(inputFile, "utf8");

// Split into interface blocks: each block starts with "export interface"
const lines = raw.split("\n");
const blocks = [];
let current = [];
let inBlock = false;

for (const line of lines) {
  if (line.startsWith("export interface ")) {
    // Save the previous block if we were in one
    if (current.length > 0) {
      blocks.push(current.join("\n").trimEnd());
      current = [];
    }
    inBlock = true;
    current.push(line);
  } else if (inBlock) {
    current.push(line);
  }
  // Lines before any interface block are ignored (empty lines between schemas)
}
if (current.length > 0 && inBlock) {
  blocks.push(current.join("\n").trimEnd());
}

// Deduplicate: keep first occurrence of each interface name
const seen = new Set();
const unique = [];
for (const block of blocks) {
  const match = block.match(/^export interface (\w+)/);
  if (match) {
    const name = match[1];
    if (!seen.has(name)) {
      seen.add(name);
      unique.push(block);
    }
  }
}

process.stdout.write(unique.join("\n\n") + "\n");
NODESCRIPT

DEDUPED="$(node "${DEDUP_SCRIPT}" "${TMPFILE}")"

# Write final output with header
{
  printf '// AUTO-GENERATED from trustedge-types JSON schemas.\n'
  printf '// Do not edit manually. Run scripts/generate-types.sh to regenerate.\n'
  printf '//\n'
  printf '// Source: crates/types/tests/fixtures/*.json\n'
  printf '// Generator: json-schema-to-typescript (via npx)\n'
  printf '\n'
  printf '%s\n' "${DEDUPED}"
} > "${OUTPUT_FILE}"

echo ""
echo "Generated: ${OUTPUT_FILE}"
echo "Interfaces exported:"
grep '^export interface' "${OUTPUT_FILE}" | sed 's/export interface /  - /'
