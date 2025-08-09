<script lang="ts">
  import { Action, Board, type Pos, State } from "beegone";
  import ActionButtonGroup from "$lib/components/ActionButtonGroup.svelte";
  import BeeToken from "$lib/components/BeeToken.svelte";
  import Polygon from "$lib/svg/Polygon.svelte";
  import Attack from "$lib/icons/Attack.svelte";

  const VIEW_BOX = 360;
  // The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
  const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3));
  const PADDING = 5;
  const TILE_SIZE = PADDED_TILE_SIZE - PADDING;
  const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2;
  const TILE_RADIUS = TILE_SIZE / 2;

  let gameState: State = $state(new State());
  let selected: Pos | null = $state(null);

  const actions = $derived(selected ? gameState.actionsFrom(selected) : []);

  function select(pos: Pos) {
    if (selected?.q === pos.q && selected.r === pos.r) {
      selected = null;
    } else {
      let piece = gameState.board.get(pos);
      if (piece?.bee && piece.bee.color === gameState.turn) {
        selected = pos;
      }
    }
  }

  async function performAction(action: Action) {
    gameState = gameState.perform(action);
    selected = null;
  }
</script>

<div
  class="bg-amber-500 p-4 size-full absolute"
>
  <svg
    viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
    xmlns="http://www.w3.org/2000/svg"
    width="100%"
    height="100%"
  >
    {#each Board.positions() as pos}
      {@const piece = gameState.board.get(pos)}
      {#if piece}
        {#if piece.bee}
          <Polygon
            class="fill-amber-700"
            cx={PADDED_TILE_RADIUS * pos.x}
            cy={PADDED_TILE_RADIUS * pos.y}
            r={TILE_RADIUS}
            sides={6}
            cornerRadius={8}
          />
          <BeeToken
            bee={piece.bee}
            width={TILE_SIZE}
            height={TILE_SIZE}
            x={PADDED_TILE_RADIUS * pos.x - TILE_RADIUS}
            y={PADDED_TILE_RADIUS * pos.y - TILE_RADIUS}
            onclick={() => select(pos)}
          />
        {:else}
          <Polygon
            class="fill-amber-400"
            cx={PADDED_TILE_RADIUS * pos.x}
            cy={PADDED_TILE_RADIUS * pos.y}
            r={TILE_RADIUS}
            sides={6}
            cornerRadius={8}
          />
        {/if}
      {:else}
        <Polygon
          class="fill-amber-700"
          cx={PADDED_TILE_RADIUS * pos.x}
          cy={PADDED_TILE_RADIUS * pos.y}
          r={TILE_RADIUS}
          sides={6}
          cornerRadius={8}
        />
      {/if}
      <ActionButtonGroup
        cx={PADDED_TILE_RADIUS * pos.x}
        cy={PADDED_TILE_RADIUS * pos.y}
        r={PADDED_TILE_RADIUS / 2}
        size={0.4 * TILE_SIZE}
        actions={actions.filter((action) =>
          action.move
            ? action.move.to.q === pos.q && action.move.to.r === pos.r
            : action.spawn
            ? action.spawn.on.q === pos.q && action.spawn.on.r === pos.r
            : {}
        )}
        piece={gameState.board.get(pos)}
        {selected}
        {pos}
        onaction={performAction}
      />
    {/each}
  </svg>
</div>
