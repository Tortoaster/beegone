<script lang="ts">
  import { Action, Piece } from "beegone"
  import type { SVGAttributes } from "svelte/elements"
  import Move from "$lib/svg/symbols/icons/action/Move.svelte"
  import Attack from "$lib/svg/symbols/icons/action/Attack.svelte"
  import Dig from "$lib/svg/symbols/icons/action/Dig.svelte"
  import SpeciesSymbol from "$lib/svg/symbols/SpeciesSymbol.svelte"
  import Build from "$lib/svg/symbols/icons/action/Build.svelte"

  interface Props extends SVGAttributes<SVGSymbolElement> {
    action: Action
    /**
     * The piece standing on the destination tile of the action.
     */
    piece: Piece | undefined
  }

  const { action, piece, ...props }: Props = $props()
</script>

<svelte:options namespace="svg" />

{#if action.move}
  {#if piece}
    {#if piece.bee}
      <Attack {...props} />
    {:else}
      <Dig {...props} />
    {/if}
  {:else}
    <Move {...props} />
  {/if}
{:else if action.spawn}
  {#if action.spawn.spawn.bee}
    <SpeciesSymbol species={action.spawn.spawn.bee.species} {...props} />
  {:else}
    <Build {...props} />
  {/if}
{/if}
