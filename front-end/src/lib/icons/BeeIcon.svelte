<script lang="ts">
  import type { SVGAttributes } from "svelte/elements";
  import { type Bee } from "beegone";
  import SpeciesIcon from "$lib/icons/SpeciesIcon.svelte";

  interface Props extends SVGAttributes<SVGElement> {
    bee: Bee;
  }

  const { bee, ...props }: Props = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  {...props}
>
  <circle
    cx="50%"
    cy="50%"
    r="27.5%"
    class={{
      "fill-primary": bee.color === "light",
      "fill-black": bee.color === "dark",
    }}
  />
  <SpeciesIcon
    class={{
      "fill-black": bee.color === "light",
      "fill-white": bee.color === "dark",
    }}
    species={bee.species}
    x="35%"
    y="35%"
    width="30%"
    height="30%"
  />
</svg>
