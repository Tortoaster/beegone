<script lang="ts">
  import type { SVGAttributes } from "svelte/elements"
  import { assertNever } from "$lib/assert-never"
  import SpeciesSymbol from "$lib/svg/symbols/SpeciesSymbol.svelte"
  import ActionSymbol from "$lib/svg/symbols/ActionSymbol.svelte"
  import Hexagon from "$lib/svg/symbols/icons/Hexagon.svelte"
  import { type AnySymbolProps, symbolId } from "$lib/svg/symbols/any-symbol"

  interface Props extends SVGAttributes<SVGSymbolElement> {
    symbolProps: AnySymbolProps
  }

  const { symbolProps, ...props }: Props = $props()
</script>

<svelte:options namespace="svg" />

{#if symbolProps.type === "species"}
  <SpeciesSymbol id={symbolId(symbolProps)} species={symbolProps.species} {...props} />
{:else if symbolProps.type === "action"}
  <ActionSymbol
    id={symbolId(symbolProps)}
    action={symbolProps.action}
    piece={symbolProps.piece}
    {...props}
  />
{:else if symbolProps.type === "tile"}
  <Hexagon id={symbolId(symbolProps)} {...props} />
{:else}
  {assertNever(symbolProps)}
{/if}
