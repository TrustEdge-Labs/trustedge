<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v2.1

**Defined:** 2026-03-16
**Core Value:** Prove that data from an edge device has not been tampered with -- from capture to verification -- using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.1 Requirements

Requirements for the Data Lifecycle & Hardware Integration milestone. Each maps to roadmap phases.

### Archive Decryption

- [ ] **UNWRAP-01**: User can run `trst unwrap` to decrypt and reassemble the original data from a .trst archive
- [ ] **UNWRAP-02**: `trst wrap` derives encryption key from device signing key via HKDF (replaces hardcoded demo key)
- [ ] **UNWRAP-03**: `trst unwrap` verifies archive integrity (signature + continuity chain) before decrypting
- [ ] **UNWRAP-04**: Existing `trst wrap` archives created with the new key derivation can be round-tripped (wrap -> unwrap -> identical data)

### YubiKey CLI

- [ ] **YUBI-01**: User can run `trst wrap --backend yubikey` to sign archives with hardware YubiKey (ECDSA P-256)
- [ ] **YUBI-02**: `trst verify` accepts both Ed25519 and ECDSA P-256 signatures (dispatches on key prefix)
- [ ] **YUBI-03**: YubiKey PIN is prompted interactively when required (not passed as CLI flag)
- [ ] **YUBI-04**: Demo script works with `--local` when no YubiKey is present (graceful skip)

### Named Profiles

- [x] **PROF-05**: User can run `trst wrap --profile sensor` with sensor-specific metadata (sample_rate, unit, sensor_model)
- [x] **PROF-06**: User can run `trst wrap --profile audio` with audio-specific metadata (sample_rate, bit_depth, channels, codec)
- [x] **PROF-07**: User can run `trst wrap --profile log` with log-specific metadata (application, host, log_level)
- [ ] **PROF-08**: All named profiles produce valid .trst archives that pass `trst verify`

## Future Requirements

Deferred to v2.2+. Tracked but not in current roadmap.

### Device Management
- **DEV-01**: Device enrollment via platform API with attestation
- **DEV-02**: Device key revocation and rotation
- **DEV-03**: Device identity bound to hardware serial number

### Key Distribution
- **KEY-01**: Recipients can decrypt archives shared with them
- **KEY-02**: Key wrapping for multi-recipient archives

### Live Capture
- **CAP-01**: Live audio capture wired into trst wrap pipeline
- **CAP-02**: Live video capture support

## Out of Scope

| Feature | Reason |
|---------|--------|
| Device agent/daemon | Edge runtime is v3+ scope -- demo uses CLI tools |
| Key distribution / multi-recipient | Requires identity system; defer to v2.2 |
| TPM support | No hardware to test against |
| Post-quantum crypto | Research phase only |
| Algorithm agility (negotiated crypto) | Hard-coded algorithms are sufficient |
| Backward compat with demo-key archives | Hardcoded key archives are test artifacts, not production data |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| UNWRAP-01 | Phase 43 | Pending |
| UNWRAP-02 | Phase 43 | Pending |
| UNWRAP-03 | Phase 43 | Pending |
| UNWRAP-04 | Phase 43 | Pending |
| YUBI-01 | Phase 44 | Pending |
| YUBI-02 | Phase 44 | Pending |
| YUBI-03 | Phase 44 | Pending |
| YUBI-04 | Phase 44 | Pending |
| PROF-05 | Phase 42 | Complete |
| PROF-06 | Phase 42 | Complete |
| PROF-07 | Phase 42 | Complete |
| PROF-08 | Phase 42 | Pending |

**Coverage:**
- v2.1 requirements: 12 total
- Mapped to phases: 12
- Unmapped: 0

---
*Requirements defined: 2026-03-16*
*Last updated: 2026-03-16 after roadmap creation — traceability complete*
