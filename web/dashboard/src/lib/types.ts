export interface Receipt {
	id: string;
	device_id: string;
	signature: boolean;
	continuity: boolean;
	segments: number;
	issued_at: string;
	payload?: any;
}

export interface Device {
	id: string;
	device_id: string;
	public_key: string;
	label: string;
	created_at: string;
	updated_at: string;
}

export interface ReceiptsResponse {
	receipts: Receipt[];
	total: number;
	page: number;
	limit: number;
}

export interface DevicesResponse {
	devices: Device[];
}

export interface CreateDeviceRequest {
	device_id: string;
	public_key: string;
	label: string;
}
