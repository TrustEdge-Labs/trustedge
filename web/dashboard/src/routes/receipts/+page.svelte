<script lang="ts">
	import { config } from '$lib/config';
</script>

<svelte:head>
	<title>Receipts - Sealedge Dashboard</title>
</svelte:head>

<div class="header">
	<h1>Receipts</h1>
	<p>Digital verification receipts from your Sealedge devices</p>
</div>

<div class="card">
	<div class="notice">
		<h3>Admin access required</h3>
		<p>The receipts list is available through the platform API directly. This dashboard provides public verification only.</p>
		<p>To query receipts, use the platform API with your admin credentials:</p>
		<pre><code>curl {config.apiBase}/v1/receipts \
  -H "Authorization: Bearer &lt;admin-token&gt;"</code></pre>
	</div>
</div>

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
</style>
