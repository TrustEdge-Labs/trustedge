<svelte:head>
	<title>TrustEdge Dashboard</title>
</svelte:head>

<div class="card">
	<h1>TrustEdge Dashboard</h1>
	<p>Welcome to the TrustEdge organization dashboard. Monitor receipts, manage devices, and track verification status.</p>

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
