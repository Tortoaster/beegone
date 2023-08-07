<script lang="ts">
	import PieceComponent from '../components/PieceComponent.svelte';
	import Tile from '../components/Tile.svelte';
	import { Pos, State } from '../../../beegone_wasm/beegone';

	const BOARD_RADIUS = 3;
	const TILE_SIZE = 50;

	let state: State = new State();

	let selected: Pos | null = null;
	const endTurn = () => {
		selected = null;
		state.end_turn();
	};

	function select(pos: Pos) {
		if (selected == null) {
			let piece = state.board.get(pos);
			if (piece?.tag === 'Bee' && piece.content[0] === state.turn) {
				selected = pos;
			}
		} else if (selected.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			const destination = state.board.get(pos);
			if (destination == null) {
				// Move
				let piece = state.board.get(selected)!;
				state.board.set(selected, null);
				state.board.set(pos, piece);
				endTurn();
			} else {
				switch (destination.type) {
					case 'Bee':
						if (destination.content[0] != state.turn) {
							// Capture
							let piece = state.board.get(selected)!;
							state.board.set(selected, null);
							state.board.set(pos, piece);
							endTurn();
						} else {
							selected = pos;
						}
						break;
					case 'Wall': {
						// Break wall
						let piece = state.board.get(selected)!;
						state.board.set(selected, null);
						state.board.set(pos, piece);
						endTurn();
						break;
					}
				}
			}
		}
	}

	const range = (start: number, stop: number, step = 1) =>
		Array.from({ length: (stop - start) / step + 1 }, (_, i) => start + i * step);
	const positions = range(-BOARD_RADIUS, BOARD_RADIUS)
		.flatMap((q) => range(-BOARD_RADIUS, BOARD_RADIUS).map((r) => new Pos(q, r)))
		.filter((pos) => (Math.abs(pos.q) + Math.abs(pos.r) + Math.abs(pos.s())) / 2 <= BOARD_RADIUS);
</script>

<svg viewBox="0 0 300 300" xmlns="http://www.w3.org/2000/svg" width="100vw" height="100vh">
	{#each positions as pos}
		<Tile
			on:click={() => select(pos)}
			{pos}
			scale={TILE_SIZE}
			selected={selected?.q === pos.q && selected.r === pos.r}
		>
			{#if state.board.get(pos) != null}
				<PieceComponent on:click={() => select(pos)} piece={state.board.get(pos)} />
			{/if}
		</Tile>
	{/each}
</svg>
