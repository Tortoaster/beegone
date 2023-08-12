import { get, writable } from 'svelte/store';
import { boardGet, stateActionsFrom, stateNew } from '../../../beegone_wasm/beegone';
import type { Action, Pos } from '../../../beegone_wasm/beegone_types';

function createState() {
	const store = writable(stateNew());

	return {
		subscribe: store.subscribe,
		get: (pos: Pos) => boardGet(get(store).board, pos),
		actionsFrom: (pos: Pos) => stateActionsFrom(get(store), pos),
		turn: () => get(store).turn,
		perform: (action: Action) => {},
	};
}

export const state = createState();
