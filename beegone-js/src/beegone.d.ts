/* tslint:disable */
/* eslint-disable */
export enum InvalidBee {
  InvalidColor = 0,
  InvalidSpecies = 1,
}
export enum PerformError {
  InvalidColor = 0,
  InvalidSpecies = 1,
  InvalidAction = 2,
}
type Color = "light" | "dark";
type Species = "drone" | "worker" | "nurse" | "builder" | "explorer" | "queen" | "guard";
export class Action {
  private constructor();
  free(): void;
  static move(from: Pos, to: Pos): Action;
  static spawn(on: Pos, spawn: Piece): Action;
  readonly move: WasmMoveAction | undefined;
  readonly spawn: WasmSpawnAction | undefined;
}
export class Bee {
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
export class InvalidColor {
  private constructor();
  free(): void;
}
export class InvalidSpecies {
  private constructor();
  free(): void;
}
export class Piece {
  private constructor();
  free(): void;
  /**
   * If `None`, the piece is a wall.
   */
  readonly bee: Bee | undefined;
}
export class Pos {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor(q: number, r: number);
  readonly q: number;
  readonly r: number;
  readonly s: number;
  readonly x: number;
  readonly y: number;
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
