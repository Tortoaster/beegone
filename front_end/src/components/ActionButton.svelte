<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { yeet } from '../animation/yeet';
	import type { Action, Piece } from '@beegone/beegone';
	import DigIcon from '../icons/actions/DigIcon.svelte';
	import BuildIcon from '../icons/actions/BuildIcon.svelte';
	import AttackIcon from '../icons/actions/AttackIcon.svelte';
	import MoveIcon from '../icons/actions/MoveIcon.svelte';
	import ExplorerIcon from '../icons/pieces/ExplorerIcon.svelte';
	import BuilderIcon from '../icons/pieces/BuilderIcon.svelte';
	import GuardIcon from '../icons/pieces/GuardIcon.svelte';
	import DroneIcon from '../icons/pieces/DroneIcon.svelte';
	import WorkerIcon from '../icons/pieces/WorkerIcon.svelte';

	export let action: Action;
	export let x = 0;
	export let y = 0;
	export let delay = 0;
	/**
	 * The piece present on the tile this button is on.
	 */
	export let piece: Piece | undefined;

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

	const VIEW_BOX = 360;
	const ICON_SIZE = VIEW_BOX / 6;
</script>

<svg
	in:yeet|global={inTransition}
	out:yeet|global={outTransition}
	class="group cursor-pointer"
	on:click={dispatchAction}
	viewBox="0 0 ${VIEW_BOX} ${VIEW_BOX}"
>
	<circle
		class="fill-black group-hover:fill-white"
		cx={VIEW_BOX / 2}
		cy={VIEW_BOX / 2}
		r={ICON_SIZE}
	/>
	{#if action.type === 'move'}
		{#if piece === undefined}
			<MoveIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else if piece.type === 'wall'}
			<DigIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else}
			<AttackIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{/if}
	{:else if action.content.spawn.inner.type === 'bee'}
		{#if action.content.spawn.inner.content.species === 'drone'}
			<DroneIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else if action.content.spawn.inner.content.species === 'worker'}
			<WorkerIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else if action.content.spawn.inner.content.species === 'builder'}
			<BuilderIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else if action.content.spawn.inner.content.species === 'explorer'}
			<ExplorerIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else if action.content.spawn.inner.content.species === 'guard'}
			<GuardIcon
				topClass="fill-white group-hover:fill-black"
				x={(VIEW_BOX - ICON_SIZE) / 2}
				y={(VIEW_BOX - ICON_SIZE) / 2}
				width={ICON_SIZE}
				height={ICON_SIZE}
			/>
		{:else}
			<text fill="red">!</text>
		{/if}
	{:else}
		<BuildIcon
			topClass="fill-white group-hover:fill-black"
			x={(VIEW_BOX - ICON_SIZE) / 2}
			y={(VIEW_BOX - ICON_SIZE) / 2}
			width={ICON_SIZE}
			height={ICON_SIZE}
		/>
	{/if}
</svg>
