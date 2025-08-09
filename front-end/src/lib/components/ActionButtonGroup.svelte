<script lang="ts">
  import { type Action, Piece, Pos } from "beegone";
  import ActionButton from "./ActionButton.svelte";
  import type { ActionButtonProps } from "$lib/components/ActionButton.svelte";

  interface Props extends
    Omit<
      ActionButtonProps,
      "action" | "x" | "y" | "dx" | "dy" | "delay"
    > {
    actions: Action[];
    cx: number;
    cy: number;
    r: number;
    pos: Pos;
    selected: Pos | null;
    piece: Piece | undefined;
  }

  const { cx, cy, r, actions, selected, pos, ...props }: Props = $props();

  const length = (pos: Pos) =>
    (Math.abs(pos.q) + Math.abs(pos.r) + Math.abs(-pos.q - pos.r)) / 2;

  const difference = $derived(
    selected === null
      ? pos
      : new Pos(selected.q - pos.q, selected.r - pos.r),
  );
  const angle = $derived(
    selected === null
      ? 0
      : Math.atan2(pos.y - selected.y, pos.x - selected.x),
  );
  const delay = $derived(
    (angle + Math.PI) * 30 + length(difference) * 2 * Math.PI * 15,
  );

  // FIXME: Duplicates code in ActionIcon.svelte, and also doesn't give compile-time errors when not handling all cases
  const actionType = (action: Action, piece: Piece | undefined) => action.move ? (piece ? (piece.bee ? 'attack' : 'dig') : 'move') : action.spawn ? (action.spawn.spawn.bee ? action.spawn.spawn.bee.species : 'build') : 'unknown'
  // Generates a unique ID for the same type of action on the same position, any other case will be rerendered
  const id = (action: Action, piece: Piece | undefined, index: number, length: number) => `${actionType(action, piece)}-${index}-${length}`;
</script>

{#each actions as action, index (id(action, props.piece, index, actions.length))}
  <ActionButton
    {action}
    x={actions.length === 1 ? cx : cx + r * Math.cos(index * 2 * Math.PI / actions.length)}
    y={actions.length === 1 ? cy : cy + r * Math.sin(index * 2 * Math.PI / actions.length)}
    dx={actions.length === 1 ? 0 : -r * Math.cos(index * 2 * Math.PI / actions.length)}
    dy={actions.length === 1 ? 0 : -r * Math.sin(index * 2 * Math.PI / actions.length)}
    {delay}
    {...props}
  />
{/each}
