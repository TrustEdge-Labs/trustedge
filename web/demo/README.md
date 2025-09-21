# TrustEdge P0 WASM Demo

This directory contains a WebAssembly-powered demo for verifying TrustEdge P0 `.trst` archives in the browser.

## 🎯 Features

- **Client-side verification**: No server required - all verification runs in the browser
- **Ed25519 signature verification**: Cryptographic validation of archive signatures
- **Continuity chain checking**: Validates that all expected chunk files are present
- **Directory upload**: Select entire `.trst` directories using modern browser APIs
- **Real-time feedback**: Visual indicators for pass/fail status

## 🚀 Quick Start

### Prerequisites

1. **Rust toolchain** with `wasm-pack` installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install wasm-pack
   ```

2. **Node.js** (for serving the demo locally):
   ```bash
   # Install Node.js from https://nodejs.org or via package manager
   node --version  # Should be v16+
   ```

### Build Steps

1. **Build the WASM module**:
   ```bash
   # From the project root
   wasm-pack build crates/trst-wasm --target web --out-dir ../../web/demo/pkg
   ```

2. **Serve the demo locally**:
   ```bash
   # From web/demo directory
   cd web/demo
   npx serve .
   ```

3. **Open in browser**:
   - Navigate to `http://localhost:3000` (or the URL shown by serve)
   - The demo should load with the TrustEdge verifier interface

### Alternative Build Script

For convenience, you can also run:

```bash
# From project root
./scripts/build-wasm-demo.sh
```

## 📱 Usage

1. **Create a test archive** (if you don't have one):
   ```bash
   # From project root
   head -c 4M </dev/urandom > test-input.bin
   cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in test-input.bin --out test-archive.trst
   ```

2. **Open the demo** in a modern browser (Chrome 86+, Edge 86+ recommended)

3. **Select archive**: Click "Select .trst Archive Directory" and choose your `.trst` folder

4. **Enter public key**: Paste the device public key from `device.pub`

5. **Verify**: Click "Verify Archive" to see the results

## 🔧 Browser Compatibility

| Feature | Chrome | Firefox | Safari | Edge |
|---------|--------|---------|--------|------|
| Directory Selection | 86+ ✅ | ❌ | ❌ | 86+ ✅ |
| WebAssembly | 57+ ✅ | 52+ ✅ | 11+ ✅ | 16+ ✅ |

**Note**: The demo uses the File System Access API for directory selection, which is currently only supported in Chromium-based browsers. Other browsers can still verify individual manifest files.

## 🏗️ Architecture

### WASM Bindings

The WASM module exposes two main functions:

- **`verify_manifest(manifest_bytes, device_pub)`**: Verifies a manifest file directly
- **`verify_archive(dir_handle, device_pub)`**: Verifies a complete archive directory

### Security Model

- **Signature verification**: Uses Ed25519 cryptography to validate manifest signatures
- **Continuity checking**: Ensures all expected chunk files are present (P0 level)
- **No decryption**: This demo only verifies signatures and structure (no data decryption)

### Limitations (P0)

- **Basic continuity**: Only checks file existence, not full hash validation
- **No chunk decryption**: Encrypted chunk contents are not validated
- **Single profile**: Only supports `cam.video` profile

## 🧪 Testing

### Manual Testing

1. Create test archives with different configurations:
   ```bash
   # Valid archive
   cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in test.bin --out valid.trst

   # Test verification
   cargo run -p trustedge-trst-cli -- verify valid.trst --device-pub $(cat device.pub)
   ```

2. Test with wrong public key to verify failure detection

3. Remove chunk files to test continuity checking

### Automated Testing

```bash
# Run WASM-specific tests
wasm-pack test crates/trst-wasm --chrome --headless
```

## 📦 Distribution

To deploy the demo:

1. Build the WASM module: `wasm-pack build crates/trst-wasm --target web`
2. Copy `web/demo/` contents to your web server
3. Ensure proper CORS headers for `.wasm` files
4. Serve with HTTPS for File System Access API support

## 🔍 Troubleshooting

### Build Issues

- **"wasm-pack not found"**: Install with `cargo install wasm-pack`
- **"target not supported"**: Ensure you're using `--target web` flag
- **"out-dir not found"**: Create the directory: `mkdir -p web/demo/pkg`

### Runtime Issues

- **"Directory selection not working"**: Use Chrome/Edge 86+ or test with individual files
- **"WASM module failed to load"**: Check browser console for CORS errors
- **"Verification always fails"**: Ensure public key format starts with `ed25519:`

### Performance

- **Large archives**: The demo is optimized for archives under 100MB
- **Many segments**: Performance may degrade with 1000+ segments
- **File I/O**: Directory scanning can be slow for very large archives

## 📚 Related Documentation

- **[P0 Implementation Status](../../P0_IMPLEMENTATION.md)** - Complete P0 progress
- **[CLI Documentation](../../crates/trst-cli/)** - Command-line interface
- **[Core API Documentation](../../crates/core/)** - Low-level verification APIs
- **[Examples](../../examples/cam.video/)** - End-to-end usage examples

---

*This demo showcases the P0 implementation of TrustEdge .trst archive verification in WebAssembly.*