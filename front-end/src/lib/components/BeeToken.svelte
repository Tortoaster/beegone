<script lang="ts">
	import { type Bee } from 'beegone';
	import SpeciesIcon from '$lib/icons/SpeciesIcon.svelte';

	interface Props {
		bee: Bee;
		filter?: string;
		x: number;
		y: number;
		width: number;
		height: number;
		onclick?: () => void;
	}

	const { bee, filter, x, y, width, height, onclick }: Props = $props();

	const topClass = $derived(
		bee.color === 'light'
			? "fill-amber-300"
			: "fill-amber-900 ",
	);
	const sideFilter = $derived(
		bee.color === 'light'
			? "url(#light-token-side)"
			: "url(#dark-token-side)",
	);
	const iconFilter = $derived(
		bee.color === 'light'
			? "url(#light-icon-color)"
			: "url(#dark-icon-color)",
	);
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	class="cursor-pointer select-none"
	{onclick}
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
	<SpeciesIcon species={bee.species} />
</svg>
