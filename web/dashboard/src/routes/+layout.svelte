<script lang="ts">
	import '../app.css';
	import { page } from '$app/state';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	let currentPath = $derived(page.url.pathname);
</script>

<nav class="nav">
	<div class="nav-container">
		<a href="/" class:active={currentPath === '/'}>
			TrustEdge Dashboard
		</a>
		<a href="/receipts" class:active={currentPath.startsWith('/receipts')}>
			Receipts
		</a>
		<a href="/devices" class:active={currentPath === '/devices'}>
			Devices
		</a>
	</div>
</nav>

<main class="container">
	{@render children()}
</main>
