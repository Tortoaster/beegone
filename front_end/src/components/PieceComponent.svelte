<script lang="ts">
	import { send, receive } from '../animation/transition';
	import type { Piece } from '@beegone/beegone';
	import Tile from './Tile.svelte';
	import Token from './Token.svelte';

	export let piece: Piece;

	const VIEW_BOX = 360;
</script>

<svg
	viewBox="0 0 {VIEW_BOX} {VIEW_BOX}"
	class="cursor-pointer"
	in:receive={{ key: piece.id }}
	out:send={{ key: piece.id }}
>
	{#if piece.kind.type === 'wall'}
		<Tile
			height={0.15}
			sideClass="fill-amber-600 dark:fill-slate-600"
			topClass="fill-amber-400 dark:fill-slate-400"
		>
			<Tile size={0.6} height={0} topClass="fill-amber-300 dark:fill-slate-300" />
		</Tile>
	{:else if piece.kind.content.color === 'light'}
		<Token
			on:click
			size={0.45}
			height={0.1}
			sideClass="fill-amber-400 dark:fill-slate-300"
			topClass="fill-amber-300 dark:fill-slate-100"
		>
			<text
				x={VIEW_BOX / 2}
				y={VIEW_BOX / 2}
				dominant-baseline="middle"
				text-anchor="middle"
				class="text-4xl font-bold select-none fill-amber-900 dark:fill-slate-900"
			>
				{piece.kind.content.species}
			</text>
		</Token>
	{:else}
		<Token
			on:click
			size={0.45}
			height={0.1}
			sideClass="fill-amber-900 dark:fill-slate-900"
			topClass="fill-amber-800  dark:fill-slate-800"
		>
			<text
				x={VIEW_BOX / 2}
				y={VIEW_BOX / 2}
				dominant-baseline="middle"
				text-anchor="middle"
				class="text-4xl font-bold select-none fill-amber-300 dark:fill-slate-100"
			>
				{piece.kind.content.species}
			</text>
		</Token>
	{/if}
</svg>
