import { get, type Writable, writable } from 'svelte/store';
import {
	Color, Piece,
	State
} from 'beegone';
import type { Action, Pos } from 'beegone';

export type StateStore = {
	get: (pos: Pos) => Piece | undefined;
	turn: () => Color;
	actionsFrom: (pos: Pos) => Action[];
	perform: (action: Action) => void;
} & Omit<Writable<State>, 'set' | 'update'>;

export function createState(): StateStore {
	const store = writable(new State());

	return {
		subscribe: store.subscribe,
		get: (pos: Pos) => get(store).board.get(pos),
		turn: () => get(store).turn,
		actionsFrom: (pos: Pos) => get(store).actionsFrom(pos),
		perform: (action: Action) => store.update((store) => store.perform(action))
	};
}
