<script lang="ts">
	import { type Action, Piece, Pos } from 'beegone';
	import ActionButton from './ActionButton.svelte';
	import type { ActionButtonProps } from '$lib/components/ActionButton.svelte';

	interface Props extends Omit<ActionButtonProps, 'action' | 'x' | 'y' | 'fromX' | 'fromY' | 'delay'> {
		actions: Action[];
		cx: number;
		cy: number;
		r: number;
		pos: Pos;
		selected: Pos | null;
		piece: Piece | undefined;
	}

	const { cx, cy, r, actions, selected, pos, ...props }: Props = $props();

	const length = (pos: Pos) => (Math.abs(pos.q) + Math.abs(pos.r) + Math.abs(-pos.q - pos.r)) / 2;

	const difference = $derived(selected === null ? pos : new Pos(selected.q - pos.q, selected.r - pos.r));
	const angle = $derived(selected === null ? 0 : Math.atan2(pos.y - selected.y, pos.x - selected.x));
	const delay = $derived((angle + Math.PI) * 30 + length(difference) * 2 * Math.PI * 15);
</script>

{#if actions.length === 1}
	<ActionButton
		action={actions[0]}
		x={cx}
		y={cy}
		fromX={cx}
		fromY={cy}
		{delay}
		{...props}
	/>
{:else}
	{#each actions as action, index}
		<ActionButton
			{action}
			x={cx + r * Math.cos((index / actions.length) * 2 * Math.PI)}
			y={cy + r * Math.sin((index / actions.length) * 2 * Math.PI)}
			fromX={cx}
			fromY={cy}
			{delay}
			{...props}
		/>
	{/each}
{/if}
