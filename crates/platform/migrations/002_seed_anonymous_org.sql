-- Seed the anonymous/unauthenticated organization (nil UUID)
-- This org is used for verify requests that carry no org context (demo mode, CLI usage).
-- Copyright (c) 2025 TRUSTEDGE LABS LLC

INSERT INTO organizations (id, name, plan) VALUES (
    '00000000-0000-0000-0000-000000000000',
    'anonymous',
    'free'
) ON CONFLICT (id) DO NOTHING;
