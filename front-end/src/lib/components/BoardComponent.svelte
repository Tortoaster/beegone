<script lang="ts">
  import { Action, Board, Piece, type Pos, State } from "beegone";
  import Polygon from "$lib/svg/Polygon.svelte";
  import ActionButtonGroup from "$lib/components/ActionButtonGroup.svelte";
  import BeeIcon from "$lib/icons/BeeIcon.svelte";

  const VIEW_BOX = 360;
  // The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
  const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3));
  const PADDING = 5;
  const TILE_SIZE = PADDED_TILE_SIZE - PADDING;
  const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2;
  const TILE_RADIUS = TILE_SIZE / 2;

  interface Props {
    gameState: State;
    onaction: (action: Action) => void;
  }

  const { gameState, onaction: _onaction }: Props = $props();

  let selected: Pos | null = $state(null);
  const actions = $derived(selected ? gameState.actionsFrom(selected) : []);

  const pieces = $derived(
    Board.positions()
      .map((pos) => [pos, gameState.board.get(pos)])
      .filter(([, piece]) => piece) as [Pos, Piece][],
  );

  const onaction = $derived((action: Action) => {
    selected = null;
    _onaction(action);
  });

  function canSelect(pos: Pos) {
    if (selected?.q === pos.q && selected.r === pos.r) {
      return true;
    } else {
      let piece = gameState.board.get(pos);
      if (piece?.bee && piece.bee.color === gameState.turn) {
        return true;
      }
    }
    return false;
  }

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

  const actionsOnTile = (actions: Action[], pos: Pos) =>
    actions.filter((action) =>
      action.move
        ? action.move.to.q === pos.q && action.move.to.r === pos.r
        : action.spawn
        ? action.spawn.on.q === pos.q && action.spawn.on.r === pos.r
        : false
    );

	const deriveId = (pos: Pos, piece: Piece) => piece.bee
		? piece.bee.species === 'nurse'
			? `${piece.bee.color}-nurse-${pos.toString()}`
			: `${piece.bee.color}-${piece.bee.species}`
		: `wall-${pos.toString()}`;
</script>

<svg
  class="bg-white"
  viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
  xmlns="http://www.w3.org/2000/svg"
  width="100%"
  height="100%"
>
  {#each Board.positions() as pos (pos.toString())}
    {@const selectable = canSelect(pos)}
    <Polygon
      cx={PADDED_TILE_RADIUS * pos.x}
      cy={PADDED_TILE_RADIUS * pos.y}
      r={TILE_RADIUS}
      sides={6}
      cornerRadius={8}
      onclick={selectable ? () => select(pos) : undefined}
      class={[
        "transition-colors",
        pos.q === selected?.q && pos.r === selected.r
          ? "fill-accent"
          : "fill-gray",
        selectable && "cursor-pointer",
      ]}
    />
  {/each}
  {#each pieces as [pos, piece] (deriveId(pos, piece))}
		{@const selectable = canSelect(pos)}
		{#if piece.bee}
			<BeeIcon
				bee={piece.bee}
				width={TILE_SIZE}
				height={TILE_SIZE}
				onclick={selectable ? () => select(pos) : undefined}
				style={`transform: translate(${PADDED_TILE_RADIUS * pos.x - TILE_RADIUS}px, ${PADDED_TILE_RADIUS * pos.y - TILE_RADIUS}px)`}
				class={["transition-transform ease-out duration-long", selectable && "cursor-pointer"]}
			/>
		{:else}
			<Polygon
				class="fill-secondary"
				cx={PADDED_TILE_RADIUS * pos.x}
				cy={PADDED_TILE_RADIUS * pos.y}
				r={TILE_RADIUS}
				sides={6}
				cornerRadius={8}
			/>
		{/if}
  {/each}
	{#each Board.positions() as pos (pos.toString())}
		<ActionButtonGroup
			cx={PADDED_TILE_RADIUS * pos.x}
			cy={PADDED_TILE_RADIUS * pos.y}
			r={PADDED_TILE_RADIUS / 2}
			size={0.4 * TILE_SIZE}
			actions={actionsOnTile(actions, pos)}
			piece={gameState.board.get(pos)}
			{selected}
			{pos}
			{onaction}
		/>
	{/each}
</svg>
