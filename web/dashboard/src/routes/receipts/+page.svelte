<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import type { ApiError } from '$lib/api';
	import type { Receipt, ReceiptsResponse } from '$lib/types';
	import { config } from '$lib/config';
	import StatusPill from '$lib/components/StatusPill.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	let receipts: Receipt[] = [];
	let loading = true;
	let error: string | null = null;
	let total = 0;
	let page = 1;
	let limit = 50;

	let filters = {
		device_id: '',
		start_date: '',
		end_date: ''
	};

	async function loadReceipts() {
		loading = true;
		error = null;

		try {
			const params = new URLSearchParams({
				limit: limit.toString(),
				page: page.toString()
			});

			if (filters.device_id) {
				params.append('device_id', filters.device_id);
			}
			if (filters.start_date) {
				params.append('start_date', filters.start_date);
			}
			if (filters.end_date) {
				params.append('end_date', filters.end_date);
			}

			const response: ReceiptsResponse = await api.get(`/v1/receipts?${params}`);
			receipts = response.receipts || [];
			total = response.total || 0;
		} catch (err) {
			const apiErr = err as ApiError;
			error = apiErr.message;
			receipts = [];
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	function applyFilters() {
		page = 1;
		loadReceipts();
	}

	function clearFilters() {
		filters = { device_id: '', start_date: '', end_date: '' };
		page = 1;
		loadReceipts();
	}

	onMount(() => {
		loadReceipts();
	});
</script>

<svelte:head>
	<title>Receipts - TrustEdge Dashboard</title>
</svelte:head>

<div class="header">
	<h1>Receipts</h1>
	<p>Digital verification receipts from your TrustEdge devices</p>
</div>

<ErrorBanner {error} />

<div class="card">
	<div class="filters">
		<h3>Filters</h3>
		<div class="filter-row">
			<div class="form-group">
				<label class="form-label" for="device_id">Device ID</label>
				<input
					class="form-input"
					id="device_id"
					type="text"
					placeholder="Filter by device ID"
					bind:value={filters.device_id}
				/>
			</div>
			<div class="form-group">
				<label class="form-label" for="start_date">Start Date</label>
				<input
					class="form-input"
					id="start_date"
					type="date"
					bind:value={filters.start_date}
				/>
			</div>
			<div class="form-group">
				<label class="form-label" for="end_date">End Date</label>
				<input
					class="form-input"
					id="end_date"
					type="date"
					bind:value={filters.end_date}
				/>
			</div>
		</div>
		<div class="filter-actions">
			<button class="btn btn-primary" on:click={applyFilters}>Apply Filters</button>
			<button class="btn btn-secondary" on:click={clearFilters}>Clear</button>
		</div>
	</div>
</div>

{#if loading}
	<div class="card">
		<div class="loading">Loading receipts...</div>
	</div>
{:else if receipts.length === 0}
	<div class="card">
		<div class="empty-state">
			<h3>No receipts found</h3>
			<p>There are no verification receipts yet. Try creating one with curl:</p>
			<pre><code>curl -X POST {config.apiBase}/v1/receipts \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{JSON.stringify({ device_id: "cam-001", data: "sample verification data" })}'</code></pre>
		</div>
	</div>
{:else}
	<div class="card">
		<div class="table-header">
			<h3>Receipts ({total} total)</h3>
		</div>
		<table class="table">
			<thead>
				<tr>
					<th>ID</th>
					<th>Device ID</th>
					<th>Signature</th>
					<th>Continuity</th>
					<th>Segments</th>
					<th>Issued At</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each receipts as receipt (receipt.id)}
					<tr>
						<td>
							<code class="receipt-id">{receipt.id.substring(0, 8)}...</code>
						</td>
						<td>{receipt.device_id}</td>
						<td>
							<StatusPill status={receipt.signature} />
						</td>
						<td>
							<StatusPill status={receipt.continuity} />
						</td>
						<td>{receipt.segments}</td>
						<td>{formatDate(receipt.issued_at)}</td>
						<td>
							<a href="/receipts/{receipt.id}" class="btn btn-primary btn-sm">
								View
							</a>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>

		{#if total > limit}
			<div class="pagination">
				<button
					class="btn btn-secondary"
					disabled={page <= 1}
					on:click={() => {
						page--;
						loadReceipts();
					}}
				>
					Previous
				</button>
				<span>Page {page} of {Math.ceil(total / limit)}</span>
				<button
					class="btn btn-secondary"
					disabled={page >= Math.ceil(total / limit)}
					on:click={() => {
						page++;
						loadReceipts();
					}}
				>
					Next
				</button>
			</div>
		{/if}
	</div>
{/if}

<style>
	.header {
		margin-bottom: 2rem;
	}

	.header h1 {
		margin: 0 0 0.5rem 0;
		color: #212529;
	}

	.header p {
		margin: 0;
		color: #6c757d;
	}

	.filters h3 {
		margin-top: 0;
		margin-bottom: 1rem;
	}

	.filter-row {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.filter-actions {
		display: flex;
		gap: 0.5rem;
	}

	.table-header {
		margin-bottom: 1rem;
	}

	.table-header h3 {
		margin: 0;
	}

	.receipt-id {
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		background: #f8f9fa;
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-size: 0.875rem;
	}

	.btn-sm {
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: #6c757d;
	}

	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e9ecef;
	}

	pre {
		background: #f8f9fa;
		padding: 1rem;
		border-radius: 4px;
		overflow-x: auto;
		font-size: 0.875rem;
	}

	code {
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}

	@media (max-width: 768px) {
		.filter-row {
			grid-template-columns: 1fr;
		}

		.filter-actions {
			flex-direction: column;
		}
	}
</style>
