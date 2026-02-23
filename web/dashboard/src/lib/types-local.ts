// Dashboard-local types not generated from trustedge-types schemas.
// These are platform-API types specific to the dashboard UI.
//
// The DashboardReceipt type represents the receipt shape returned by the platform
// list/detail API endpoints. It differs from the trustedge-types VerificationReceipt (in types.ts),
// which is the verification receipt wire type issued by the verify engine.

// Dashboard-specific receipt shape (as returned by the platform list/get API).
// signature and continuity are booleans (pass/fail) in the platform UI API.
export interface DashboardReceipt {
	id: string;
	device_id: string;
	signature: boolean;
	continuity: boolean;
	segments: number;
	issued_at: string;
	payload?: unknown;
}

export interface ReceiptsResponse {
	receipts: DashboardReceipt[];
	total: number;
	page: number;
	limit: number;
}

export interface Device {
	id: string;
	device_id: string;
	public_key: string;
	label: string;
	created_at: string;
	updated_at: string;
}

export interface DevicesResponse {
	devices: Device[];
}

export interface CreateDeviceRequest {
	device_id: string;
	public_key: string;
	label: string;
}
