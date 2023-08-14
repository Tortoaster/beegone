<script lang="ts">
	import type { Piece } from '../../../beegone_wasm/beegone_types';
	import Tile from './Tile.svelte';
	import Token from './Token.svelte';

	export let piece: Piece;
</script>

{#if piece.type === 'wall'}
	<Tile
		height={0.15}
		sideClass="fill-amber-500 dark:fill-slate-600"
		topClass="fill-amber-400 dark:fill-slate-500"
	/>
{:else if piece.content.color === 'light'}
	<!-- in:receive={{ key: piece }} -->
	<!-- out:send={{ key: piece }} -->
	<Token size={0.45} height={0.04} sideClass="fill-amber-500 dark:fill-slate-300">
		<Token size={0.45} height={0.04} sideClass="fill-amber-800 dark:fill-slate-300">
			<Token
				size={0.45}
				height={0.04}
				sideClass="fill-amber-500 dark:fill-slate-300"
				topClass="fill-amber-400 dark:fill-slate-100"
			/>
		</Token>
	</Token>
{:else}
	<Token size={0.45} height={0.04} sideClass="fill-amber-800 dark:fill-slate-900">
		<Token size={0.45} height={0.04} sideClass="fill-amber-500 dark:fill-slate-900">
			<Token
				size={0.45}
				height={0.04}
				sideClass="fill-amber-800 dark:fill-slate-900"
				topClass="fill-amber-700  dark:fill-slate-800"
			/>
		</Token>
	</Token>
{/if}
