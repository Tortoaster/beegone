<script lang="ts">
	import { polygon } from '../utils/polygon';

	export let sideClass = 'fill-transparent';
	export let topClass = 'fill-transparent';
	export let size = 0.8;
	export let height = 0.15;

	const viewBox = 360;
	const tileSize = size * viewBox;
	$: tileHeight = height * viewBox;
	const cornerRadius = 0.1 * viewBox;
	const margin = cornerRadius - Math.cos(Math.PI / 6) * cornerRadius;
</script>

<svg viewBox="0 0 {viewBox} {viewBox}">
	<path class={sideClass} d={polygon(viewBox, tileSize / 2, cornerRadius)} />
	<rect
		class={sideClass}
		x={(viewBox - tileSize) / 2 + margin}
		y={viewBox / 2 - Math.max(0, tileHeight)}
		width={tileSize - margin * 2}
		height={Math.abs(tileHeight)}
	/>
	<svg viewBox="0 0 {viewBox} {viewBox}" y={-tileHeight}>
		<path class={topClass} d={polygon(viewBox, tileSize / 2, cornerRadius)} />
		<slot />
	</svg>
</svg>
