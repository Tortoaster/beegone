<script lang="ts">
  import { Action, Board, distance, Piece, type Pos, State, x, y } from "beegone"
  import FilterContext, { getFilters } from "$lib/svg/FilterContext.svelte"
  import UseAnySymbol from "$lib/svg/symbols/UseAnySymbol.svelte"
  import AnySymbol from "$lib/svg/symbols/AnySymbol.svelte"
  import { allSymbolProps } from "$lib/svg/symbols/any-symbol"

  const VIEW_BOX = 360
  // The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
  const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3))
  const PADDING = 7.5
  const TILE_SIZE = PADDED_TILE_SIZE - PADDING
  const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2
  const TILE_RADIUS = TILE_SIZE / 2

  interface Props {
    gameState: State
    onaction: (action: Action) => void
  }

  const { gameState, onaction: _onaction }: Props = $props()

  interface PieceOnPos {
    piece: Piece
    pos: Pos
  }

  interface IdentifiedPiece {
    id: number
    pieceOnPos: PieceOnPos
  }

  let pieces: Map<number, PieceOnPos> = $state(new Map())
  let nextId = $state(0)
  $effect(() => {
    const equals =
      ({ piece: pieceA, pos: posA }: PieceOnPos) =>
      ({ piece: pieceB, pos: posB }: PieceOnPos) =>
        pieceA.toString() === pieceB.toString() && posA === posB

    let newPieces: PieceOnPos[] = Board.positions()
      .flatMap((pos) => {
        const piece = gameState.board.get(pos)
        return piece ? [{ piece, pos }] : []
      })

    let removed = [
      ...pieces
        .entries()
        .map(([id, pieceOnPos]): IdentifiedPiece => ({ id, pieceOnPos }))
        .filter(({ pieceOnPos }) => !newPieces.some(equals(pieceOnPos))),
    ]
    let added = newPieces.filter((newPiece) => !pieces.values().some(equals(newPiece)))

    if (removed.length === 2 && added.length === 1) {
      // Piece captured
      const { id: idA, pieceOnPos } = removed[0]!
      const { id: idB } = removed[1]!
      const addedPiece = added[0]!
      if (addedPiece.piece.toString() === pieceOnPos.piece.toString()) {
        pieces.set(idA, addedPiece)
      } else {
        pieces.set(idB, addedPiece)
      }
      pieces = new Map(pieces)
    } else if (removed.length === 1 && added.length === 1) {
      const removedPiece = removed[0]!
      const addedPiece = added[0]!
      if (removedPiece.pieceOnPos.pos === addedPiece.pos) {
        // Piece promoted
        pieces.delete(removedPiece.id)
        pieces.set(nextId++, addedPiece)
      } else {
        // Piece moved
        pieces.set(removedPiece.id, addedPiece)
      }
      pieces = new Map(pieces)
    } else if (removed.length > 0 || added.length > 0) {
      // Piece(s) spawned, or state changed abruptly
      removed.forEach((removedPiece) => pieces.delete(removedPiece.id))
      added.forEach((addedPiece) => pieces.set(nextId++, addedPiece))
      pieces = new Map(pieces)
    }
  })

  let selected: Pos | null = $state(null)
  const actions = $derived(selected ? gameState.actionsFrom(selected) : [])

  const onaction = $derived((action: Action) => {
    selected = null
    _onaction(action)
  })

  function canSelect(pos: Pos) {
    if (selected === pos) {
      return true
    } else {
      let piece = gameState.board.get(pos)
      if (piece?.bee && piece.bee.color === gameState.turn) {
        return true
      }
    }
    return false
  }

  function select(pos: Pos) {
    if (selected === pos) {
      selected = null
    } else {
      let piece = gameState.board.get(pos)
      if (piece?.bee && piece.bee.color === gameState.turn) {
        selected = pos
      }
    }
  }

  const actionsOnTile = (actions: Action[], pos: Pos) =>
    actions.filter((action) =>
      action.move ? action.move.to === pos : action.spawn ? action.spawn.on === pos : false
    )
</script>

<svg
  class="bg-white max-h-dvh"
  viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
  xmlns="http://www.w3.org/2000/svg"
  width="100%"
  height="100%"
>
  <FilterContext>
    {#each allSymbolProps as symbolProps}
      <AnySymbol {symbolProps} />
    {/each}
    {#each Board.positions() as pos (pos.toString())}
      {@const selectable = canSelect(pos)}
      <g
        class={[
          "transition-colors",
          pos === selected ? "fill-accent-light" : "fill-white-dark",
          selectable && "cursor-pointer",
        ]}
        filter={getFilters({ insetShadow: ["flood-shadow"] })}
      >
        <UseAnySymbol
          type="tile"
          x={PADDED_TILE_RADIUS * x(pos) - TILE_RADIUS}
          y={PADDED_TILE_RADIUS * y(pos) - TILE_RADIUS}
          width={TILE_SIZE}
          height={TILE_SIZE}
          onclick={selectable ? () => select(pos) : undefined}
        />
      </g>
    {/each}
    {#each pieces.entries() as [id, { piece, pos }] (id)}
      {@const selectable = canSelect(pos)}
      {#if piece.bee}
        <g
          style={`transform: translate(${PADDED_TILE_RADIUS * x(pos) - TILE_RADIUS}px, ${
            PADDED_TILE_RADIUS * y(pos) - TILE_RADIUS
          }px)`}
          class={["transition-transform ease-out duration-long", selectable && "cursor-pointer"]}
          onclick={selectable ? () => select(pos) : undefined}
        >
          <circle
            cx={TILE_RADIUS}
            cy={TILE_RADIUS}
            r={0.285 * TILE_SIZE}
            class={{
              "fill-primary": piece.bee.color === "light",
              "fill-black": piece.bee.color === "dark",
            }}
            filter={getFilters({
              roundShadow: [
                piece.bee.color === "light" ? "flood-primary-dark" : "flood-black-dark",
              ],
              shadow: ["flood-shadow"],
            })}
          />
          <UseAnySymbol
            type="species"
            species={piece.bee.species}
            width={0.3 * TILE_SIZE}
            height={0.3 * TILE_SIZE}
            x={0.7 * TILE_RADIUS}
            y={0.7 * TILE_RADIUS}
            class={{
              "fill-black": piece.bee.color === "light",
              "fill-white": piece.bee.color === "dark",
            }}
          />
        </g>
      {:else}
        <g
          class="fill-primary"
          filter={getFilters({ roundShadow: ["flood-primary-dark"], shadow: ["flood-shadow"] })}
        >
          <UseAnySymbol
            type="tile"
            x={PADDED_TILE_RADIUS * x(pos) - TILE_RADIUS}
            y={PADDED_TILE_RADIUS * y(pos) - TILE_RADIUS}
            width={TILE_SIZE}
            height={TILE_SIZE}
            onclick={selectable ? () => select(pos) : undefined}
          />
        </g>
      {/if}
    {/each}
    {#each Board.positions() as pos (pos.toString())}
      {@const posAngle = Math.atan2(y(pos) - y(selected ?? "D4"), x(pos) - x(selected ?? "D4"))}
      {@const maxPosDelayDuration = 300}
      {@const posDelay = (posAngle + Math.PI) / Math.PI / 2 * 0.8 * maxPosDelayDuration + distance(selected ?? pos, pos) / 6 * 0.2 * maxPosDelayDuration}
      {@const centerX = PADDED_TILE_RADIUS * x(pos)}
      {@const centerY = PADDED_TILE_RADIUS * y(pos)}
      {@const actionsHere = actionsOnTile(actions, pos)}
      {#key actionsHere}
        {#each actionsHere as action, index}
          {@const actionAngle = index * 2 * Math.PI / actionsHere.length}
          {@const actionDelay = actionAngle / 2 / Math.PI * 200}
          {@const actionRadius = (TILE_SIZE - PADDING) / 4}
          {@const deltaX = actionsHere.length === 1 ? 0 : Math.cos(actionAngle) * (TILE_RADIUS - actionRadius)}
          {@const deltaY = actionsHere.length === 1 ? 0 : Math.sin(actionAngle) * (TILE_RADIUS - actionRadius)}
          <g
            style={`transform: translate(${deltaX}px, ${deltaY}px); transition-delay: ${
              posDelay + actionDelay
            }ms;`}
            class="group initial-transform-0 transition-all ease-out duration-normal cursor-pointer"
            onclick={() => onaction(action)}
          >
            <circle
              cx={centerX}
              cy={centerY}
              r={actionRadius}
              class="transition-colors fill-accent-light group-hover:fill-accent"
              filter={getFilters({ roundShadow: ["flood-accent-dark"], shadow: ["flood-shadow"] })}
            />
            <UseAnySymbol
              type="action"
              {action}
              piece={gameState.board.get(pos)}
              class="transition-colors fill-accent-dark group-hover:fill-white"
              width={actionRadius}
              height={actionRadius}
              x={centerX - actionRadius / 2}
              y={centerY - actionRadius / 2}
            />
          </g>
        {/each}
      {/key}
    {/each}
  </FilterContext>
</svg>
