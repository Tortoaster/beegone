<script lang="ts">
  import { type Bee, Color } from 'beegone';
  import SpeciesIcon from '$lib/icons/SpeciesIcon.svelte';

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
    bee.color === Color.Light
      ? "fill-amber-300"
      : "fill-amber-900 ",
  );
  const sideFilter = $derived(
    bee.color === Color.Light
      ? "url(#light-token-side)"
      : "url(#dark-token-side)",
  );
  const iconFilter = $derived(
    bee.color === Color.Light
      ? "url(#light-icon-color)"
      : "url(#dark-icon-color)",
  );
</script>

<SpeciesIcon species={bee.species}/>
