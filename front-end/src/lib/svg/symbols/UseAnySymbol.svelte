<script lang="ts">
  import type { SVGAttributes } from "svelte/elements"
  import { type AnySymbolProps, symbolId } from "$lib/svg/symbols/any-symbol"

  type Props = AnySymbolProps & SVGAttributes<SVGUseElement>

  const props: Props = $props()
</script>

<svelte:options namespace="svg" />

<use href="#{symbolId(props)}" {...props} />
