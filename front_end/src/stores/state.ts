import { get, writable } from 'svelte/store';
import {
	boardGet,
	boardPositions,
	stateActionsFrom,
	stateNew,
	stateProgress,
	submitAction,
} from '../beegone/beegone';
import type { Action, Pos } from '../beegone/types';

export function createState(players: number) {
	const store = writable({
		state: stateNew(players),
		positions: () => boardPositions(),
	});

	return {
		subscribe: store.subscribe,
		progress: async () => {
			const state = await stateProgress(get(store).state);
			store.update(({ positions }) => ({
				state,
				positions,
			}));
		},
		get: (pos: Pos) => boardGet(get(store).state.board, pos),
		turn: () => get(store).state.turn,
		actionsFrom: (pos: Pos) => stateActionsFrom(get(store).state, pos),
		perform: async (action: Action) => await submitAction(action),
	};
}
