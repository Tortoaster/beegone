/* tslint:disable */
/* eslint-disable */
export function stateNew(players: number): State;
export function stateActionsFrom(state: State, pos: Pos): Action[];
export function submitAction(action: Action): Promise<void>;
export function stateProgress(state: State): Promise<State>;
export function boardGet(board: Board, pos: Pos): WithId<Piece> | undefined;
export function boardPositions(): Pos[];
import { Action, Board, Piece, Pos, State, WithId } from './types';

export { Action, Board, Piece, Pos, State, WithId };

