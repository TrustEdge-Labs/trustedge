# Test Inventory Baseline

**Generated:** 2026-02-10

## Package: trustedge-attestation (10 tests)

### Unit Tests (lib) — 10 tests

#### Module: tests (10 tests)
```
tests::test_attestation_data_fields: test
tests::test_centralized_envelope_attestation: test
tests::test_centralized_json_attestation: test
tests::test_file_not_found_error: test
tests::test_force_json_verification: test
tests::test_json_only_with_provided_key: test
tests::test_provided_key_source: test
tests::test_verification_details: test
tests::test_verification_failure: test
tests::test_verification_with_missing_attestation: test
```


## Package: trustedge-core (258 tests)

### Unit Tests (lib) — 133 tests

#### Module: archive (7 tests)
```
archive::tests::test_archive_dir_name: test
archive::tests::test_archive_validation: test
archive::tests::test_mutation_missing_chunk_causes_validation_failure: test
archive::tests::test_parse_chunk_index: test
archive::tests::test_schema_mismatch_chunk_count: test
archive::tests::test_signature_mismatch: test
archive::tests::test_write_and_read_archive_round_trip: test
```

#### Module: asymmetric (6 tests)
```
asymmetric::tests::test_ecdh_p256: test
asymmetric::tests::test_ecdsa_p256_key_generation: test
asymmetric::tests::test_ed25519_key_generation: test
asymmetric::tests::test_public_key_serialization: test
asymmetric::tests::test_rsa_key_encryption: test
asymmetric::tests::test_rsa_key_generation: test
```

#### Module: audio (3 tests)
```
audio::tests::test_audio_chunk_to_from_bytes: test
audio::tests::test_audio_config_default: test
audio::tests::test_audio_stub: test
```

#### Module: backends::keyring (3 tests)
```
backends::keyring::tests::test_backend_info: test
backends::keyring::tests::test_key_derivation_requires_16_byte_salt: test
backends::keyring::tests::test_keyring_backend_creation: test
```

#### Module: backends::software_hsm (33 tests)
```
backends::software_hsm::tests::test_algorithm_mismatch_signing: test
backends::software_hsm::tests::test_algorithm_mismatch_verification: test
backends::software_hsm::tests::test_backend_capabilities: test
backends::software_hsm::tests::test_backend_creation_default_config: test
backends::software_hsm::tests::test_backend_creation_with_new: test
backends::software_hsm::tests::test_backend_info: test
backends::software_hsm::tests::test_concurrent_key_operations: test
backends::software_hsm::tests::test_corrupted_key_files: test
backends::software_hsm::tests::test_duplicate_key_generation: test
backends::software_hsm::tests::test_ed25519_key_generation_and_signing: test
backends::software_hsm::tests::test_empty_data_signing: test
backends::software_hsm::tests::test_invalid_signature_lengths: test
backends::software_hsm::tests::test_key_file_storage: test
backends::software_hsm::tests::test_key_store_directory_creation: test
backends::software_hsm::tests::test_key_usage_tracking: test
backends::software_hsm::tests::test_large_data_signing: test
backends::software_hsm::tests::test_list_keys: test
backends::software_hsm::tests::test_many_keys: test
backends::software_hsm::tests::test_metadata_persistence: test
backends::software_hsm::tests::test_missing_key_files: test
backends::software_hsm::tests::test_multiple_signatures_same_key: test
backends::software_hsm::tests::test_operation_support: test
backends::software_hsm::tests::test_p256_key_generation_and_signing: test
backends::software_hsm::tests::test_p256_signature_variations: test
backends::software_hsm::tests::test_signature_determinism_ed25519: test
backends::software_hsm::tests::test_signature_randomness_p256: test
backends::software_hsm::tests::test_sign_with_nonexistent_key: test
backends::software_hsm::tests::test_universal_backend_get_public_key: test
backends::software_hsm::tests::test_universal_backend_hash_operations: test
backends::software_hsm::tests::test_universal_backend_interface: test
backends::software_hsm::tests::test_universal_backend_unsupported_operations: test
backends::software_hsm::tests::test_unsupported_algorithm_generation: test
backends::software_hsm::tests::test_verify_with_nonexistent_key: test
```

#### Module: backends::universal (3 tests)
```
backends::universal::tests::test_backend_capabilities: test
backends::universal::tests::test_key_derivation_context_builder: test
backends::universal::tests::test_operation_type_supported: test
```

#### Module: backends::universal_keyring (5 tests)
```
backends::universal_keyring::tests::test_backend_capabilities: test
backends::universal_keyring::tests::test_hash_operation: test
backends::universal_keyring::tests::test_key_derivation_operation: test
backends::universal_keyring::tests::test_supports_operation: test
backends::universal_keyring::tests::test_universal_keyring_backend_creation: test
```

#### Module: backends::universal_registry (5 tests)
```
backends::universal_registry::tests::test_backend_preferences: test
backends::universal_registry::tests::test_find_backend_for_operation: test
backends::universal_registry::tests::test_perform_operation_via_registry: test
backends::universal_registry::tests::test_registry_creation: test
backends::universal_registry::tests::test_registry_with_defaults: test
```

#### Module: backends::yubikey (3 tests)
```
backends::yubikey::tests::test_backend_capabilities: test
backends::yubikey::tests::test_backend_info: test
backends::yubikey::tests::test_yubikey_config_default: test
```

#### Module: chain (10 tests)
```
chain::tests::test_base64_encoding: test
chain::tests::test_blake3_hex_or_b64: test
chain::tests::test_chain_next: test
chain::tests::test_empty_segments: test
chain::tests::test_gap_in_segments: test
chain::tests::test_genesis_computation: test
chain::tests::test_happy_path_with_3_segments: test
chain::tests::test_removing_last_segment_fails_end_of_chain_truncated: test
chain::tests::test_segment_hash: test
chain::tests::test_swapping_segments_fails_out_of_order: test
```

#### Module: crypto (9 tests)
```
crypto::tests::test_aad_generation: test
crypto::tests::test_aead_encrypt_decrypt_round_trip: test
crypto::tests::test_base64_encoding_decoding: test
crypto::tests::test_device_keypair_generation: test
crypto::tests::test_device_keypair_import_export: test
crypto::tests::test_nonce_generation_and_formatting: test
crypto::tests::test_public_key_parsing: test
crypto::tests::test_round_trip_sign_verify: test
crypto::tests::test_secret_zeroization: test
```

#### Module: envelope (7 tests)
```
envelope::tests::test_envelope_creation: test
envelope::tests::test_envelope_hash_consistency: test
envelope::tests::test_envelope_large_payload_roundtrip: test
envelope::tests::test_envelope_seal_unseal_roundtrip: test
envelope::tests::test_envelope_verification: test
envelope::tests::test_envelope_wrong_key_fails: test
envelope::tests::test_large_payload_chunking: test
```

#### Module: envelope_v2_bridge (3 tests)
```
envelope_v2_bridge::tests::test_envelope_inspection: test
envelope_v2_bridge::tests::test_format_detection: test
envelope_v2_bridge::tests::test_unified_envelope: test
```

#### Module: format (7 tests)
```
format::tests::test_algorithm_enum_roundtrip: test
format::tests::test_default_header_creation: test
format::tests::test_fileheader_v2_roundtrip: test
format::tests::test_invalid_algorithm_parsing: test
format::tests::test_non_default_algorithms: test
format::tests::test_unsupported_algorithm_rejection: test
format::tests::test_v1_to_v2_migration: test
```

#### Module: hybrid (6 tests)
```
hybrid::tests::test_envelope_structure: test
hybrid::tests::test_hybrid_encryption_rsa: test
hybrid::tests::test_invalid_envelope: test
hybrid::tests::test_symmetric_encryption_roundtrip: test
hybrid::tests::test_symmetric_key_generation: test
hybrid::tests::test_wrong_private_key: test
```

#### Module: manifest (6 tests)
```
manifest::tests::test_canonical_bytes_excludes_signature: test
manifest::tests::test_decimal_precision: test
manifest::tests::test_key_ordering: test
manifest::tests::test_manifest_creation: test
manifest::tests::test_stable_canonicalization: test
manifest::tests::test_validation: test
```

#### Module: transport::quic (7 tests)
```
transport::quic::tests::test_certificate_requirements: test
transport::quic::tests::test_network_chunk_compatibility: test
transport::quic::tests::test_quic_config_extremes: test
transport::quic::tests::test_quic_transport_creation: test
transport::quic::tests::test_socket_address_parsing: test
transport::quic::tests::test_transport_config_validation: test
transport::quic::tests::test_transport_defaults: test
```

#### Module: transport::tcp (9 tests)
```
transport::tcp::tests::test_address_parsing_for_tcp: test
transport::tcp::tests::test_large_data_handling: test
transport::tcp::tests::test_network_chunk_framing: test
transport::tcp::tests::test_tcp_connection_state: test
transport::tcp::tests::test_tcp_transport_cleanup: test
transport::tcp::tests::test_tcp_transport_config_validation: test
transport::tcp::tests::test_tcp_transport_creation: test
transport::tcp::tests::test_tcp_transport_stats: test
transport::tcp::tests::test_transport_config_defaults: test
```

#### Module: vectors (1 tests)
```
vectors::tests::golden_trst_digest_is_stable: test
```


### Integration Tests: auth_integration — 3 tests
```
test_certificate_generation_and_verification: test
test_mutual_authentication: test
test_session_management: test
```

### Integration Tests: domain_separation_test — 7 tests
```
test_domain_separation_basic_functionality: test
test_domain_separation_different_manifests: test
test_domain_separation_prevents_cross_context_reuse: test
test_domain_separation_prevents_raw_signature_reuse: test
test_domain_separation_tampered_prefix_fails: test
test_domain_string_content: test
test_signature_determinism_with_domain_separation: test
```

### Integration Tests: network_integration — 7 tests
```
test_authenticated_transfer: test
test_basic_file_transfer: test
test_connection_error_handling: test
test_data_integrity: test
test_empty_file_transfer: test
test_large_file_transfer: test
test_multiple_file_types: test
```

### Integration Tests: roundtrip_integration — 15 tests
```
test_binary_file_roundtrip: test
test_byte_perfect_restoration: test
test_comprehensive_chunk_sizes: test
test_comprehensive_mime_type_detection: test
test_empty_file_roundtrip: test
test_format_detection_accuracy: test
test_inspect_encrypted_file: test
test_json_file_roundtrip: test
test_medium_file_roundtrip: test
test_mp3_file_roundtrip: test
test_multiple_chunk_sizes: test
test_pdf_file_roundtrip: test
test_small_file_roundtrip: test
test_text_file_roundtrip: test
test_unknown_format_roundtrip: test
```

### Integration Tests: software_hsm_integration — 9 tests
```
test_backend_preference_selection: test
test_cli_key_lifecycle: test
test_cross_session_key_persistence: test
test_disk_space_and_permissions: test
test_file_based_signing_workflow: test
test_large_scale_key_management: test
test_metadata_corruption_recovery: test
test_partial_file_corruption_recovery: test
test_software_hsm_registry_integration: test
```

### Integration Tests: transport_integration — 13 tests
```
test_concurrent_tcp_connections: test
test_multi_chunk_data_transfer: test
test_real_network_chunk_serialization: test
test_real_quic_data_transfer: test
test_real_tcp_bidirectional_communication: test
test_real_tcp_connection_failure: test
test_real_tcp_data_transfer: test
test_real_tcp_large_data_transfer: test
test_real_tcp_message_size_limits: test
test_real_transport_timeout_scenarios: test
test_transport_data_integrity: test
test_transport_protocol_negotiation: test
test_transport_security_configuration: test
```

### Integration Tests: universal_backend_integration — 6 tests
```
test_universal_backend_capability_based_selection: test
test_universal_backend_encrypt_decrypt_workflow: test
test_universal_backend_error_handling: test
test_universal_backend_multiple_operations_workflow: test
test_universal_backend_performance_characteristics: test
test_universal_backend_registry_management: test
```

### Integration Tests: yubikey_certificate_debug — 5 tests
```
test_certificate_id_debug: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_hardware_detection — 4 tests
```
tests::test_ci_environment_detection: test
tests::test_environment_detection: test
tests::test_pkcs11_module_validation: test
tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_hardware_tests — 10 tests
```
test_hardware_capabilities: test
test_hardware_initialization: test
test_hardware_operation_support: test
test_hardware_pkcs11_session: test
test_hardware_slot_enumeration: test
test_pin_requirement_detection: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_integration — 8 tests
```
test_certificate_quic_compatibility: test
test_multi_slot_operations: test
test_phase1_certificate_validation: test
test_phase2_certificate_generation: test
test_phase3_quic_integration: test
test_yubikey_backend_initialization: test
test_yubikey_capabilities: test
test_yubikey_error_handling: test
```

### Integration Tests: yubikey_piv_analysis — 7 tests
```
test_certificate_based_discovery: test
test_direct_pkcs11_enumeration: test
test_piv_slot_analysis: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_real_operations — 10 tests
```
test_real_attestation_operation: test
test_real_backend_initialization_with_pin: test
test_real_certificate_operations: test
test_real_concurrent_operations: test
test_real_key_enumeration: test
test_real_signing_operation: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_simulation_tests — 11 tests
```
test_certificate_parameter_validation: test
test_configuration_serialization: test
test_error_handling_simulation: test
test_piv_slot_validation: test
test_pkcs11_module_validation: test
test_yubikey_backend_interface: test
test_yubikey_configuration_validation: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```

### Integration Tests: yubikey_strict_hardware — 10 tests
```
test_strict_hardware_capabilities: test
test_strict_operation_support: test
test_strict_piv_slots_accessible: test
test_strict_pkcs11_operations: test
test_strict_yubikey_backend_initialization: test
test_strict_yubikey_hardware_required: test
yubikey_hardware_detection::tests::test_ci_environment_detection: test
yubikey_hardware_detection::tests::test_environment_detection: test
yubikey_hardware_detection::tests::test_pkcs11_module_validation: test
yubikey_hardware_detection::tests::test_slot_enumeration: test
```


## Package: trustedge-pubky (19 tests)

### Unit Tests (lib) — 7 tests

#### Module: mock (1 tests)
```
mock::tests::test_mock_backend: test
```

#### Module: tests (6 tests)
```
tests::test_backend_creation: test
tests::test_deterministic_key_generation: test
tests::test_extract_private_key_seed: test
tests::test_mock_integration: test
tests::test_public_key_serialization: test
tests::test_receive_trusted_data: test
```


### Integration Tests: integration_tests — 12 tests
```
test_cli_help: test
test_decrypt_invalid_key_format: test
test_decrypt_missing_key: test
test_encrypt_invalid_recipient: test
test_encrypt_nonexistent_recipient: test
test_file_io_errors: test
test_invalid_seed_error: test
test_key_generation_randomness: test
test_key_generation: test
test_key_generation_with_seed: test
test_migrate_missing_files: test
test_resolve_invalid_id: test
```


## Package: trustedge-pubky-advanced (10 tests)

### Unit Tests (lib) — 10 tests

#### Module: envelope (3 tests)
```
envelope::tests::test_envelope_serialization: test
envelope::tests::test_envelope_v2_seal_unseal: test
envelope::tests::test_large_payload_v2: test
```

#### Module: keys (4 tests)
```
keys::tests::test_dual_key_generation: test
keys::tests::test_key_derivation: test
keys::tests::test_key_serialization: test
keys::tests::test_pubky_identity_creation: test
```

#### Module: pubky_client (3 tests)
```
pubky_client::tests::test_identity_record_creation: test
pubky_client::tests::test_record_expiration: test
pubky_client::tests::test_record_serialization: test
```


## Package: trustedge-receipts (23 tests)

### Unit Tests (lib) — 23 tests

#### Module: tests (23 tests)
```
tests::test_amount_tampering_resistance: test
tests::test_assign_receipt: test
tests::test_chain_integrity_validation: test
tests::test_complete_receipt_chain_with_decryption: test
tests::test_comprehensive_multi_party_chain: test
tests::test_concurrent_assignments_fail: test
tests::test_create_receipt_envelope: test
tests::test_cryptographic_key_isolation: test
tests::test_description_handling: test
tests::test_envelope_metadata_integrity: test
tests::test_envelope_size_and_performance: test
tests::test_envelope_tampering_detection: test
tests::test_envelope_unseal_integration: test
tests::test_invalid_assignment: test
tests::test_key_derivation_determinism: test
tests::test_large_amount_precision: test
tests::test_receipt_chain_verification: test
tests::test_receipt_creation: test
tests::test_receipt_validation: test
tests::test_replay_attack_resistance: test
tests::test_signature_forgery_resistance: test
tests::test_wrong_key_unseal_fails: test
tests::test_zero_amount_validation: test
```


## Package: trustedge-trst-cli (23 tests)

### Integration Tests: acceptance — 7 tests
```
acceptance_a1_signature_flip: test
acceptance_a2_missing_chunk: test
acceptance_a3_swap_chunks: test
acceptance_a4_truncated_chain: test
acceptance_a5_wrong_key: test
acceptance_a6_duration_sanity: test
acceptance_happy_path: test
```

### Integration Tests: integration_tests — 16 tests
```
test_a1_successful_verification: test
test_a2_archive_not_found: test
test_a3_signature_verification_failure: test
test_a4_missing_signature_in_manifest: test
test_a5_continuity_failure_with_json: test
test_emit_request_basic_functionality: test
test_emit_request_blake3_computation: test
test_emit_request_http_error_handling: test
test_invalid_output_directory_name: test
test_seed_deterministic_output: test
test_verify_emit_receipt: test
test_verify_nonexistent_archive: test
test_verify_with_wrong_public_key: test
test_wrap_and_verify_basic_workflow: test
test_wrap_nonexistent_input_file: test
test_wrap_with_existing_device_key: test
```


## Package: trustedge-trst-core (5 tests)

### Unit Tests (lib) — 5 tests

#### Module: manifest (5 tests)
```
manifest::tests::test_canonical_bytes_excludes_signature: test
manifest::tests::test_key_ordering: test
manifest::tests::test_manifest_creation: test
manifest::tests::test_stable_canonicalization: test
manifest::tests::test_validation: test
```


---

## Summary

| Package | Tests |
|---------|-------|
| trustedge-attestation | 10 |
| trustedge-core | 258 |
| trustedge-pubky | 19 |
| trustedge-pubky-advanced | 10 |
| trustedge-receipts | 23 |
| trustedge-trst-cli | 23 |
| trustedge-trst-core | 5 |
| **Total** | **348** |

**Total tests:** 348
