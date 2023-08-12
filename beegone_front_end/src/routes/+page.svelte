<script lang="ts">
	import PieceComponent from '../components/PieceComponent.svelte';
	import Tile from '../components/Tile.svelte';
	import { state } from '../stores/state';
	import type { Action, Pos } from '../../../beegone_wasm/beegone_types';

	let selected: Pos | null = null;

	$: moves = state
		.actionsFrom(selected ?? { q: -10, r: -10 })
		.flatMap((action) => (action.type === 'move' ? [action.content.to] : []));

	function select(pos: Pos) {
		if (selected == null) {
			let piece = state.get(pos);
			if (piece?.type === 'bee' && piece.content.color === state.turn()) {
				selected = pos;
			}
		} else if (selected.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			const destination = state.get(pos);
			if (destination == null) {
				// Move
				let move: Action = { type: 'move', content: { from: selected, to: pos } };
				state.perform(move);
				selected = null;
			} else {
				switch (destination.type) {
					case 'bee':
						if (destination.content.color !== state.turn()) {
							// Capture
							let move: Action = { type: 'move', content: { from: selected, to: pos } };
							state.perform(move);
							selected = null;
						} else {
							selected = pos;
						}
						break;
					case 'wall': {
						// Break wall
						let move: Action = { type: 'move', content: { from: selected, to: pos } };
						state.perform(move);
						selected = null;
						break;
					}
				}
			}
		}
	}
</script>

<svg viewBox="0 0 300 300" xmlns="http://www.w3.org/2000/svg" width="100vw" height="100vh">
	{#each state.positions() as pos}
		<Tile
			on:click={() => select(pos)}
			{pos}
			scale={50}
			selected={selected?.q === pos.q && selected.r === pos.r}
			highlight={moves.find((p) => p.q === pos.q && p.r === pos.r) !== undefined}
		>
			{#if state.get(pos) != null}
				<PieceComponent on:click={() => select(pos)} piece={state.get(pos)} />
			{/if}
		</Tile>
	{/each}
</svg>
