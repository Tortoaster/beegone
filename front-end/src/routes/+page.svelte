<script lang="ts">
	import BeeComponent from '$lib/components/BeeComponent.svelte';
	import type { Action } from 'beegone'
	import { Color, Species } from 'beegone/src/types';

	const VIEW_BOX = 360;
</script>

<svg
	viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
	xmlns="http://www.w3.org/2000/svg"
	width="100%"
	height="100%"
>
	<BeeComponent bee={{ color: Color.Light, species: Species.Queen }} x={0} y={0} width={10} height={10} />
</svg>
