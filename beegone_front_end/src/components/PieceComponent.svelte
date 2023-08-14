<script lang="ts">
	import { send, receive } from '../animation/transition';
	import type { Piece } from '../../../beegone_wasm/beegone_types';
	import Tile from './Tile.svelte';
	import Token from './Token.svelte';

	export let piece: Piece;
</script>

<svg class="cursor-pointer" in:receive={{ key: piece.id }} out:send={{ key: piece.id }}>
	{#if piece.kind.type === 'wall'}
		<Tile
			height={0.15}
			sideClass="fill-amber-600 dark:fill-slate-600"
			topClass="fill-amber-400 dark:fill-slate-500"
		/>
	{:else if piece.kind.content.color === 'light'}
		<Token
			on:click
			size={0.45}
			height={0.1}
			sideClass="fill-amber-400 dark:fill-slate-300"
			topClass="fill-amber-300 dark:fill-slate-100"
		/>
	{:else}
		<Token
			on:click
			size={0.45}
			height={0.1}
			sideClass="fill-amber-900 dark:fill-slate-900"
			topClass="fill-amber-800  dark:fill-slate-800"
		/>
	{/if}
</svg>
