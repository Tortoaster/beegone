<script lang="ts">
  import { Action, Board, Piece, type Pos, State, x, y } from "beegone"
  import Polygon from "$lib/svg/Polygon.svelte"
  import ActionButtonGroup from "$lib/components/ActionButtonGroup.svelte"
  import BeeIcon from "$lib/icons/BeeIcon.svelte"

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
  <defs>
    <filter id="inset-shadow-filter">
      <feFlood class="flood-shadow" />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceGraphic" />
    </filter>
    <filter id="light-shadow-filter">
      <feFlood class="flood-primary-dark" />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="-1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceGraphic" result="Piece" />
      <feFlood class="flood-shadow" />
      <feComposite operator="in" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode />
        <feMergeNode in="Piece" />
      </feMerge>
    </filter>
    <filter id="dark-shadow-filter">
      <feFlood class="flood-black-dark" />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="-1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceGraphic" result="Piece" />
      <feFlood class="flood-shadow" />
      <feComposite operator="in" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode />
        <feMergeNode in="Piece" />
      </feMerge>
    </filter>
    <filter id="accent-shadow-filter">
      <feFlood class="flood-accent-dark" />
      <feComposite operator="out" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="-1" />
      <feMerge>
        <feMergeNode in="SourceGraphic" />
        <feMergeNode />
      </feMerge>
      <feComposite operator="in" in2="SourceGraphic" result="Piece" />
      <feFlood class="flood-shadow" />
      <feComposite operator="in" in2="SourceAlpha" />
      <feGaussianBlur stdDeviation="1" />
      <feOffset dy="1" />
      <feMerge>
        <feMergeNode />
        <feMergeNode in="Piece" />
      </feMerge>
    </filter>
  </defs>
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
      filter="url(#inset-shadow-filter)"
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
        filter={piece.bee.color === "light" ? "url(#light-shadow-filter)" : "url(#dark-shadow-filter)"}
      />
    {:else}
      <Polygon
        class="fill-primary"
        filter="url(#light-shadow-filter)"
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
      filter="url(#accent-shadow-filter)"
      {selected}
      {pos}
      {onaction}
    />
  {/each}
</svg>
