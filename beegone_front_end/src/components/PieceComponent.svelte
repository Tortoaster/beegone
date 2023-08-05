<script lang="ts">
	import {receive, send} from '../animation/transition.js';
	import type {Piece} from "../model/model";
	import {Color} from "../model/model";

	export let piece: Piece;

	let color = '#00F';
	$: color = piece.type == "Bee" ? (piece.content[0] == Color.Light ? '#FFF' : '#000') : '#FF0';
</script>

<circle on:click fill={color} cx="150" cy="150" r="50"
		in:receive={{ key: piece }}
		out:send={{ key: piece }}/>
