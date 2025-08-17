<script lang="ts">
  import { Action, Board, Piece, type Pos, State, x, y } from "beegone"
  import Polygon from "$lib/svg/Polygon.svelte"
  import ActionButtonGroup from "$lib/components/ActionButtonGroup.svelte"
  import BeeIcon from "$lib/icons/BeeIcon.svelte"
  import FilterContext, { getFilters } from '$lib/svg/FilterContext.svelte';

  const VIEW_BOX = 360
  // The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
  const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3))
  const PADDING = 5
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
    {#each Board.positions() as pos (pos.toString())}
      {@const selectable = canSelect(pos)}
      <Polygon
        cx={PADDED_TILE_RADIUS * x(pos)}
        cy={PADDED_TILE_RADIUS * y(pos)}
        r={TILE_RADIUS}
        sides={6}
        cornerRadius={8}
        onclick={selectable ? () => select(pos) : undefined}
        class={[
          "transition-colors",
          pos === selected ? "fill-accent-light" : "fill-white-dark",
          selectable && "cursor-pointer",
        ]}
        filter={getFilters({ insetShadow: ['flood-shadow'] })}
      />
    {/each}
    {#each pieces.entries() as [id, { piece, pos }] (id)}
      {@const selectable = canSelect(pos)}
      {#if piece.bee}
        <BeeIcon
          bee={piece.bee}
          width={TILE_SIZE}
          height={TILE_SIZE}
          onclick={selectable ? () => select(pos) : undefined}
          style={`transform: translate(${PADDED_TILE_RADIUS * x(pos) - TILE_RADIUS}px, ${
            PADDED_TILE_RADIUS * y(pos) - TILE_RADIUS
          }px)`}
          class={[
            "transition-transform ease-out duration-long",
            selectable && "cursor-pointer",
          ]}
          filter={getFilters({ roundShadow: [piece.bee.color === "light" ? 'flood-primary-dark' : 'flood-black-dark'], shadow: ['flood-shadow'] })}
        />
      {:else}
        <Polygon
          class="fill-primary"
          filter={getFilters({ roundShadow: ['flood-primary-dark'], shadow: ['flood-shadow'] })}
          cx={PADDED_TILE_RADIUS * x(pos)}
          cy={PADDED_TILE_RADIUS * y(pos)}
          r={TILE_RADIUS}
          sides={6}
          cornerRadius={8}
        />
      {/if}
    {/each}
    {#each Board.positions() as pos (pos.toString())}
      <ActionButtonGroup
        cx={PADDED_TILE_RADIUS * x(pos)}
        cy={PADDED_TILE_RADIUS * y(pos)}
        r={PADDED_TILE_RADIUS / 2}
        size={0.4 * TILE_SIZE}
        actions={actionsOnTile(actions, pos)}
        piece={gameState.board.get(pos)}
        filter={getFilters({ roundShadow: ['flood-accent-dark'], shadow: ['flood-shadow'] })}
        {selected}
        {pos}
        {onaction}
      />
    {/each}
  </FilterContext>
</svg>
