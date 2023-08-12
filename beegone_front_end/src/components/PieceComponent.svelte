<script lang="ts">
	import type { Piece } from '../../../beegone_wasm/beegone_types';
	import Tile from './Tile.svelte';
	import Token from './Token.svelte';

	export let piece: Piece;
</script>

{#if piece.type === 'wall'}
	<Tile height={0.15} sideClass="fill-amber-500" topClass="fill-amber-400" />
{:else if piece.content.color === 'light'}
	<!-- in:receive={{ key: piece }} -->
	<!-- out:send={{ key: piece }} -->
	<Token size={0.45} height={0.04} sideClass="fill-amber-500">
		<Token size={0.45} height={0.04} sideClass="fill-amber-800">
			<Token size={0.45} height={0.04} sideClass="fill-amber-500" topClass="fill-amber-400" />
		</Token>
	</Token>
{:else}
	<Token size={0.45} height={0.04} sideClass="fill-amber-800">
		<Token size={0.45} height={0.04} sideClass="fill-amber-500">
			<Token size={0.45} height={0.04} sideClass="fill-amber-800" topClass="fill-amber-700" />
		</Token>
	</Token>
{/if}
