# TrustEdge Platform Server — Multi-stage Docker build
#
# Stage 1 (builder): Compiles the trustedge-platform-server binary from source
# Stage 2 (runtime): Minimal debian-slim image with just the binary
#
# Note: If you maintain a .dockerignore at repo root, exclude target/ and .git/
# to avoid sending those to the Docker build context (can be several GB).
#
# Build:
#   docker build -f deploy/Dockerfile -t trustedge-platform-server .
# Run:
#   docker run -e DATABASE_URL=postgres://... -e PORT=3001 trustedge-platform-server

# ── Stage 1: Builder ──────────────────────────────────────────────────────────

FROM rust:1.82-slim AS builder

WORKDIR /usr/src/app

# Install build-time dependencies for sqlx TLS (native-tls backend)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy entire workspace source
COPY . .

# Build only the platform-server binary in release mode
# postgres feature is enabled by default in Cargo.toml
RUN cargo build -p trustedge-platform-server --release

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────

FROM debian:bookworm-slim AS runtime

# Install runtime dependencies: CA certificates and libssl3 for TLS postgres connections
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for running the server
RUN useradd -r -s /bin/false trustedge

# Copy compiled binary from builder stage
COPY --from=builder /usr/src/app/target/release/trustedge-platform-server /usr/local/bin/trustedge-platform-server

# Drop to non-root user
USER trustedge

# Platform server HTTP port (must match PORT env var)
EXPOSE 3001

# Default command: start the HTTP server
# Use 'migrate' subcommand to run migrations before first serve
CMD ["trustedge-platform-server", "serve"]
