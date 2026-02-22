<script lang="ts">
	export let status: boolean;
	export let label: string = '';
</script>

<span class="status-pill" class:pass={status} class:fail={!status}>
	{#if status}
		<span class="icon">✓</span>
	{:else}
		<span class="icon">✗</span>
	{/if}
	{#if label}
		<span class="label">{label}</span>
	{/if}
</span>

<style>
	.status-pill {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-pill.pass {
		background: #d4edda;
		color: #155724;
	}

	.status-pill.fail {
		background: #f8d7da;
		color: #721c24;
	}

	.icon {
		font-weight: bold;
	}
</style>
