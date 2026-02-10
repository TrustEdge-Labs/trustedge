#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# scripts/test-inventory.sh — Generate test inventory with per-crate and per-module granularity.
# Includes full test names for baseline diffing across consolidation phases.
#
# Usage:
#   ./scripts/test-inventory.sh                  # Output to stdout
#   ./scripts/test-inventory.sh output.md        # Output to file
#
# Re-run after each phase and diff against TEST-BASELINE.md to detect regressions.

set -euo pipefail

OUTPUT="${1:-/dev/stdout}"
WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT"

TOTAL_TESTS=0
declare -A CRATE_TOTALS

# Get all workspace package names
PACKAGES=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r '.packages[].name' | sort)

{
    echo "# Test Inventory Baseline"
    echo ""
    echo "**Generated:** $(date -u +%Y-%m-%d)"
    echo ""

    for pkg in $PACKAGES; do
        # Get crate directory from manifest path
        manifest_path=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r ".packages[] | select(.name == \"$pkg\") | .manifest_path")
        crate_dir=$(dirname "$manifest_path")
        crate_rel=$(realpath --relative-to="$WORKSPACE_ROOT" "$crate_dir" 2>/dev/null || echo "$crate_dir")

        pkg_total=0
        unit_output=""
        int_output=""

        # Unit tests (lib)
        unit_tests=$(cargo test --package "$pkg" --lib -- --list 2>/dev/null | grep ': test$' || true)
        if [ -n "$unit_tests" ]; then
            # Group by module
            declare -A modules
            while IFS= read -r line; do
                test_name=$(echo "$line" | sed 's/: test$//')
                # Extract module: everything before ::tests:: or last :: segment
                if echo "$test_name" | grep -q '::tests::'; then
                    module=$(echo "$test_name" | sed 's/::tests::.*$//')
                else
                    module=$(echo "$test_name" | rev | cut -d: -f3- | rev)
                    [ -z "$module" ] && module="(root)"
                fi
                modules["$module"]+="$line"$'\n'
            done <<< "$unit_tests"

            unit_count=$(echo "$unit_tests" | wc -l)
            pkg_total=$((pkg_total + unit_count))

            unit_output+="### Unit Tests (lib) — $unit_count tests"$'\n'
            unit_output+=""$'\n'

            # Sort modules and output
            for module in $(echo "${!modules[@]}" | tr ' ' '\n' | sort); do
                mod_tests="${modules[$module]}"
                mod_count=$(echo -n "$mod_tests" | grep -c ': test$' || echo 0)
                unit_output+="#### Module: $module ($mod_count tests)"$'\n'
                unit_output+='```'$'\n'
                unit_output+="$(echo -n "$mod_tests" | sort)"$'\n'
                unit_output+='```'$'\n'
                unit_output+=""$'\n'
            done

            unset modules
            declare -A modules
        fi

        # Integration tests
        tests_dir="$crate_dir/tests"
        if [ -d "$tests_dir" ]; then
            for test_file in "$tests_dir"/*.rs; do
                [ -f "$test_file" ] || continue
                test_name=$(basename "$test_file" .rs)
                # Skip mod.rs and helper files
                [ "$test_name" = "mod" ] && continue

                int_tests=$(cargo test --package "$pkg" --test "$test_name" -- --list 2>/dev/null | grep ': test$' || true)
                if [ -n "$int_tests" ]; then
                    int_count=$(echo "$int_tests" | wc -l)
                    pkg_total=$((pkg_total + int_count))

                    int_output+="### Integration Tests: $test_name — $int_count tests"$'\n'
                    int_output+='```'$'\n'
                    int_output+="$(echo "$int_tests" | sort)"$'\n'
                    int_output+='```'$'\n'
                    int_output+=""$'\n'
                fi
            done
        fi

        # Only output package section if it has tests
        if [ "$pkg_total" -gt 0 ]; then
            echo "## Package: $pkg ($pkg_total tests)"
            echo ""
            [ -n "$unit_output" ] && echo "$unit_output"
            [ -n "$int_output" ] && echo "$int_output"
        fi

        CRATE_TOTALS["$pkg"]=$pkg_total
        TOTAL_TESTS=$((TOTAL_TESTS + pkg_total))
    done

    echo "---"
    echo ""
    echo "## Summary"
    echo ""
    echo "| Package | Tests |"
    echo "|---------|-------|"
    for pkg in $(echo "${!CRATE_TOTALS[@]}" | tr ' ' '\n' | sort); do
        count="${CRATE_TOTALS[$pkg]}"
        [ "$count" -gt 0 ] && echo "| $pkg | $count |"
    done
    echo "| **Total** | **$TOTAL_TESTS** |"
    echo ""
    echo "**Total tests:** $TOTAL_TESTS"

} > "$OUTPUT"

if [ "$OUTPUT" != "/dev/stdout" ]; then
    echo "✔ Test inventory written to $OUTPUT ($TOTAL_TESTS tests)"
else
    echo "" >&2
    echo "✔ Test inventory complete ($TOTAL_TESTS tests)" >&2
fi
