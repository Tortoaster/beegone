<script lang="ts">
	import type { Pos } from '../../../beegone_wasm/beegone_types';

	export let pos: Pos;
	export let scale: number;
	export let selected: boolean;
	export let highlight: boolean;

	$: x = (scale / 2) * ((3 / 2) * pos.q) + scale * (3 - 0.5);
	$: y = (scale / 2) * ((Math.sqrt(3) / 2) * pos.q + Math.sqrt(3) * pos.r) + scale * (3 - 0.5);

	$: color = selected ? '#BBB' : highlight ? '#BB8' : '#888';
</script>

<svg on:click viewBox="0 0 300 300" {x} {y} width={scale} height={scale}>
	<path fill={color} d="m300,150 l-75,130 l-150,0 l-75,-130 l75,-130 l150,0z" />
	<slot />
</svg>
