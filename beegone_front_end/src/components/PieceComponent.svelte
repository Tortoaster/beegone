<script lang="ts">
	import { receive, send } from '../animation/transition.js';
	import type { Piece } from '../../../beegone_wasm/beegone_types';
	import { polygon } from '../utils/polygon';

	export let piece: Piece;

	const viewBox = 360;

	$: color = piece.type == 'bee' ? (piece.content.color == 'light' ? '#FFF' : '#000') : '#FF0';
</script>

{#if piece.type === 'wall'}
	<svg viewBox="0 0 {viewBox} {viewBox}">
		<path class="fill-amber-500" d={polygon(viewBox, 0.46 * viewBox, 0.1 * viewBox)} />
		<slot />
	</svg>
{:else}
	<svg viewBox="0 0 {viewBox} {viewBox}">
		<circle
			fill={color}
			cx="180"
			cy="180"
			r="90"
			in:receive={{ key: piece }}
			out:send={{ key: piece }}
		/>
	</svg>
{/if}
