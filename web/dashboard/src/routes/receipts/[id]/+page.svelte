<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { api } from '$lib/api';
	import type { ApiError } from '$lib/api';
	import type { DashboardReceipt } from '$lib/types-local';
	import StatusPill from '$lib/components/StatusPill.svelte';
	import KeyValue from '$lib/components/KeyValue.svelte';
	import JsonViewer from '$lib/components/JsonViewer.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	let receipt: DashboardReceipt | null = null;
	let loading = true;
	let error: string | null = null;
	let receiptId: string;

	$: receiptId = $page.params.id;

	async function loadReceipt() {
		if (!receiptId) return;

		loading = true;
		error = null;

		try {
			receipt = await api.get(`/v1/receipts/${receiptId}`);
		} catch (err) {
			const apiErr = err as ApiError;
			error = apiErr.message;
			receipt = null;
		} finally {
			loading = false;
		}
	}

	function downloadReceipt() {
		if (!receipt || !receipt.payload) return;

		const dataStr = JSON.stringify(receipt.payload, null, 2);
		const dataBlob = new Blob([dataStr], { type: 'application/json' });
		const url = URL.createObjectURL(dataBlob);

		const link = document.createElement('a');
		link.href = url;
		link.download = `receipt-${receipt.id}.json`;
		link.click();

		URL.revokeObjectURL(url);
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	function generateShareUrl(): string {
		return `${window.location.origin}/receipts/${receiptId}/public`;
	}

	onMount(() => {
		loadReceipt();
	});
</script>

<svelte:head>
	<title>Receipt {receiptId} - TrustEdge Dashboard</title>
</svelte:head>

<div class="header">
	<div class="breadcrumb">
		<a href="/receipts">← Back to Receipts</a>
	</div>
	<h1>Receipt Details</h1>
</div>

<ErrorBanner {error} />

{#if loading}
	<div class="card">
		<div class="loading">Loading receipt details...</div>
	</div>
{:else if !receipt}
	<div class="card">
		<div class="empty-state">
			<h3>Receipt not found</h3>
			<p>The receipt with ID <code>{receiptId}</code> could not be found.</p>
		</div>
	</div>
{:else}
	<div class="card">
		<div class="receipt-header">
			<h2>Receipt {receipt.id}</h2>
			<div class="actions">
				<button class="btn btn-primary" on:click={downloadReceipt}>
					Download receipt.json
				</button>
				<a href={generateShareUrl()} class="btn btn-secondary" target="_blank">
					Open public share page
				</a>
			</div>
		</div>

		<div class="verification-status">
			<h3>Verification Status</h3>
			<div class="status-grid">
				<div class="status-item">
					<StatusPill status={receipt.signature} label="Signature Valid" />
				</div>
				<div class="status-item">
					<StatusPill status={receipt.continuity} label="Continuity Verified" />
				</div>
			</div>
		</div>

		<div class="receipt-details">
			<h3>Receipt Information</h3>
			<div class="details-grid">
				<KeyValue label="Receipt ID" value={receipt.id} copyable={true} />
				<KeyValue label="Device ID" value={receipt.device_id} copyable={true} />
				<KeyValue label="Segments" value={receipt.segments} />
				<KeyValue label="Issued At" value={formatDate(receipt.issued_at)} />
			</div>
		</div>

		{#if receipt.payload}
			<JsonViewer data={receipt.payload} title="JWS Payload (Claims)" />
		{:else}
			<div class="card">
				<div class="empty-state">
					<h3>No payload data</h3>
					<p>The JWS payload for this receipt is not available.</p>
				</div>
			</div>
		{/if}
	</div>
{/if}

<style>
	.header {
		margin-bottom: 2rem;
	}

	.breadcrumb {
		margin-bottom: 1rem;
	}

	.breadcrumb a {
		color: #007bff;
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.header h1 {
		margin: 0;
		color: #212529;
	}

	.receipt-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid #e9ecef;
	}

	.receipt-header h2 {
		margin: 0;
		font-size: 1.5rem;
		word-break: break-all;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.verification-status {
		margin-bottom: 2rem;
	}

	.verification-status h3 {
		margin-bottom: 1rem;
	}

	.status-grid {
		display: flex;
		gap: 1rem;
	}

	.status-item {
		display: flex;
		align-items: center;
	}

	.receipt-details h3 {
		margin-bottom: 1rem;
	}

	.details-grid {
		display: grid;
		gap: 0.5rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: #6c757d;
	}

	code {
		background: #f8f9fa;
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}

	@media (max-width: 768px) {
		.receipt-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.actions {
			width: 100%;
			flex-direction: column;
		}

		.status-grid {
			flex-direction: column;
		}
	}
</style>
