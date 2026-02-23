// AUTO-GENERATED from trustedge-types JSON schemas.
// Do not edit manually. Run scripts/generate-types.sh to regenerate.
//
// Source: crates/types/tests/fixtures/*.json
// Generator: json-schema-to-typescript (via npx)

export interface VerifyReport {
  chain_tip?: string | null;
  continuity: string;
  device_id: string;
  duration_s: number;
  error?: string | null;
  first_gap_index?: number | null;
  out_of_order?: OutOfOrder | null;
  profile: string;
  segments: number;
  signature: string;
  verify_time_ms: number;
}

export interface OutOfOrder {
  expected: number;
  found: number;
}

export interface VerificationReceipt {
  chain_tip: string;
  continuity: string;
  device_id: string;
  duration_s: number;
  issued_at: string;
  manifest_digest: string;
  profile: string;
  segments: number;
  service_kid: string;
  signature: string;
  verification_id: string;
  [k: string]: unknown;
}

export interface VerifyRequest {
  device_pub: string;
  manifest: unknown;
  options?: VerifyOptions;
  segments: SegmentRef[];
}

export interface VerifyOptions {
  device_id?: string | null;
  return_receipt?: boolean;
}

export interface SegmentRef {
  hash: string;
  index: number;
}

export interface VerifyResponse {
  receipt?: string | null;
  result: VerifyReport;
  verification_id: string;
}
