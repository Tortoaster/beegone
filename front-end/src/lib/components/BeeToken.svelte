<script lang="ts">
	import { type Bee, Color, Species } from 'beegone';
	import { speciesName } from '$lib/species-name';

	export let bee: Bee;

	export let filter = '';
	export let x: number | string = 0;
	export let y: number | string = 0;
	export let width: number | string = '100%';
	export let height: number | string = '100%';

	$: topClass =
		bee.color === Color.Light
			? 'fill-amber-300'
			: 'fill-amber-900 ';
	$: sideFilter = bee.color === Color.Light ? 'url(#light-token-side)' : 'url(#dark-token-side)';
	$: iconFilter = bee.color === Color.Light ? 'url(#light-icon-color)' : 'url(#dark-icon-color)';
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	class="cursor-pointer select-none"
	on:click
	{filter}
	{width}
	{height}
	{x}
	{y}
>
	<defs>
		<filter id="light-token-side" y="-25%" height="125%">
			<feFlood
				class="flood-amber-400"
				result="Color"
			/>
			<feComposite operator="in" in="Color" in2="SourceGraphic" result="Shadow" />
			<feOffset in="Shadow" dy="-3" result="Shadow3" />
			<feOffset in="SourceGraphic" dy="-6" result="Offset" />
			<feMerge>
				<feMergeNode in="Shadow" />
				<feMergeNode in="Shadow3" />
				<feMergeNode in="Offset" />
			</feMerge>
		</filter>
		<filter id="dark-token-side" y="-25%" height="125%">
			<feFlood
				class="flood-amber-950"
				result="Color"
			/>
			<feComposite operator="in" in="Color" in2="SourceGraphic" result="Shadow" />
			<feOffset in="Shadow" dy="-3" result="Shadow3" />
			<feOffset in="SourceGraphic" dy="-6" result="Offset" />
			<feMerge>
				<feMergeNode in="Shadow" />
				<feMergeNode in="Shadow3" />
				<feMergeNode in="Offset" />
			</feMerge>
		</filter>
		<filter id="light-icon-color" y="-40%" height="140%">
			<feFlood class="flood-amber-500" />
			<feComposite operator="in" in2="SourceGraphic" />
			<feOffset dy="-6" />
		</filter>
		<filter id="dark-icon-color" y="-40%" height="140%">
			<feFlood class="flood-amber-700" />
			<feComposite operator="in" in2="SourceGraphic" />
			<feOffset dy="-6" />
		</filter>
	</defs>
	<circle cx="50%" cy="50%" r="27.5%" class={topClass} filter={sideFilter} />
	<image
		xlink:href="/{speciesName(bee.species)}.svg"
		x="35%"
		y="35%"
		width="30%"
		height="30%"
		filter={iconFilter}
	/>
</svg>
