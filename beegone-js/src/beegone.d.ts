/* tslint:disable */
/* eslint-disable */
export function stateNew(players: number): State;
export function stateActionsFrom(state: State, pos: Pos): Action[];
export function submitAction(action: Action): Promise<void>;
export function stateProgress(state: State): Promise<State>;
export function boardGet(board: Board, pos: Pos): Piece | undefined;
export function boardPositions(): Pos[];
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
}
export class ActionError {
  private constructor();
  free(): void;
}
export class Bee {
  private constructor();
  free(): void;
}
export class Board {
  private constructor();
  free(): void;
}
export class MoveAction {
  private constructor();
  free(): void;
}
export class Piece {
  private constructor();
  free(): void;
}
export class Pos {
  private constructor();
  free(): void;
  q: number;
  r: number;
}
export class SpawnAction {
  private constructor();
  free(): void;
}
export class State {
  private constructor();
  free(): void;
}
