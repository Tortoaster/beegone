<script lang="ts">
  import type { Bee } from 'beegone/types';

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
    bee.color === "light"
      ? "transition-colors duration-300 fill-amber-300 dark:fill-slate-100"
      : "transition-colors duration-300 fill-amber-900  dark:fill-slate-800",
  );
  const sideFilter = $derived(
    bee.color === "light"
      ? "url(#light-token-side)"
      : "url(#dark-token-side)",
  );
  const iconFilter = $derived(
    bee.color === "light"
      ? "url(#light-icon-color)"
      : "url(#dark-icon-color)",
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
  <defs>
    <filter id="light-token-side" y="-25%" height="125%">
      <feFlood
        class="transition-all duration-300 flood-amber-400 dark:flood-slate-300"
        result="Color"
      />
      <feComposite
        operator="in"
        in="Color"
        in2="SourceGraphic"
        result="Shadow"
      />
      <feOffset in="Shadow" dy="-3" result="Shadow3" />
      <feOffset in="SourceGraphic" dy="-6" result="Offset" />
      <feMerge>
        <feMergeNode in="Shadow" />
        <feMergeNode in="Shadow3" />
        <feMergeNode in="Offset" />
      </feMerge>
    </filter>
    <filter id="dark-token-side" y="-25%" height="125%">
      <feFlood
        class="transition-all duration-300 flood-amber-950 dark:flood-slate-900"
        result="Color"
      />
      <feComposite
        operator="in"
        in="Color"
        in2="SourceGraphic"
        result="Shadow"
      />
      <feOffset in="Shadow" dy="-3" result="Shadow3" />
      <feOffset in="SourceGraphic" dy="-6" result="Offset" />
      <feMerge>
        <feMergeNode in="Shadow" />
        <feMergeNode in="Shadow3" />
        <feMergeNode in="Offset" />
      </feMerge>
    </filter>
    <filter id="light-icon-color" y="-40%" height="140%">
      <feFlood
        class="transition-all duration-300 flood-amber-500 dark:flood-slate-400"
      />
      <feComposite operator="in" in2="SourceGraphic" />
      <feOffset dy="-6" />
    </filter>
    <filter id="dark-icon-color" y="-40%" height="140%">
      <feFlood
        class="transition-all duration-300 flood-amber-700 dark:flood-slate-600"
      />
      <feComposite operator="in" in2="SourceGraphic" />
      <feOffset dy="-6" />
    </filter>
  </defs>
  <circle cx="50%" cy="50%" r="27.5%" class={topClass} filter={sideFilter} />
  <image
    xlink:href="/{bee.species}.svg"
    x="35%"
    y="35%"
    width="30%"
    height="30%"
    filter={iconFilter}
  />
</svg>
