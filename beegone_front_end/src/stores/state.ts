import { get, writable } from 'svelte/store';
import {
	boardGet,
	boardPositions,
	stateActionsFrom,
	stateNew,
	statePerform,
} from '../../../beegone_wasm/beegone';
import type { Action, Pos } from '../../../beegone_wasm/beegone_types';

function createState() {
	const store = writable({
		state: stateNew(),
		positions: () => boardPositions(),
	});

	return {
		subscribe: store.subscribe,
		get: (pos: Pos) => boardGet(get(store).state.board, pos),
		turn: () => get(store).state.turn,
		actionsFrom: (pos: Pos) => stateActionsFrom(get(store).state, pos),
		perform: (action: Action) =>
			store.update(({ state, positions }) => ({
				state: statePerform(state, action),
				positions,
			})),
	};
}

export const state = createState();
