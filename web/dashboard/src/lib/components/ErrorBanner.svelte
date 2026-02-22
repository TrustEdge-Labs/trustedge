<script lang="ts">
	export let error: string | null = null;
	export let dismissible: boolean = true;

	function dismiss() {
		if (dismissible) {
			error = null;
		}
	}
</script>

{#if error}
	<div class="error-banner">
		<span>{error}</span>
		{#if dismissible}
			<button class="dismiss-btn" on:click={dismiss}>×</button>
		{/if}
	</div>
{/if}

<style>
	.error-banner {
		background: #f8d7da;
		color: #721c24;
		padding: 1rem;
		border-radius: 4px;
		margin-bottom: 1rem;
		border: 1px solid #f5c6cb;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dismiss-btn {
		background: none;
		border: none;
		color: #721c24;
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0;
		margin-left: 1rem;
	}

	.dismiss-btn:hover {
		opacity: 0.7;
	}
</style>
