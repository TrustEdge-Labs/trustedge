<script lang="ts">
	export let label: string;
	export let value: string | number | boolean;
	export let copyable: boolean = false;

	async function copyToClipboard() {
		if (copyable && navigator.clipboard) {
			await navigator.clipboard.writeText(String(value));
		}
	}
</script>

<div class="key-value">
	<div class="label">{label}:</div>
	<div class="value">
		<span>{value}</span>
		{#if copyable}
			<button class="copy-btn" on:click={copyToClipboard} title="Copy to clipboard">
				[copy]
			</button>
		{/if}
	</div>
</div>

<style>
	.key-value {
		display: flex;
		margin-bottom: 0.5rem;
		gap: 1rem;
	}

	.label {
		font-weight: 600;
		color: #495057;
		min-width: 100px;
	}

	.value {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		word-break: break-all;
	}

	.copy-btn {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 2px;
		opacity: 0.7;
		font-size: 0.75rem;
		color: #495057;
	}

	.copy-btn:hover {
		opacity: 1;
		background: #f8f9fa;
	}
</style>
