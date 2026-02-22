-- Multi-tenant schema for TrustEdge Platform API
-- Copyright (c) 2025 TRUSTEDGE LABS LLC

-- Organizations table (tenants)
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    plan TEXT NOT NULL DEFAULT 'free',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    email TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- API keys for authentication
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Device registry per organization
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    device_id TEXT NOT NULL,
    device_pub TEXT NOT NULL,
    label TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(org_id, device_id)
);

-- Verification results
CREATE TABLE verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    manifest_digest TEXT NOT NULL,
    result_json JSONB NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Receipt storage
CREATE TABLE receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    verification_id UUID NOT NULL REFERENCES verifications(id) ON DELETE CASCADE,
    jws TEXT NOT NULL,
    kid TEXT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Policies for organizations
CREATE TABLE policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    rules_json JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_organizations_name ON organizations(name);
CREATE INDEX idx_users_org_id ON users(org_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_api_keys_org_id ON api_keys(org_id);
CREATE INDEX idx_api_keys_token_hash ON api_keys(token_hash);
CREATE INDEX idx_devices_org_id ON devices(org_id);
CREATE INDEX idx_devices_device_id ON devices(org_id, device_id);
CREATE INDEX idx_verifications_org_id ON verifications(org_id);
CREATE INDEX idx_verifications_device_id ON verifications(device_id);
CREATE INDEX idx_verifications_issued_at ON verifications(issued_at);
CREATE INDEX idx_receipts_verification_id ON receipts(verification_id);
CREATE INDEX idx_policies_org_id ON policies(org_id);
