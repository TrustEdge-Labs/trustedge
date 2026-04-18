# TrustEdge Verifier — DigitalOcean App Platform Deployment
<!-- Copyright (c) 2025 TRUSTEDGE LABS LLC - MPL-2.0: https://mozilla.org/MPL/2.0/ -->

Verify-only platform server. No database. No auth. Stateless.

## Prerequisites

- `doctl` CLI installed and authenticated (`doctl auth init`)
- GitHub repo connected to your DigitalOcean account

## Deploy

```bash
doctl apps create --spec deploy/digitalocean/app.yaml
```

Build takes ~5-10 minutes on first deploy (Rust compile time).

## Verify Deployment

```bash
# Health check
curl https://<app-url>/healthz

# Verify page (browser)
open https://<app-url>/verify

# API smoke test
curl -X POST https://<app-url>/v1/verify-attestation \
  -d @path/to/attestation.se-attestation.json
```

## Update

Push to `main` triggers auto-deploy (`deploy_on_push: true` in app.yaml).

To update the App Platform spec:
```bash
doctl apps update <app-id> --spec deploy/digitalocean/app.yaml
```

## Local Testing

```bash
# Build the verify-only image
docker build -f deploy/digitalocean/Dockerfile -t trustedge-verifier .

# Run locally
docker run -p 3001:3001 trustedge-verifier

# Verify
curl http://localhost:3001/healthz
open http://localhost:3001/verify
```

## Environment Variables

See `deploy/digitalocean/.env.example`. Key vars (set in app.yaml): `PORT`, `RECEIPT_TTL_SECS`, `RATE_LIMIT_RPS`, `RUST_LOG`.
No `DATABASE_URL` needed — verify-only mode uses an in-memory backend.
