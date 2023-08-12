<script lang="ts">
	import { polygon } from '../utils/polygon';
	import type { Pos } from '../../../beegone_wasm/beegone_types';

	export let pos: Pos;
	export let scale: number;

	$: x = (scale / 2) * ((3 / 2) * pos.q) + scale * (3 - 0.5);
	$: y = (scale / 2) * ((Math.sqrt(3) / 2) * pos.q + Math.sqrt(3) * pos.r) + scale * (3 - 0.5);

	const viewBox = 360;
</script>

<svg viewBox="0 0 {viewBox} {viewBox}" {x} {y} width={scale} height={scale}>
	<path class="fill-amber-300" d={polygon(viewBox, 0.45 * viewBox, 0.1 * viewBox)} />
	<slot />
</svg>
