<script lang="ts">
  import { Action, Piece } from "beegone";
  import type { SVGAttributes } from "svelte/elements";
  import Move from "$lib/icons/Move.svelte";
  import Attack from "$lib/icons/Attack.svelte";
  import Dig from "$lib/icons/Dig.svelte";
  import SpeciesIcon from "$lib/icons/SpeciesIcon.svelte";
  import Build from "$lib/icons/Build.svelte";

  interface Props extends SVGAttributes<SVGElement> {
    action: Action;
    /**
     * The piece standing on the destination tile of the action.
     */
    piece: Piece | undefined;
  }

  const { action, piece, ...props }: Props = $props();
</script>

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
    <SpeciesIcon species={action.spawn.spawn.bee.species} {...props} />
  {:else}
    <Build {...props} />
  {/if}
{/if}
