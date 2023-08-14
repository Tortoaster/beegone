<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Action } from '../../../beegone_wasm/beegone_types';

	export let action: Action;

	const dispatch = createEventDispatcher();

	function dispatchAction() {
		dispatch('action', {
			action: action,
		});
	}
</script>

<svg on:click={dispatchAction} viewBox="0 0 300 300">
	{#if action.type === 'move'}
		<circle class="cursor-pointer fill-black hover:fill-white" cx="150" cy="150" r="50" />
	{:else}
		<circle class="cursor-pointer fill-black hover:fill-white" cx="150" cy="150" r="50" />
	{/if}
</svg>
