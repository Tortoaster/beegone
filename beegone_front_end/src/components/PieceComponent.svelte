<script lang="ts">
	import { receive, send } from '../animation/transition.js';
	import { Color } from '../../../beegone_wasm/beegone';
	import type { Piece } from '../../../beegone_wasm/beegone';

	export let piece: Piece;

	$: color = piece.tag == 'Bee' ? (piece.content[0] == Color.Light ? '#FFF' : '#333') : '#FF0';
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
