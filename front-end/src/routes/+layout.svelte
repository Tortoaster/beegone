<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-dvh bg-white-dark">
  <div class="container mx-auto min-h-dvh bg-white shadow-lg shadow-shadow">
    {@render children?.()}
  </div>
</div>
