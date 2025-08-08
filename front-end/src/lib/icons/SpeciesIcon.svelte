<script lang="ts">
	import { Species } from 'beegone';
	import Drone from '$lib/icons/Drone.svelte';
	import Worker from '$lib/icons/Worker.svelte';
	import Nurse from '$lib/icons/Nurse.svelte';
	import Explorer from '$lib/icons/Explorer.svelte';
	import Builder from '$lib/icons/Builder.svelte';
	import Guard from '$lib/icons/Guard.svelte';
	import Queen from '$lib/icons/Queen.svelte';
	import { assertNever } from '$lib/assert-never';

	interface Props {
		species: Species;
	}

	const { species }: Props = $props();
</script>

{#if species === Species.Drone}
	<Drone />
{:else if species === Species.Worker}
	<Worker />
{:else if species === Species.Nurse}
	<Nurse />
{:else if species === Species.Explorer}
	<Explorer />
{:else if species === Species.Builder}
	<Builder />
{:else if species === Species.Guard}
	<Guard />
{:else if species === Species.Queen}
	<Queen />
{:else}
	{assertNever(species)}
{/if}
