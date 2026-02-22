import { config } from './config';

export interface ApiError {
	message: string;
	status: number;
	code?: string;
}

export class ApiClient {
	private baseUrl: string;
	private apiKey: string;

	constructor() {
		this.baseUrl = config.apiBase;
		this.apiKey = config.apiKey;
	}

	private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		const url = `${this.baseUrl}${endpoint}`;

		const headers = {
			'Content-Type': 'application/json',
			'Authorization': `Bearer ${this.apiKey}`,
			...options.headers
		};

		try {
			const response = await fetch(url, {
				...options,
				headers
			});

			if (!response.ok) {
				const error: ApiError = {
					message: `HTTP ${response.status}: ${response.statusText}`,
					status: response.status
				};

				if (response.status === 401) {
					error.message = 'Invalid API key or unauthorized access';
					error.code = 'UNAUTHORIZED';
				} else if (response.status === 429) {
					error.message = 'Rate limit exceeded';
					error.code = 'RATE_LIMIT';
				}

				throw error;
			}

			return await response.json();
		} catch (err) {
			if (err instanceof Error && 'status' in err) {
				throw err;
			}

			throw {
				message: 'Network error or server unavailable',
				status: 0,
				code: 'NETWORK_ERROR'
			} as ApiError;
		}
	}

	async get<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint, { method: 'GET' });
	}

	async post<T>(endpoint: string, data?: any): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'POST',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async put<T>(endpoint: string, data?: any): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'PUT',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async delete<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint, { method: 'DELETE' });
	}
}

export const api = new ApiClient();
