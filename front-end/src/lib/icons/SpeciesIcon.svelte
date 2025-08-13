<script lang="ts">
  import Drone from "$lib/icons/Drone.svelte"
  import Worker from "$lib/icons/Worker.svelte"
  import Nurse from "$lib/icons/Nurse.svelte"
  import Explorer from "$lib/icons/Explorer.svelte"
  import Builder from "$lib/icons/Builder.svelte"
  import Guard from "$lib/icons/Guard.svelte"
  import Queen from "$lib/icons/Queen.svelte"
  import { assertNever } from "$lib/assert-never"
  import type { Species } from "beegone"
  import type { SVGAttributes } from "svelte/elements"

  interface Props extends SVGAttributes<SVGElement> {
    species: Species
  }

  const { species, ...props }: Props = $props()
</script>

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
