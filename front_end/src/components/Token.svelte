<script lang="ts">
	export let sideClass = 'fill-transparent';
	export let topClass = 'fill-transparent';
	export let size = 0.5;
	export let height = 0.15;

	const viewBox = 360;
	const tokenSize = size * viewBox;
	$: tokenHeight = height * viewBox;
</script>

<svg on:click viewBox="0 0 {viewBox} {viewBox}">
	<circle class={sideClass} cx={viewBox / 2} cy={viewBox / 2} r={tokenSize / 2} />
	<rect
		class={sideClass}
		x={(viewBox - tokenSize) / 2}
		y={viewBox / 2 - Math.max(0, tokenHeight)}
		width={tokenSize}
		height={Math.abs(tokenHeight)}
	/>
	<svg viewBox="0 0 {viewBox} {viewBox}" y={-tokenHeight}>
		<circle class={topClass} cx={viewBox / 2} cy={viewBox / 2} r={tokenSize / 2} />
		<slot />
	</svg>
</svg>
