# TrustEdge Platform Server — DigitalOcean App Platform (verify-only mode)
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# Project: trustedge — Privacy and trust at the edge.
#
# Verify-only mode: no postgres feature, no DATABASE_URL required.
# Serves: POST /v1/verify, POST /v1/verify-attestation, GET /.well-known/jwks.json, GET /healthz
#
# Build:
#   docker build -f deploy/digitalocean/Dockerfile -t trustedge-verifier .
# Run:
#   docker run -p 3001:3001 trustedge-verifier

# ── Stage 1: Builder ──────────────────────────────────────────────────────────

FROM rust:1.88-slim AS builder

WORKDIR /usr/src/app

# Install build-time dependencies (pkg-config + libssl-dev for TLS in transport layer)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy entire workspace source
COPY . .

# Build the platform-server binary in release mode — http feature only, NO postgres
RUN cargo build -p trustedge-platform-server --features http --release

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────

FROM debian:bookworm-slim AS runtime

# Install runtime dependencies: CA certs, libssl3 for TLS, wget for healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for running the server
RUN useradd -r -m -s /bin/false trustedge

# Copy compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/trustedge-platform-server /usr/local/bin/trustedge-platform-server

# Set working directory
WORKDIR /home/trustedge

# Drop to non-root user
USER trustedge

# Platform server HTTP port (must match PORT env var)
EXPOSE 3001

# Health check — DO App Platform also checks /healthz via app.yaml
HEALTHCHECK --interval=30s --timeout=5s CMD wget -qO- http://localhost:3001/healthz || exit 1

# Default command: start the HTTP server
CMD ["trustedge-platform-server", "serve"]
