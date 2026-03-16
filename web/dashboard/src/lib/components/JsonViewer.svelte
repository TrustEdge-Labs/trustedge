<script lang="ts">
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- JSON.stringify accepts any serializable value
	let { data, title = 'JSON Data' }: { data: any; title?: string } = $props();

	let copied = $state(false);

	let jsonString = $derived.by(() => {
		try {
			return JSON.stringify(data, null, 2);
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		} catch (_) {
			return 'Invalid JSON data';
		}
	});

	async function copyJson() {
		if (navigator.clipboard) {
			await navigator.clipboard.writeText(jsonString);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		}
	}
</script>

<div class="json-viewer">
	<div class="header">
		<h3>{title}</h3>
		<button class="btn btn-secondary" onclick={copyJson}>
			{copied ? 'Copied!' : 'Copy JSON'}
		</button>
	</div>
	<pre class="json-content"><code>{jsonString}</code></pre>
</div>

<style>
	.json-viewer {
		border: 1px solid #e9ecef;
		border-radius: 8px;
		overflow: hidden;
		margin: 1rem 0;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e9ecef;
	}

	.header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
	}

	.json-content {
		padding: 1rem;
		margin: 0;
		background: white;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.875rem;
		line-height: 1.5;
		overflow-x: auto;
		white-space: pre-wrap;
	}
</style>
