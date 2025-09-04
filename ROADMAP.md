<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge

* ✅ A reader implemented from the spec (no repo code) can verify test vectors.
* ✅ Deterministic test vectors with golden hash: `8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c`
* ✅ Network stack: complete client/server with chunk validation and ACK protocol
* ✅ Comprehensive validation: all security invariants enforced during processing
* [ ] Fuzz on `deserialize(Record)` doesn't crash (see M3).Privacy and trust at the edge.
-->

# TrustEdge Roadmap

> **North Star:** Trusted data from the edge. Capture bytes (audio/video/sensor), encrypt at the edge, attach verifiable provenance, and move them through untrusted pipes. Anyone downstream can **route** and **verify**; nobody can **forge** or **peek**.

> **Security**: For security policies and vulnerability reporting, see [`SECURITY.md`](./SECURITY.md)

---

## Outcomes (To Be Shipped)

1. **TrustEdge SDK (Rust crate + FFI-ready)**

   * Per-chunk AES-GCM with strict nonce discipline.
   * C2PA-inspired signed manifests bound into AEAD AAD.
   * Stable **envelope format** (`.trst`) with preamble & versioning.
   * Key management adapters (demo keyring → TPM/KMS path).
   * Verifier APIs (manifest sig + AAD + payload hash).

2. **CLI tools**

   * `trustedge-audio` — local capture → encrypt → envelope; `--decrypt` verify/restore.
   * `trustedge-client` / `trustedge-server` — reference network path with ACKs.

3. **Reference Router / Gateway (stateless)**

   * Accept `.trst` streams, enforce invariants, route to storage/queues (S3, NATS/Kafka).
   * Verify-optional; never needs decryption to route.

4. **Verification tooling**

   * `trustedge-verify` CLI: validate envelopes/streams and emit human/JSONL audit.

5. **Format spec**

   * Public, minimal, **test-vectored** spec for `StreamHeader` + `Record` + manifest schema.
   * Clear evolution story (MAGIC, VERSION, reserved fields).

---

## Architecture (snapshot)

* **Edge/Producer**
  Chunk → hash → sign manifest (Ed25519) → AAD = `{ header_hash || seq || nonce || blake3(manifest) }` → AES-GCM encrypt → emit `.trst` or network frame.

* **Transit/Router**
  Sees headers/sequences; can route/rate-limit; cannot read or forge.

* **Consumer**
  Verify signature & AAD, decrypt (if authorized), reassemble; or verify-only.

**Security goals**

* **Confidentiality:** AES-256-GCM; unique nonce per key/session.
* **Integrity & provenance:** Ed25519 over manifest + AAD binding.
* **Replay resistance:** contiguous `seq`, checked nonce prefix.
* **Privacy by default:** only minimal routable metadata is exposed.

---

## Current Status & Implementation Plan

**✅ Phase 1: Core Foundation - COMPLETED:**
* [x] Per-chunk encrypt/decrypt round-trip
* [x] Signed manifest bound into AAD
* [x] `.trst` envelope with preamble (`MAGIC="TRST"`, `VERSION=1`) and invariants
* [x] Reference client/server with ACKs and network streaming
* [x] Keyring-derived key support with PBKDF2
* [x] Shared types/helpers centralized in the lib crate
* [x] Format types consolidated in `src/format.rs`
* [x] Key ID field added to manifest for rotation support
* [x] Comprehensive validation: header consistency, sequence integrity, nonce verification
* [x] Production-ready network stack with TCP client/server
* [x] Deterministic test vectors with golden hash verification
* [x] Integration testing with CLI and network protocol validation

---

## Phase 2: Harden & Modularize Key Management 🚀

**Goal:** Production-ready key management with modular backend support and comprehensive CLI workflows.

**Key Tasks:**

### 2.1 CLI Documentation & Workflow Enhancement
* [ ] **Complete CLI flag documentation** with examples for each function:
  - `--set-passphrase <passphrase>` - Initial keyring setup
  - `--rotate-key --current-key-id <old> --new-key-id <new>` - Key rotation
  - `--export-key --key-id <id> --output-file <path>` - Key export
  - `--import-key --input-file <path> --verify-signature` - Key import
  - `--list-keys --show-metadata` - Key inventory
  - `--migrate-backend --from <source> --to <dest>` - Backend migration

### 2.2 Universal Backend Architecture ✅ COMPLETED
* ✅ **Capability-based Universal Backend system** for pluggable crypto operations:
  - Enum-based operation dispatch (`CryptoOperation`/`CryptoResult`)
  - Runtime capability discovery and backend selection
  - Preference-based backend routing with fallbacks
  - Type-safe operation validation and error handling
* ✅ **Backend registry system** for runtime backend selection
* ✅ **Comprehensive backend abstraction** supporting:
  - ✅ Keyring backend (available)
  - 📋 YubiKey PIV backend (Universal Backend ready)
  - 📋 TPM 2.0 backend (Universal Backend ready)
  - 📋 HSM/PKCS#11 backend (Universal Backend ready)

### 2.3 Migration & Error Handling
* [ ] **Comprehensive migration documentation** with step-by-step guides:
  - Software → hardware-backed key migration
  - Key rotation procedures and rollback scenarios
  - Cross-platform migration considerations
* [ ] **Enhanced error handling** with detailed CLI output examples:
  - Key mismatch scenarios and recovery steps
  - Backend failure modes and fallback procedures
  - Network connectivity issues during key operations

**Acceptance Criteria:**
* ✅ Complete CLI documentation with working examples for all key operations
* ✅ Universal Backend system supports easy addition of new backend types
* ✅ Capability-based operation dispatch with type safety and error handling
* ✅ Comprehensive test coverage (45 tests including 13 Universal Backend tests)
* 📋 Migration procedures tested across different backend combinations
* 📋 Hardware backend implementations (YubiKey, TPM, HSM)

---

## Phase 3: Network Operations & Authentication 🌐 ✅ COMPLETED

**Goal:** Enhanced network capabilities with Ed25519 mutual authentication and robust client-server tools.

**Completed Tasks:**

### 3.1 Enhanced Client-Server Architecture ✅
* [x] **Basic client-server architecture** with TCP networking
* [x] **Connection timeouts and retry logic** with configurable parameters
* [x] **Graceful server shutdown** with connection cleanup
* [x] **Enhanced connection management** with session tracking

### 3.2 Ed25519 Mutual Authentication System ✅
* [x] **Server authentication implementation** with identity certificates
* [x] **Client authentication implementation** with challenge-response protocol
* [x] **Automatic certificate generation** and management
* [x] **Session-based security** with timeout controls
* [x] **CLI integration for authentication**:
  - Server: `--require-auth --server-identity <name> --server-key <path>`
  - Client: `--enable-auth --client-identity <name> --client-cert <path>`

### 3.3 Production-Ready Network Stack ✅
* [x] **Structured logging and error handling** with verbose output modes
* [x] **Chunk validation reporting** with detailed status information
* [x] **Network resilience features** (reconnection, buffering, retry logic)
* [x] **Professional terminal output** with UTF-8 symbols and status indicators

**Achievement:** Complete authentication infrastructure with 672-line `auth.rs` implementation, CLI integration, and comprehensive documentation in CLI.md and EXAMPLES.md.

---

## Phase 4: Live Audio Capture & Chunking Pipeline 🎙️ ✅ COMPLETED

**Goal:** Real-time audio processing with live microphone capture and streaming capabilities.

**Key Tasks:** ✅ **ALL COMPLETED**

### 4.1 Cross-Platform Audio Integration ✅
* ✅ **Integrate `cpal` crate** for cross-platform microphone input - DONE
* ✅ **Linux audio testing** with ALSA/PulseAudio compatibility - DONE  
* ✅ **Audio device enumeration** and selection workflows - DONE
* ✅ **Real-time capture optimization** for low-latency processing - DONE

### 4.2 Live Streaming Architecture ✅
* ✅ **Real-time chunking pipeline**: mic → encrypt → stream - DONE
* ✅ **CLI mode for live streaming**: DONE
  - `--live-capture --audio-device <device> --chunk-duration-ms <ms>`
  - `--sample-rate <rate> --channels <num> --max-duration <seconds>`
* ✅ **Buffer management** for continuous audio processing - DONE
* ✅ **Latency optimization** and performance tuning - DONE

### 4.3 Comprehensive Workflow Documentation ✅
* ✅ **End-to-end audio workflow guide**: record → encrypt → transmit → decrypt → playback - DONE
* ✅ **Sample CLI invocations** and demonstration scripts - DONE
* ✅ **Integration with EXAMPLES.md** showing audio-specific examples - DONE  
* ✅ **Round-trip testing scripts** for validation - DONE

**Acceptance Criteria:** ✅ **ALL COMPLETED**
* ✅ Live microphone capture working cross-platform with cpal integration
* ✅ Complete audio workflow demonstrable via CLI commands
* ✅ Documentation includes working demo scripts and comprehensive examples
* ✅ Performance benchmarks for real-time processing

**Achievement:** Complete live audio capture pipeline with 504-line `audio.rs` implementation, cpal cross-platform integration, comprehensive CLI support, and extensive documentation across EXAMPLES.md and CLI.md.

---

## Phase 5: Advanced Network Features & Interoperability 🔧 📋 PLANNED

**Goal:** Extended network capabilities with Matter compatibility and advanced streaming features.

**Key Tasks:**

### 5.1 Enhanced Network Streaming
* [ ] **Improved logging and error handling** with structured output
* [ ] **Chunk validation reporting** with detailed status information
* [ ] **Sequence validation** and out-of-order handling
* [ ] **Network resilience** features (reconnection, buffering)

### 5.2 Live Network Streaming
* [ ] **Server-side live stream handling** with transparent audit logging
* [ ] **CLI options for live streaming**:
  - `--server-mode live-stream --audit-output <path>`
  - `--client-mode live-stream --target <server>`
* [ ] **Manifest audit trail** generation for compliance
* [ ] **Real-time monitoring** and health checks

### 5.3 Matter Compatibility Framework
* [ ] **Local test CA/certificate workflow** mimicking Matter device onboarding
* [ ] **Certificate-to-envelope mapping** for Matter device IDs
* [ ] **PROTOCOL.md updates** with Matter integration examples
* [ ] **Simulation tools** for Matter device scenarios

**Acceptance Criteria:**
* Enhanced client-server tools with production-ready logging
* Live streaming capabilities with audit trail generation
* Matter compatibility documented with working examples
* Protocol documentation covers device integration scenarios

---

## Phase 6: Testing, Fuzzing & Auditability 🔍

**Goal:** Comprehensive testing infrastructure with fuzzing and audit capabilities.

**Key Tasks:**

### 5.1 Enhanced Test Vectors
* [ ] **Deterministic vectors** for files and live streams
* [ ] **Cross-platform test coverage** ensuring consistent behavior
* [ ] **Performance benchmarks** and regression testing
* [ ] **Test vector publication** for third-party validation

### 5.2 Comprehensive Error Simulation
* [ ] **Tampering simulation** with documented failure modes
* [ ] **Reordering attack testing** and prevention validation
* [ ] **Key mismatch scenarios** with recovery procedures
* [ ] **Network failure simulation** and resilience testing

### 5.3 Fuzzing & Property-Based Testing
* [ ] **Integration with fuzzing tools** (cargo-fuzz, etc.)
* [ ] **Property-based testing** with proptest or similar
* [ ] **Fuzzing configuration documentation** in TESTING.md
* [ ] **Continuous fuzzing** in CI/CD pipeline

**Acceptance Criteria:**
* Comprehensive test suite covering all critical paths
* Fuzzing runs 24+ hours without crashes
* All error scenarios documented with expected behaviors
* Test infrastructure supports regression testing

---

## Phase 7: Documentation & Community Engagement 📖

**Goal:** Complete documentation ecosystem and community building for feedback and contributions.

**Key Tasks:**

### 6.1 Complete Documentation Update
* [ ] **All markdown docs reflect new CLI workflows** and features
* [ ] **Error handling documentation** with real CLI examples
* [ ] **Integration scenario guides** for various use cases
* [ ] **Migration and upgrade path documentation**

### 6.2 Demo Scripts & Lab Guides
* [ ] **Step-by-step demo script**: "record → stream → decrypt → verify → audit"
* [ ] **Lab guide creation** for hands-on learning
* [ ] **Video tutorials** or screen recordings for complex workflows
* [ ] **Example configurations** for common scenarios

### 6.3 Community Outreach Strategy
* [ ] **Beta testing program** with privacy-focused communities
* [ ] **Feedback collection framework** for user experience improvements
* [ ] **Contribution guidelines** and development setup documentation
* [ ] **Community engagement plan**:
  - Privacy advocacy groups
  - Maker communities
  - IoT developer communities
  - Security researcher networks

**Target Communities (documented in ROADMAP.md):**
- Privacy advocacy organizations (EFF, etc.)
- Maker spaces and hardware hacker communities  
- IoT developer forums and conferences
- Security research communities
- Edge AI developer groups

**Acceptance Criteria:**
* All documentation updated and consistent across repository
* Demo scripts work out-of-the-box for new users
* Community outreach plan implemented with measurable engagement
* Beta testing program active with regular feedback collection

---

## Phase 8: Modular Hardware & Ecosystem Integration 🔧

**Goal:** Production-ready hardware integration and ecosystem compatibility.

**Key Tasks:**

### 7.1 Hardware Abstraction Layer
* [ ] **Abstract key management** for easy backend swapping
* [ ] **TPM 2.0 integration** with proper key isolation
* [ ] **HSM support** for enterprise scenarios
* [ ] **Hardware-specific documentation** and setup guides

### 7.2 Upgrade Path Documentation
* [ ] **Migration procedures** between different hardware backends
* [ ] **Compatibility matrices** for supported hardware
* [ ] **Troubleshooting guides** for common hardware issues
* [ ] **Future roadmap** for additional hardware support

**Acceptance Criteria:**
* Clean abstraction allows easy addition of new backends
* TPM integration working with proper security boundaries
* Migration between backends tested and documented
* Hardware compatibility clearly documented

---

## Immediate Next Actions (Priority Order)

### Week 1-2: Planning & Documentation
1. **Update README and ROADMAP.md** with this comprehensive plan ✅
2. **Create GitHub issues/milestones** for each discrete deliverable
3. **Establish project board** with clear task tracking
4. **Document current CLI gaps** and plan implementation order

### Week 3-4: Key Management Foundation
1. **Begin modular key management refactoring**
2. **Document all existing CLI flows** with examples
3. **Design backend abstraction interface**
4. **Start TPM integration research and planning**

### Month 2: Live Audio Integration
1. **Integrate cpal for audio capture**
2. **Build real-time chunking pipeline**
3. **Implement basic live streaming CLI**
4. **Create audio workflow documentation**

### Month 3: Network & Testing
1. **Enhance client-server tools**
2. **Add comprehensive error simulation**
3. **Begin fuzzing integration**
4. **Start community outreach planning**

---

## Milestone M1 — **Format v1 Freeze (MVP hardening)**

**Goal:** Freeze v1 of the envelope & manifest and publish a minimal spec + vectors.

**Scope**

* [x] Finalize manifest fields (add `key_id`; keep `model_ids`, `ai_used`; reserve `device_attest`).
* [x] Consolidate format types in centralized `format.rs` module.
* [x] Spec doc (`FORMAT.md`): structures, byte orders, invariants, failure modes.
* [x] Confirm invariants: contiguous `seq`, fixed `nonce_prefix`, `header_hash` match, `key_id` match, AAD layout.
* [x] Test vectors: deterministic `.trst` with known keys and golden hash verification.
* [x] File framing: keep preamble + bincode framing; document record boundaries & EOF handling.
* [x] Enhanced validation: header consistency, key ID verification, strict sequencing
* [x] Network protocol: complete client/server implementation with validation
* [x] Comprehensive testing: unit tests, integration tests, CLI testing, network validation

**Acceptance**

* A reader implemented from the spec (no repo code) can verify test vectors.
* Fuzz on `deserialize(Record)` doesn’t crash (see M3).

---

## Milestone M2 — **Key Management & Enhanced Security**

**Goal:** Production-ready key management and advanced security features.

**Scope**

* [x] Add `key_id` to `FileHeader` and surface in spec + CLIs.
* [x] Derive per-session AEAD keys from a device root via PBKDF2 (keyring passphrase + salt).
* [x] Reject decrypt when `key_id` mismatches the configured/derivable key.
* [x] CLI UX: `--use-keyring --salt-hex …` (derive); `--key-hex` (override).
* [x] Document rotation & recovery (how to re-derive past session keys).
* [ ] Add key versioning and migration tools
* [ ] Implement secure key storage with proper zeroization
* [ ] Add HSM/TPM integration points for production deployments

**Acceptance**

* ✅ Decrypt fails fast on wrong `key_id`.
* ✅ Round-trip passes with both keyring-derived and explicit `--key-hex`.
* [ ] Key rotation scenarios fully tested and documented
* [ ] Production key management best practices documented

---

## Milestone M3 — **Verification Tooling & QA**

**Goal:** Ship a verifier and raise safety with tests & fuzzing.

**Scope**

* [ ] `trustedge-verify`: read `.trst`, print human-readable report (seq, signer, hashes, sizes).
* [ ] `--json` output for SIEM (JSONL per record).
* [ ] Property tests (proptest): AAD layout, seq monotonicity, header hash binding.
* [ ] Fuzzing (`cargo-fuzz`) on decoders (preamble, `StreamHeader`, `Record`).
* [ ] Known-answer tests (KATs): deterministic vectors in CI.
* [ ] Zeroization audit for secrets; non-leaky error messages.

**Acceptance**

* CI runs proptests + fuzz corpus sanity (+1min smoke).
* Verifier detects: nonce prefix mismatch, seq gap, tampered manifest/payload.

---

## Milestone M4 — **Transport & Router**

**Goal:** Better network story + reference router.

**Scope**

* [ ] QUIC or NATS transport with backpressure/ordering metrics.
* [ ] Stateless Router: verify (optional), enforce invariants, forward to sinks (S3/NATS/Kafka).
* [ ] Structured logs + Prometheus counters: chunks, drops, verify failures, latency.

**Acceptance**

* E2E demo: client → router → sink → verifier/decrypt.
* Backpressure test: router throttles without data loss or deadlocks.

---

## Milestone M5 — **Attestation Hook (future-proof)**

**Goal:** Reserve space for device/TEE attestation without blocking v1.

**Scope**

* [ ] Add optional `device_attest` field in manifest (opaque bytes, signed).
* [ ] Document how a TEE/TPM quote would be produced/verified (deferred).
* [ ] Keep verification tolerant when absent.

**Acceptance**

* Manifests with/without attestation both verify; size overhead documented.

---

## Milestone M6 — **Security Review & Hardening**

**Goal:** Comprehensive security audit and hardening before v1.0 release.

**Scope**

* [ ] External cryptographic audit of implementation
* [ ] Side-channel attack analysis and mitigations
* [ ] Fuzzing campaign for all deserializers and network code
* [ ] Security testing of key management and nonce handling
* [ ] Review dependency supply chain and implement security monitoring
* [ ] Documentation review for security implications
* [ ] Penetration testing of network protocol components
* [ ] Performance impact analysis of security features

**Acceptance**

* Clean external security audit report
* Fuzzing runs 24h+ without crashes on all parsers
* Security test suite covers all critical paths
* Security documentation complete and reviewed

---

## Adding New Universal Backends (Developer Guide)

### Universal Backend Implementation Pattern

To add a new backend (e.g., YubiKey, TPM, HSM), implement the `UniversalBackend` trait:

```rust
use trustedge_audio::backends::universal::{
    UniversalBackend, BackendCapabilities, CryptoOperation, CryptoResult
};

struct YubiKeyBackend {
    config: YubiKeyConfig,
}

impl UniversalBackend for YubiKeyBackend {
    fn name(&self) -> &'static str {
        "yubikey"
    }
    
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            derive_key: true,
            compute_hash: true,
            generate_nonce: true,
            sign_data: true,      // YubiKey-specific capability
            verify_signature: true, // YubiKey-specific capability
        }
    }
    
    fn perform_operation(&self, operation: CryptoOperation) -> anyhow::Result<CryptoResult> {
        match operation {
            CryptoOperation::DeriveKey { domain, purpose } => {
                // YubiKey PIV slot-based key derivation
                let key = self.derive_from_piv_slot(&domain, &purpose)?;
                Ok(CryptoResult::DerivedKey(key))
            }
            CryptoOperation::ComputeHash { algorithm, data } => {
                // Hardware-accelerated hashing if available
                let hash = self.hardware_hash(&algorithm, &data)?;
                Ok(CryptoResult::ComputedHash(hash))
            }
            // Add YubiKey-specific operations
            _ => Err(anyhow::anyhow!("Operation not supported by YubiKey backend"))
        }
    }
}
```

### Registering Your Backend

```rust
use trustedge_audio::backends::universal_registry::UniversalBackendRegistry;

// Create registry with your backend
let mut registry = UniversalBackendRegistry::new();
registry.register_backend(Box::new(YubiKeyBackend::new(config)?));

// Use with preference-based selection
let preferences = BackendPreferences::default()
    .prefer_backend("yubikey")
    .fallback_to("keyring");
    
let backend = registry.select_backend_with_preferences(&preferences)?;
```

### Testing Your Backend

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yubikey_capabilities() {
        let backend = YubiKeyBackend::new(test_config()).unwrap();
        let caps = backend.capabilities();
        
        assert!(caps.derive_key);
        assert!(caps.sign_data);
        assert!(caps.verify_signature);
    }
    
    #[test]
    fn test_yubikey_key_derivation() {
        let backend = YubiKeyBackend::new(test_config()).unwrap();
        let operation = CryptoOperation::DeriveKey {
            domain: "audio".to_string(),
            purpose: "encryption".to_string(),
        };
        
        let result = backend.perform_operation(operation).unwrap();
        match result {
            CryptoResult::DerivedKey(key) => {
                assert_eq!(key.len(), 32);
                // Verify deterministic derivation
            }
            _ => panic!("Expected DerivedKey result"),
        }
    }
}
```

**📖 For comprehensive documentation and examples, see [UNIVERSAL_BACKEND.md](UNIVERSAL_BACKEND.md).**

### Integration Steps

1. **Add backend-specific dependencies** to `Cargo.toml`
2. **Implement the UniversalBackend trait** for your hardware/service
3. **Add capability discovery** for your backend's supported operations
4. **Register backend** in the Universal Backend registry system
5. **Add integration tests** with mock hardware if needed
6. **Document CLI usage** and migration procedures

### Example Backend Integrations

**TPM 2.0 Backend:**
- Use `tss-esapi` crate for TPM communication
- Implement secure key storage in TPM NVRAM
- Add attestation support for key provenance

**HSM Backend:**
- Use PKCS#11 interface for hardware security modules
- Implement proper session management
- Add support for high-availability HSM clusters

**Matter Certificate Backend:**
- Integrate with Matter commissioning protocols
- Map device certificates to encryption keys
- Support fabric-based key isolation

* **Interoperability:** Optional CBOR/COSE envelope variant; map manifest to C2PA claims where practical.
* **Packaging:** C ABI for SDK; language bindings.
* **Edge AI:** Standardize `model_ids` (name+version+hash); record basic runtime signals.
* **Storage adapters:** S3/GCS writers & lifecycle policies for encrypted objects.

---

## Non-Goals (for v1)

* End-to-end identity binding to a PKI (beyond Ed25519 keys in manifests).
* Multi-party re-encryption (KMS/ABE/MLS) — future exploration.
* Lossless media containerization — payload is opaque bytes.

---

## Risks & Mitigations

* **Nonce misuse** → Strict prefix+counter enforcement; tests preventing reuse; fail-closed.
* **Spec drift** → Single source of truth in `trustedge_audio` crate; doc + vectors.
* **Key loss** → Documented key derivation and rotation; warn loud in CLIs.
* **DoS via malformed input** → Length checks, fuzzing, non-allocating decode paths where possible.

> **See Also**: [`THREAT_MODEL.md`](./THREAT_MODEL.md) for comprehensive threat analysis and [`SECURITY.md`](./SECURITY.md) for security policies.

---

## Developer Ergonomics

* **Issue labels:** `M1-Format`, `M2-KeyMgmt`, `M3-Verify`, `M4-Transport`, `M5-Attest`, `good-first-issue`.
* **CI matrix:** linux/mac/windows stable; minimal nightly for fuzz.
* **Examples:** `examples/` with tiny WAV → `.trst` → verify → restore pipeline.

---

## Quick Commands

```bash
# Encrypt → envelope + plaintext
trustedge-audio -i input.wav -o roundtrip.wav \
  --chunk 8192 --envelope out.trst --use-keyring --salt-hex "$SALT"

# Decrypt → verify + restore from envelope
trustedge-audio --decrypt -i out.trst -o restored.wav \
  --use-keyring --salt-hex "$SALT"

# Network demo (TCP) - start server
trustedge-server --listen 127.0.0.1:8080 --decrypt \
  --use-keyring --salt-hex "$SALT" --verbose

# Network demo (TCP) - send file
trustedge-client --server 127.0.0.1:8080 --file input.wav \
  --use-keyring --salt-hex "$SALT"

# Test network with synthetic chunks
trustedge-client --server 127.0.0.1:8080 --test-chunks 100 \
  --key-hex <64-char-hex-key>
```

---

## Definition Snippets (v1)

* **AAD layout:** `header_hash (32) || seq (8, BE) || nonce (12) || blake3(manifest) (32)`
* **Preamble:** `MAGIC="TRST"`, `VERSION=1`
* **Envelope:** `StreamHeader { v, header (58), header_hash (32) }` then repeated
  `Record { seq, nonce, sm, ct }`
* **Manifest:** Includes `key_id` field for key rotation support
* **FileHeader:** 58 bytes with `key_id`, `device_id_hash`, `nonce_prefix`, and other metadata

---

## Community Engagement Strategy (Phase 6)

### Target Communities for Beta Testing & Feedback

**Privacy Advocacy Groups:**
- Electronic Frontier Foundation (EFF) developer community
- Tor Project ecosystem developers
- Privacy-focused hackers and security researchers
- Digital rights organizations with technical capacity

**Maker & Hardware Communities:**
- Local maker spaces with IoT projects
- Arduino and Raspberry Pi communities
- Hardware hacking groups (DEF CON villages, etc.)
- DIY security and privacy tool builders

**IoT Developer Communities:**
- Matter/Thread developer forums
- Zigbee Alliance technical working groups
- Edge computing developer conferences
- Industrial IoT security practitioners

**Security Research Networks:**
- Academic researchers in applied cryptography
- Bug bounty hunters interested in new protocols
- Security consultants working with IoT devices
- Penetration testers needing new tools

**Edge AI Developer Groups:**
- TinyML community members
- Edge computing framework developers
- Privacy-preserving ML researchers
- Federated learning practitioners

### Engagement Activities

**Phase 6.1: Community Research & Outreach**
- [ ] **Identify key community leaders** and potential early adopters
- [ ] **Create community engagement plan** with specific outreach targets
- [ ] **Develop presentation materials** for conferences and meetups
- [ ] **Establish feedback collection mechanisms** (surveys, GitHub discussions)

**Phase 6.2: Beta Testing Program**
- [ ] **Recruit 10-15 beta testers** from target communities
- [ ] **Provide comprehensive testing guides** and support documentation
- [ ] **Schedule regular feedback sessions** and technical reviews
- [ ] **Implement feedback tracking system** for feature prioritization

**Phase 6.3: Conference & Community Presentations**
- [ ] **Submit to relevant conferences**:
  - DEF CON (IoT Village, Crypto & Privacy Village)
  - RSA Conference (Applied Cryptography track)
  - Black Hat (IoT Security presentations)
  - Privacy engineering conferences
- [ ] **Local meetup presentations**:
  - Privacy engineering meetups
  - Rust user groups
  - IoT developer meetups
  - Security researcher gatherings

**Phase 6.4: Open Source Community Building**
- [ ] **Establish contributor guidelines** and code of conduct
- [ ] **Create beginner-friendly issues** with "good first issue" labels
- [ ] **Mentorship program** for new contributors
- [ ] **Regular community calls** for coordination and feedback

### Community Success Metrics

**Engagement Targets:**
- 50+ GitHub stars from technical community members
- 10+ meaningful contributions from external developers
- 5+ production deployments by beta testers
- Positive feedback from 80%+ of beta testing participants

**Technical Validation:**
- External security review by 2+ independent researchers
- Successful integration demonstrations in 3+ different environments
- Performance validation by community members
- Documentation rated as "comprehensive" by 90%+ of users

---

## 📊 Current Progress Summary

| **Phase** | **Status** | **Key Achievements** |
|-----------|------------|---------------------|
| Phase 1 ✅ | **COMPLETE** | Core encryption, signed manifests, .trst format, test vectors |
| Phase 2 ✅ | **COMPLETE** | Keyring integration, pluggable backends, production CLI |
| Phase 3 ✅ | **COMPLETE** | **Network operations & Ed25519 mutual authentication** |
| Phase 4 ✅ | **COMPLETE** | **Live audio capture & streaming pipeline with cpal integration** |
| Phase 5 📋 | **PLANNED** | Advanced network features & Matter compatibility |
| Phase 6 🔬 | **PLANNED** | Testing infrastructure & security audit |
| Phase 7 🌍 | **PLANNED** | Community engagement & beta testing |
| Phase 8 🔧 | **PLANNED** | Hardware integration & ecosystem expansion |

**🎯 Current Focus**: Transitioning to **Phase 5: Advanced Network Features** with Matter compatibility and enhanced network operations.

**🔐 Major Achievements**: 
- Complete Ed25519 mutual authentication system with CLI integration (672-line implementation)
- Full live audio capture pipeline with cross-platform cpal integration (504-line audio module)
- Comprehensive testing infrastructure with 51 automated tests covering all workflows

---

## Success Criteria

* A third-party can implement a verifier from the spec + vectors and reach the same results.
* Tamper and misuse cases are reliably detected (and documented).
* Transport/storage backends are interchangeable without changing guarantees.
* Low-friction adoption: simple SDK, clear examples, deterministic tests.

---

## Legal & Attribution

**Copyright** © 2025 John Turner. All rights reserved.

**License**: This roadmap is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/TrustEdge-Labs/trustedge) — Privacy and trust at the edge.

**Contributing**: See project repository for contribution guidelines and milestone tracking.
