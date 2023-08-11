<script lang="ts">
	import { receive, send } from '../animation/transition.js';
	import type { Piece } from '../../../beegone_wasm/beegone_types';

	export let piece: Piece;

	$: color = piece.type == 'bee' ? (piece.content.color == 'light' ? '#FFF' : '#000') : '#FF0';
</script>

<circle
	on:click
	fill={color}
	cx="150"
	cy="150"
	r="50"
	in:receive={{ key: piece }}
	out:send={{ key: piece }}
/>
