<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { yeet } from '../animation/yeet';
	import type { Action } from '@beegone/beegone';

	export let action: Action;
	export let x = 0;
	export let y = 0;
	export let delay = 0;

	$: inTransition = {
		x: -x,
		y: -y,
		delay: (Math.atan2(y, x) + Math.PI) * 30 + delay,
		duration: 200,
	};
	$: outTransition = {
		x,
		y,
		delay: (Math.atan2(y, x) + Math.PI) * 30 + delay,
		duration: 200,
	};

	const dispatch = createEventDispatcher();

	function dispatchAction() {
		dispatch('action', {
			action: action,
		});
	}
</script>

<svg
	in:yeet|global={inTransition}
	out:yeet|global={outTransition}
	class="group cursor-pointer"
	on:click={dispatchAction}
	viewBox="0 0 300 300"
>
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
