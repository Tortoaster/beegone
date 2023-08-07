<script lang="ts">
	import { Pos } from '../../../beegone_wasm/beegone';

	export let pos: Pos;
	export let scale: number;
	export let selected: boolean;

	$: x = pos.x(scale);
	$: y = pos.y(scale);

	$: color = selected ? '#BBB' : '#888';
</script>

<svg on:click viewBox="0 0 300 300" {x} {y} width={scale} height={scale}>
	<path fill={color} d="m300,150 l-75,130 l-150,0 l-75,-130 l75,-130 l150,0z" />
	<slot />
</svg>
