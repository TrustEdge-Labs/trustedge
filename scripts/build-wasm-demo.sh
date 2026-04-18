#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#

set -e

echo "🔧 Building TrustEdge P0 WASM Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ Error: wasm-pack is not installed"
    echo "📦 Install with: cargo install wasm-pack"
    exit 1
fi

# Build the WASM module
echo "📦 Building WASM module..."
wasm-pack build crates/seal-wasm --target web --out-dir ../../web/demo/pkg

# Check if Node.js is available for serving
if command -v npx &> /dev/null; then
    echo "✅ WASM module built successfully!"
    echo ""
    echo "🚀 To serve the demo locally:"
    echo "   cd web/demo"
    echo "   npx serve ."
    echo ""
    echo "📱 Then open your browser to the displayed URL"
else
    echo "✅ WASM module built successfully!"
    echo ""
    echo "⚠️  Node.js not found - you'll need to serve the files manually"
    echo "🚀 Alternative serving options:"
    echo "   python3 -m http.server 8000"
    echo "   php -S localhost:8000"
    echo "   Or any static file server in the web/demo directory"
    echo ""
    echo "📱 Then open your browser to http://localhost:8000"
fi

echo ""
echo "📂 Demo files ready in web/demo/"