<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Integration Examples

Real-world integration scenarios and performance examples.

## Integration Examples

### Docker Container Integration

```bash
# Dockerfile for TrustEdge integration
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/trustedge-core /usr/local/bin/
ENTRYPOINT ["trustedge-core"]
```

### CI/CD Pipeline Integration

```yaml
# .github/workflows/secure-build.yml
name: Secure Build with TrustEdge
on: [push]
jobs:
  secure-build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build with TrustEdge
        run: |
          cargo build --release
          ./target/release/trustedge-core \
            --input target/release/my-app \
            --envelope my-app.trst \
            --key-out deploy.key
```

## Performance Examples

### Throughput Benchmarking

```bash
# Large file encryption performance
time ./target/release/trustedge-core \
  --input large_file_1GB.bin \
  --envelope large_file.trst \
  --key-out large.key \
  --verbose

# Network throughput test
time ./target/release/trustedge-client \
  --server 192.168.1.100:8080 \
  --input large_dataset.bin \
  --verbose
```

### Memory Usage Profiling

```bash
# Monitor memory usage during encryption
/usr/bin/time -v ./target/release/trustedge-core \
  --input huge_file.bin \
  --envelope huge.trst \
  --key-out huge.key
```

## Error Handling Examples

### Network Error Recovery

```bash
# Graceful handling of network failures
./target/release/trustedge-client \
  --server unstable-server:8080 \
  --input important.txt \
  --retry-attempts 3 \
  --timeout 10000 \
  --verbose 2>&1 | tee connection.log
```

### File System Error Handling

```bash
# Handle permission errors gracefully
./target/release/trustedge-core \
  --input /protected/file.txt \
  --envelope output.trst \
  --key-out key.hex \
  --verbose 2>&1 || echo "Handle encryption failure"
```

## Real-World Use Cases

### Healthcare Data Protection

```bash
# HIPAA-compliant patient data encryption
./target/release/trustedge-core \
  --input patient_records.xml \
  --envelope secure_records.trst \
  --backend yubikey \
  --slot 9c \
  --pin-prompt \
  --audit-log
```

### Financial Data Processing

```bash
# PCI DSS compliant transaction processing
./target/release/trustedge-core \
  --input transactions.csv \
  --envelope secure_transactions.trst \
  --backend software-hsm \
  --key-derivation pbkdf2 \
  --iterations 100000
```

### Legal Evidence Chain

```bash
# Tamper-evident legal document storage
./target/release/trst wrap \
  --in court_document.pdf \
  --out evidence.trst \
  --device-id "COURT-SYSTEM-01" \
  --metadata "case=12345,date=2025-01-15"
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
