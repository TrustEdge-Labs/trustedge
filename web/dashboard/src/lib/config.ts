// API base URL is the only required configuration.
// No API key is needed — the dashboard accesses public endpoints only.
export const config = {
	apiBase: import.meta.env.VITE_API_BASE || 'http://localhost:3001'
};
