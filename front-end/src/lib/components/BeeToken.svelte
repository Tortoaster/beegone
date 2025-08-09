<script lang="ts">
  import { type Bee } from "beegone";
  import SpeciesIcon from "$lib/icons/SpeciesIcon.svelte";

  interface Props {
    bee: Bee;
    filter?: string;
    x: number;
    y: number;
    width: number;
    height: number;
    onclick?: () => void;
  }

  const { bee, filter, x, y, width, height, onclick }: Props = $props();

  const topClass = $derived(
    bee.color === "light" ? "fill-amber-300" : "fill-amber-900 ",
  );
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class="cursor-pointer select-none"
  {onclick}
  {filter}
  {width}
  {height}
  {x}
  {y}
>
  <circle cx="50%" cy="50%" r="27.5%" class={topClass} />
  <SpeciesIcon
    species={bee.species}
    x="35%"
    y="35%"
    width="30%"
    height="30%"
  />
</svg>
