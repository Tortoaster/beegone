<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Action } from '@beegone/beegone';

	export let action: Action;

	const dispatch = createEventDispatcher();

	function dispatchAction() {
		dispatch('action', {
			action: action,
		});
	}
</script>

<svg class="group cursor-pointer" on:click={dispatchAction} viewBox="0 0 300 300">
	{#if action.type === 'move'}
		<circle class="fill-black group-hover:fill-white" cx="150" cy="150" r="50" />
		<text
			x="150"
			y="150"
			dominant-baseline="middle"
			text-anchor="middle"
			class="text-2xl font-bold select-none fill-white group-hover:fill-black">Move</text
		>
	{:else}
		<circle class="fill-black group-hover:fill-white" cx="150" cy="150" r="50" />
		<text
			x="150"
			y="150"
			dominant-baseline="middle"
			text-anchor="middle"
			class="text-2xl font-bold select-none fill-white group-hover:fill-black"
		>
			{action.content.spawn.kind.type == 'bee' ? action.content.spawn.kind.content.species : 'Wall'}
		</text>
	{/if}
</svg>
