/* tslint:disable */
/* eslint-disable */
export function x(pos: Pos): number;
export function y(pos: Pos): number;
export function distance(from: Pos, to: Pos): number;
export enum BeegoneError {
  InvalidColor = 0,
  InvalidSpecies = 1,
  InvalidAction = 2,
  InvalidPos = 3,
}
type Color = "light" | "dark";
type Pos = "A4" | "A5" | "A6" | "A7" | "B3" | "B4" | "B5" | "B6" | "B7" | "C2" | "C3" | "C4" | "C5" | "C6" | "C7" | "D1" | "D2" | "D3" | "D4" | "D5" | "D6" | "D7" | "E1" | "E2" | "E3" | "E4" | "E5" | "E6" | "F1" | "F2" | "F3" | "F4" | "F5" | "G1" | "G2" | "G3" | "G4";
type Species = "drone" | "worker" | "nurse" | "explorer" | "builder" | "queen" | "guard";
export class Action {
  private constructor();
  free(): void;
  static move(from: Pos, to: Pos): Action;
  static spawn(on: Pos, spawn: Piece): Action;
  readonly move: WasmMoveAction | undefined;
  readonly spawn: WasmSpawnAction | undefined;
}
export class Bee {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor(color: Color, species: Species);
  readonly color: Color;
  readonly species: Species;
}
export class Board {
  private constructor();
  free(): void;
  get(pos: Pos): Piece | undefined;
  static positions(): Pos[];
}
export class Piece {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  /**
   * If `None`, the piece is a wall.
   */
  readonly bee: Bee | undefined;
}
export class State {
  free(): void;
  constructor();
  actionsFrom(pos: Pos): Action[];
  perform(action: Action): State;
  readonly board: Board;
  readonly turn: Color;
}
export class WasmMoveAction {
  private constructor();
  free(): void;
  readonly from: Pos;
  readonly to: Pos;
}
export class WasmSpawnAction {
  private constructor();
  free(): void;
  readonly on: Pos;
  readonly spawn: Piece;
}
