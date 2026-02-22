<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import type { ApiError } from '$lib/api';
	import type { Device, DevicesResponse, CreateDeviceRequest } from '$lib/types';
	import KeyValue from '$lib/components/KeyValue.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	let devices: Device[] = [];
	let loading = true;
	let error: string | null = null;
	let submitError: string | null = null;
	let submitting = false;
	let showRegistrationForm = false;

	let newDevice: CreateDeviceRequest = {
		device_id: '',
		public_key: '',
		label: ''
	};

	async function loadDevices() {
		loading = true;
		error = null;

		try {
			const response: DevicesResponse = await api.get('/v1/devices');
			devices = response.devices || [];
		} catch (err) {
			const apiErr = err as ApiError;
			error = apiErr.message;
			devices = [];
		} finally {
			loading = false;
		}
	}

	async function registerDevice() {
		if (!newDevice.device_id || !newDevice.public_key || !newDevice.label) {
			submitError = 'All fields are required';
			return;
		}

		if (!newDevice.public_key.startsWith('ed25519:')) {
			submitError = 'Public key must start with "ed25519:"';
			return;
		}

		submitting = true;
		submitError = null;

		try {
			await api.post('/v1/devices', newDevice);
			newDevice = { device_id: '', public_key: '', label: '' };
			showRegistrationForm = false;
			await loadDevices();
		} catch (err) {
			const apiErr = err as ApiError;
			submitError = apiErr.message;
		} finally {
			submitting = false;
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	function cancelRegistration() {
		showRegistrationForm = false;
		newDevice = { device_id: '', public_key: '', label: '' };
		submitError = null;
	}

	onMount(() => {
		loadDevices();
	});
</script>

<svelte:head>
	<title>Devices - TrustEdge Dashboard</title>
</svelte:head>

<div class="header">
	<div class="header-content">
		<div>
			<h1>Devices</h1>
			<p>Manage TrustEdge devices and their cryptographic keys</p>
		</div>
		<button
			class="btn btn-primary"
			on:click={() => showRegistrationForm = !showRegistrationForm}
		>
			{showRegistrationForm ? 'Cancel' : 'Register Device'}
		</button>
	</div>
</div>

<ErrorBanner {error} />

{#if showRegistrationForm}
	<div class="card">
		<h3>Register New Device</h3>

		<ErrorBanner error={submitError} />

		<form on:submit|preventDefault={registerDevice}>
			<div class="form-group">
				<label class="form-label" for="device_id">Device ID</label>
				<input
					class="form-input"
					id="device_id"
					type="text"
					placeholder="e.g., cam-001, sensor-123"
					bind:value={newDevice.device_id}
					required
				/>
			</div>

			<div class="form-group">
				<label class="form-label" for="public_key">Ed25519 Public Key</label>
				<input
					class="form-input"
					id="public_key"
					type="text"
					placeholder="ed25519:GAUpGXoor5gP..."
					bind:value={newDevice.public_key}
					required
				/>
				<small class="form-help">Must start with "ed25519:" followed by the base64-encoded key</small>
			</div>

			<div class="form-group">
				<label class="form-label" for="label">Label</label>
				<input
					class="form-input"
					id="label"
					type="text"
					placeholder="Descriptive name for this device"
					bind:value={newDevice.label}
					required
				/>
			</div>

			<div class="form-actions">
				<button
					type="submit"
					class="btn btn-primary"
					disabled={submitting}
				>
					{submitting ? 'Registering...' : 'Register Device'}
				</button>
				<button
					type="button"
					class="btn btn-secondary"
					on:click={cancelRegistration}
				>
					Cancel
				</button>
			</div>
		</form>
	</div>
{/if}

{#if loading}
	<div class="card">
		<div class="loading">Loading devices...</div>
	</div>
{:else if devices.length === 0}
	<div class="card">
		<div class="empty-state">
			<h3>No devices registered</h3>
			<p>Register your first TrustEdge device to start generating verification receipts.</p>
			<button
				class="btn btn-primary"
				on:click={() => showRegistrationForm = true}
			>
				Register Device
			</button>
		</div>
	</div>
{:else}
	<div class="devices-grid">
		{#each devices as device (device.id)}
			<div class="card device-card">
				<div class="device-header">
					<h3>{device.label}</h3>
					<code class="device-id">{device.device_id}</code>
				</div>

				<div class="device-details">
					<KeyValue label="Device ID" value={device.device_id} copyable={true} />
					<KeyValue label="Public Key" value={device.public_key} copyable={true} />
					<KeyValue label="Registered" value={formatDate(device.created_at)} />
					<KeyValue label="Last Updated" value={formatDate(device.updated_at)} />
				</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	.header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.header h1 {
		margin: 0 0 0.5rem 0;
		color: #212529;
	}

	.header p {
		margin: 0;
		color: #6c757d;
	}

	.form-help {
		color: #6c757d;
		font-size: 0.875rem;
		margin-top: 0.25rem;
	}

	.form-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 1.5rem;
	}

	.devices-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
		gap: 1.5rem;
	}

	.device-card {
		background: white;
		border: 1px solid #e9ecef;
		border-radius: 8px;
		padding: 1.5rem;
	}

	.device-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid #e9ecef;
	}

	.device-header h3 {
		margin: 0;
		color: #212529;
	}

	.device-id {
		background: #f8f9fa;
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.875rem;
	}

	.device-details {
		display: grid;
		gap: 0.5rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: #6c757d;
	}

	@media (max-width: 768px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}

		.devices-grid {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
		}

		.device-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}
	}
</style>
