<script lang="ts">
  import type { SVGAttributes } from "svelte/elements"
  import Drone from "$lib/svg/symbols/icons/species/Drone.svelte"
  import Worker from "$lib/svg/symbols/icons/species/Worker.svelte"
  import Nurse from "$lib/svg/symbols/icons/species/Nurse.svelte"
  import Explorer from "$lib/svg/symbols/icons/species/Explorer.svelte"
  import Builder from "$lib/svg/symbols/icons/species/Builder.svelte"
  import Guard from "$lib/svg/symbols/icons/species/Guard.svelte"
  import Queen from "$lib/svg/symbols/icons/species/Queen.svelte"
  import { assertNever } from "$lib/assert-never"
  import type { Species } from "beegone"

  interface Props extends SVGAttributes<SVGSymbolElement> {
    species: Species
  }

  const { species, ...props }: Props = $props()
</script>

<svelte:options namespace="svg" />

{#if species === "drone"}
  <Drone {...props} />
{:else if species === "worker"}
  <Worker {...props} />
{:else if species === "nurse"}
  <Nurse {...props} />
{:else if species === "explorer"}
  <Explorer {...props} />
{:else if species === "builder"}
  <Builder {...props} />
{:else if species === "guard"}
  <Guard {...props} />
{:else if species === "queen"}
  <Queen {...props} />
{:else}
  {assertNever(species)}
{/if}
