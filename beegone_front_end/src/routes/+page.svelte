<script lang="ts">
	import PieceComponent from '../components/PieceComponent.svelte';
	import Tile from '../components/Tile.svelte';
	import { state } from '../stores/state';
	import type { Action, Pos } from '../../../beegone_wasm/beegone_types';
	import ActionButtonGroup from '../components/ActionButtonGroup.svelte';
	import LightSwitch from '../components/LightSwitch.svelte';

	let selected: Pos | null = null;

	$: actions = state.actionsFrom(selected ?? { q: 999, r: 999 });
	/**
	 * All actions belonging to a specific tile
	 */
	$: actionsOn = (pos: Pos) =>
		actions.filter((action) =>
			action.type === 'move'
				? action.content.to.q === pos.q && action.content.to.r === pos.r
				: action.content.on.q === pos.q && action.content.on.r === pos.r
		);

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

	const viewBox = 360;
	const tileSize = viewBox / 6;
	const x = (pos: Pos) => (tileSize / 2) * ((3 / 2) * pos.q - 1);
	const y = (pos: Pos) => (tileSize / 2) * ((Math.sqrt(3) / 2) * pos.q + Math.sqrt(3) * pos.r - 1);
</script>

<div>
	<div class="fixed top-0 right-0 m-4">
		<LightSwitch />
	</div>
	<svg
		class="bg-amber-800 dark:bg-slate-800"
		viewBox="{-viewBox / 2} {-viewBox / 2} {viewBox} {viewBox}"
		xmlns="http://www.w3.org/2000/svg"
		width="100vw"
		height="100vh"
	>
		{#each state.positions() as pos}
			<svg x={x(pos) * 0.9} y={y(pos) * 0.9} width={tileSize} height={tileSize}>
				<Tile
					height={-0.05}
					sideClass="fill-amber-600 dark:fill-slate-500"
					topClass="fill-amber-300 dark:fill-slate-400"
				>
					{#if state.get(pos) != null}
						<PieceComponent piece={state.get(pos)} />
					{/if}
					<ActionButtonGroup actions={actionsOn(pos)} />
				</Tile>
			</svg>
		{/each}
	</svg>
</div>
