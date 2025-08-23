<script lang="ts">
  import { setContext } from "svelte"

  const { children } = $props()

  $effect(() => {
    setContext(contextKey, true)
  })
</script>

<script lang="ts" module>
  const contextKey = "filters"

  const allShadowColors = [
    "flood-shadow",
  ] as const

  const allInsetShadowColors = [
    "flood-shadow",
  ] as const

  const allRoundShadowColors = [
    "flood-primary-dark",
    "flood-accent-dark",
    "flood-black-dark",
  ] as const

  type FilterType = "shadow" | "insetShadow" | "roundShadow"

  export type ShadowColor = typeof allShadowColors[number]
  export type InsetShadowColor = typeof allInsetShadowColors[number]
  export type RoundShadowColor = typeof allRoundShadowColors[number]

  // FIXME: Enforce that all keys are FilterType
  export interface Filters {
    shadow?: ShadowColor[]
    insetShadow?: InsetShadowColor[]
    roundShadow?: RoundShadowColor[]
  }

  const colorToId = (
    color: ShadowColor | RoundShadowColor | InsetShadowColor,
    type: FilterType,
  ): string => `${type}${color.substring(6)}Filter`
  const idToFilter = (id: string): string => `url(#${id})`

  export function getFilters(
    { shadow = [], insetShadow = [], roundShadow = [] }: Filters,
  ): string {
    return insetShadow.map((color) => idToFilter(colorToId(color, "insetShadow")))
      .concat(roundShadow.map((color) => idToFilter(colorToId(color, "roundShadow"))))
      // Shadow filters change the shape of the source graphic, so they are applied last so the filters above aren't applied to the shadow part
      .concat(shadow.map((color) => idToFilter(colorToId(color, "shadow"))))
      .join(" ")
  }
</script>

<svelte:options namespace="svg" />

<defs>
  {#each allShadowColors as classValue}
    <filter id={colorToId(classValue, "shadow")}>
      <feFlood class={classValue} />
      <feComposite operator="in" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode />
        <feMergeNode in="SourceGraphic" />
      </feMerge>
    </filter>
  {/each}
  {#each allInsetShadowColors as classValue}
    <filter id={colorToId(classValue, "insetShadow")}>
      <feFlood class={classValue} />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceAlpha" />
    </filter>
  {/each}
  {#each allRoundShadowColors as classValue}
    <filter id={colorToId(classValue, "roundShadow")}>
      <feFlood class={classValue} />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="-1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceAlpha" />
    </filter>
  {/each}
</defs>

{@render children?.()}
