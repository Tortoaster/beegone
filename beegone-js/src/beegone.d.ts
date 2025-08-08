/* tslint:disable */
/* eslint-disable */
export enum Color {
  Light = 0,
  Dark = 1,
}
export enum Species {
  Drone = 0,
  Worker = 1,
  Nurse = 2,
  Builder = 3,
  Explorer = 4,
  Guard = 5,
  Queen = 6,
}
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
export class InvalidAction {
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
