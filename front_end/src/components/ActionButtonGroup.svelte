<script lang="ts">
	import type { Action, Pos, Piece } from '@beegone/beegone';
	import ActionButton from './ActionButton.svelte';

	export let cx = 0;
	export let cy = 0;
	export let r = 0;
	export let size = 1;
	export let actions: Action[];
	export let selected: Pos | null;
	export let pos: Pos;
	export let piece: Piece | undefined;

	$: difference = selected === null ? pos : { q: selected.q - pos.q, r: selected.r - pos.r };
	$: angle = selected === null ? 0 : Math.atan2(y(pos) - y(selected), x(pos) - x(selected));
	$: delay = (angle + Math.PI) * 30 + length(difference) * 2 * Math.PI * 30;

	const x = (pos: Pos) => (3 / 2) * pos.q - 1;
	const y = (pos: Pos) => (Math.sqrt(3) / 2) * pos.q + Math.sqrt(3) * pos.r - 1;
	const length = (pos: Pos) => (Math.abs(pos.q) + Math.abs(pos.r) + Math.abs(-pos.q - pos.r)) / 2;
</script>

{#if actions.length === 1}
	<ActionButton
		x={cx}
		y={cy}
		fromX={cx}
		fromY={cy}
		{size}
		{piece}
		{delay}
		action={actions[0]}
		on:action
	/>
{:else}
	{#each actions as action, index}
		<ActionButton
			x={cx + r * Math.cos((index / actions.length) * 2 * Math.PI)}
			y={cy + r * Math.sin((index / actions.length) * 2 * Math.PI)}
			fromX={cx}
			fromY={cy}
			{size}
			{piece}
			{delay}
			{action}
			on:action
		/>
	{/each}
{/if}
