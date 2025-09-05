#!/bin/bash
#
# Fast benchmark runner for TrustEdge (Local Development)
# Usage: ./fast-bench.sh [crypto|network|all]
#
# This script runs benchmarks in "fast mode" for quick performance checks
# during development. For full statistical accuracy, use: cargo bench
#

set -e

cd "$(dirname "$0")"

echo "🚀 TrustEdge Fast Benchmarks (Local Development)"
echo "=============================================="
echo "💡 Note: These are quick checks, not statistically rigorous"
echo "   For full accuracy, run: cargo bench"
echo ""

# Set fast benchmark mode
export BENCH_FAST=1

case "${1:-all}" in
    "crypto")
        echo "📊 Running crypto benchmarks (fast mode)..."
        cargo bench --bench crypto_benchmarks
        ;;
    "network")
        echo "🌐 Running network benchmarks (fast mode)..."
        cargo bench --bench network_benchmarks
        ;;
    "all")
        echo "📊 Running all benchmarks (fast mode)..."
        echo "⏱️  Expected runtime: ~45 seconds"
        echo ""
        cargo bench
        ;;
    *)
        echo "Usage: $0 [crypto|network|all]"
        echo ""
        echo "Examples:"
        echo "  $0 crypto    # Fast crypto benchmarks (~30s)"
        echo "  $0 network   # Fast network benchmarks (~15s)"
        echo "  $0 all       # All fast benchmarks (~45s)"
        echo "  $0           # Same as 'all'"
        exit 1
        ;;
esac

echo ""
echo "✅ Fast benchmarks completed!"
echo "💡 For full statistical accuracy, run: cargo bench"
