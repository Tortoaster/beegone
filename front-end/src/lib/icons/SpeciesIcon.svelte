<script lang="ts">
	import Drone from '$lib/icons/Drone.svelte';
	import Worker from '$lib/icons/Worker.svelte';
	import Nurse from '$lib/icons/Nurse.svelte';
	import Explorer from '$lib/icons/Explorer.svelte';
	import Builder from '$lib/icons/Builder.svelte';
	import Guard from '$lib/icons/Guard.svelte';
	import Queen from '$lib/icons/Queen.svelte';
	import { assertNever } from '$lib/assert-never';
	import type { Species } from 'beegone';

	interface Props {
		species: Species;
	}

	const { species }: Props = $props();
</script>

{#if species === 'drone'}
	<Drone />
{:else if species === 'worker'}
	<Worker />
{:else if species === 'nurse'}
	<Nurse />
{:else if species === 'explorer'}
	<Explorer />
{:else if species === 'builder'}
	<Builder />
{:else if species === 'guard'}
	<Guard />
{:else if species === 'queen'}
	<Queen />
{:else}
	{assertNever(species)}
{/if}
