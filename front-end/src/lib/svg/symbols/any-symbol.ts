import { Action, Bee, Piece, type Species } from "beegone"
import { assertNever } from "$lib/assert-never.ts"

export type AnySymbolProps =
  | { type: "species"; species: Species }
  | { type: "action"; action: Action; piece: Piece | undefined }
  | { type: "tile" }

export const symbolId = (props: AnySymbolProps) =>
  props.type === "species"
    ? `${props.species}-symbol`
    : props.type === "action"
    ? (props.action.spawn
      ? (props.action.spawn.spawn.bee
        ? `${props.action.spawn.spawn.bee.species}-symbol`
        : `build-symbol`)
      : props.action.move
      ? (props.piece ? (props.piece.bee ? `attack-symbol` : `dig-symbol`) : `move-symbol`)
      : `unknown-action-symbol`)
    : props.type === "tile"
    ? "tile-symbol"
    : assertNever(props.type)

const species: Species[] = [
  "drone",
  "worker",
  "nurse",
  "explorer",
  "builder",
  "guard",
  "queen",
]

const moveTargets = [
  Piece.bee(new Bee("light", "drone")),
  Piece.wall(),
  undefined,
]

export const allSymbolProps: AnySymbolProps[] = species.map((species) => ({
  type: "species",
  species,
}))
  .concat({ type: "action", action: Action.spawn("A4", Piece.wall()), piece: undefined })
  .concat(moveTargets.map((piece) => ({ type: "action", action: Action.move("A4", "A5"), piece })))
  .concat({ type: "tile" })
