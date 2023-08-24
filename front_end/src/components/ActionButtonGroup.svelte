<script lang="ts">
	import type { Action, Pos } from '@beegone/beegone';
	import ActionButton from './ActionButton.svelte';

	export let actions: Action[];
	export let selected: Pos | null;
	export let pos: Pos;

	$: difference = selected === null ? pos : { q: selected.q - pos.q, r: selected.r - pos.r };
	$: angle = Math.atan2(y(difference), x(difference));
	$: delay = (angle + Math.PI) * 30 + length(difference) * 60;

	const x = (pos: Pos) => (3 / 2) * pos.q - 1;
	const y = (pos: Pos) => (Math.sqrt(3) / 2) * pos.q + Math.sqrt(3) * pos.r - 1;
	const length = (pos: Pos) => (Math.abs(pos.q) + Math.abs(pos.r) + Math.abs(-pos.q - pos.r)) / 2;
</script>

{#if actions.length !== 0}
	<svg viewBox="0 0 360 360">
		{#if actions.length === 1}
			<ActionButton {delay} on:action action={actions[0]} />
		{:else if actions.length === 2}
			<svg viewBox="0 0 360 360" x="-90">
				<ActionButton {delay} x={-90} on:action action={actions[0]} />
			</svg>
			<svg viewBox="0 0 360 360" x="90">
				<ActionButton {delay} x={90} on:action action={actions[1]} />
			</svg>
		{:else if actions.length === 3}
			<svg viewBox="0 0 360 360" x="45" y="-78">
				<ActionButton {delay} x={45} y={-78} on:action action={actions[0]} />
			</svg>
			<svg viewBox="0 0 360 360" x="45" y="78">
				<ActionButton {delay} x={45} y={78} on:action action={actions[1]} />
			</svg>
			<svg viewBox="0 0 360 360" x="-90">
				<ActionButton {delay} x={-90} on:action action={actions[2]} />
			</svg>
		{:else}
			<text fill="red">!</text>
		{/if}
	</svg>
{/if}
