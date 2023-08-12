<script lang="ts">
	import type { Action } from '../../../beegone_wasm/beegone_types';

	export let action: Action;
</script>

<svg viewBox="0 0 300 300">
	{#if action.type === 'move'}
		<circle class="fill-black hover:fill-white" cx="150" cy="150" r="50" />
	{:else}
		<circle class="fill-black hover:fill-white" cx="150" cy="150" r="50" />
	{/if}
</svg>
