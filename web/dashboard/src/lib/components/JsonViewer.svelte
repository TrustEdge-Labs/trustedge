<script lang="ts">
	export let data: any;
	export let title: string = 'JSON Data';

	let jsonString: string;
	let copied = false;

	$: {
		try {
			jsonString = JSON.stringify(data, null, 2);
		} catch (e) {
			jsonString = 'Invalid JSON data';
		}
	}

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
		<button class="btn btn-secondary" on:click={copyJson}>
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
