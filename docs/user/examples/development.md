<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Development Examples

Development workflows and project management examples.

## Development and Project Management Examples

### Development Workflow Integration

```bash
# Pre-commit hook for secure builds
#!/bin/bash
# .git/hooks/pre-commit

# Build and test
cargo build --release
cargo test

# Create attestation for development build
./target/release/trustedge-attest \
  --file target/release/my-app \
  --builder-id "dev-$(whoami)" \
  --output dev-attestation.trst

echo "✔ Development build attested"
```

### Debugging and Troubleshooting

```bash
# Debug mode with verbose logging
RUST_LOG=debug ./target/release/trustedge-core \
  --input test.txt \
  --envelope debug.trst \
  --key-out debug.key \
  --verbose

# Inspect internal state
./target/release/trustedge-core \
  --input debug.trst \
  --inspect \
  --verbose \
  --debug-chunks
```

### Testing Workflows

```bash
# Automated testing script
#!/bin/bash
set -e

echo "Running TrustEdge integration tests..."

# Test basic encryption/decryption
echo "Test data" > test_input.txt
./target/release/trustedge-core \
  --input test_input.txt \
  --envelope test.trst \
  --key-out test.key

./target/release/trustedge-core \
  --decrypt \
  --input test.trst \
  --out test_output.txt \
  --key-hex $(cat test.key)

# Verify integrity
if diff test_input.txt test_output.txt; then
  echo "✔ Basic encryption test passed"
else
  echo "✖ Basic encryption test failed"
  exit 1
fi

# Cleanup
rm test_input.txt test.trst test.key test_output.txt
echo "✔ All tests passed"
```

### Release Management

```bash
# Release preparation script
#!/bin/bash
VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

# Build release
cargo build --release --all-features

# Create attestations for all binaries
for binary in trustedge-core trustedge-server trustedge-client trst; do
  ./target/release/trustedge-attest \
    --file "target/release/$binary" \
    --builder-id "release-$VERSION" \
    --output "release/$binary-$VERSION.trst"
done

echo "✔ Release $VERSION prepared with attestations"
```

### Monitoring and Metrics

```bash
# Performance monitoring during development
#!/bin/bash

# Monitor encryption performance
echo "Monitoring encryption performance..."
for size in 1M 10M 100M; do
  dd if=/dev/urandom of="test_$size.bin" bs=1 count=$size 2>/dev/null

  start_time=$(date +%s.%N)
  ./target/release/trustedge-core \
    --input "test_$size.bin" \
    --envelope "test_$size.trst" \
    --key-out "key_$size.hex" \
    --quiet
  end_time=$(date +%s.%N)

  duration=$(echo "$end_time - $start_time" | bc)
  echo "$size file: ${duration}s"

  # Cleanup
  rm "test_$size.bin" "test_$size.trst" "key_$size.hex"
done
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
