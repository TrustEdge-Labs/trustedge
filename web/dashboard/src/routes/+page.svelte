<script lang="ts">
	import { onMount } from 'svelte';
	import { config } from '$lib/config';

	let apiConfigured = false;

	onMount(() => {
		apiConfigured = !!config.apiKey;
	});
</script>

<svelte:head>
	<title>TrustEdge Dashboard</title>
</svelte:head>

<div class="card">
	<h1>TrustEdge Dashboard</h1>
	<p>Welcome to the TrustEdge organization dashboard. Monitor receipts, manage devices, and track verification status.</p>

	{#if !apiConfigured}
		<div class="error-banner">
			<strong>Configuration Required:</strong> Please set your API key in <code>.env.local</code>
			<br>
			<small>Add: <code>VITE_API_KEY=your-api-key-here</code></small>
		</div>
	{/if}

	<div class="quick-links">
		<a href="/receipts" class="btn btn-primary">View Receipts</a>
		<a href="/devices" class="btn btn-secondary">Manage Devices</a>
	</div>
</div>

<style>
	.quick-links {
		display: flex;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	h1 {
		margin-top: 0;
		color: #212529;
	}

	p {
		color: #6c757d;
		margin-bottom: 1.5rem;
	}

	code {
		background: #f8f9fa;
		padding: 0.25rem 0.5rem;
		border-radius: 3px;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}
</style>
