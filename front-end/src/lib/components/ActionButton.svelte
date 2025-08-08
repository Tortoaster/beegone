<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fly } from 'svelte/transition';
	import type { Action, Piece } from 'beegone';

	export let action: Action;
	export let x = 0;
	export let y = 0;
	export let size = 1;
	export let piece: Piece | undefined = undefined;
	export let fromX = 0;
	export let fromY = 0;
	export let delay = 0;

	$: inTransition = {
		x: fromX - x,
		y: fromY - y,
		delay: (Math.atan2(y - fromY, x - fromX) + Math.PI) * 30 + delay,
		duration: 200,
	};
	$: outTransition = {
		x: x - fromX,
		y: y - fromY,
		duration: 200,
	};
	$: icon =
		action.move
			? piece === undefined
				? 'move'
				: piece.bee
				? 'attack'
				: 'dig'
			: action.spawn?.spawn.bee
			? action.spawn?.spawn.bee.species
			: 'build';

	const dispatch = createEventDispatcher();
	function dispatchAction() {
		dispatch('action', {
			action,
		});
	}
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	x={x - size / 2}
	y={y - size / 2}
	width={size}
	height={size}
	class="group cursor-pointer select-none"
	in:fly|global={inTransition}
	out:fly|global={outTransition}
	on:click={dispatchAction}
>
	<defs>
		<filter id="make-white">
			<feFlood flood-color="white" />
			<feComposite operator="in" in2="SourceGraphic" />
		</filter>
	</defs>
	<circle class="fill-black group-hover:fill-white" cx="50%" cy="50%" r="50%" />
	<image
		xlink:href="/{icon}.svg"
		class="group-hover:filter-none"
		x="25%"
		y="25%"
		width="50%"
		height="50%"
		filter="url(#make-white)"
	/>
</svg>
