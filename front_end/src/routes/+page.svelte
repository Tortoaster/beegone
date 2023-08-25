<script lang="ts">
	import PieceComponent from '../components/PieceComponent.svelte';
	import Tile from '../components/Tile.svelte';
	import { state } from '../stores/state';
	import type { Pos } from '@beegone/beegone';
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
		if (selected?.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			let piece = state.get(pos);
			if (piece?.inner.type === 'bee' && piece.inner.content.color === state.turn()) {
				selected = pos;
			}
		}
	}

	function performAction(event: CustomEvent) {
		state.perform(event.detail.action);
		selected = null;
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
		class="transition-colors duration-300 bg-amber-500 dark:bg-slate-800"
		viewBox="{-viewBox / 2} {-viewBox / 2} {viewBox} {viewBox}"
		xmlns="http://www.w3.org/2000/svg"
		width="100vw"
		height="100vh"
	>
		{#each $state.positions() as pos}
			<svg x={x(pos) * 0.9} y={y(pos) * 0.92} width={tileSize} height={tileSize}>
				<Tile
					height={-0.05}
					sideClass="transition-colors duration-300 fill-amber-600 dark:fill-slate-600"
					topClass="transition-colors duration-300 fill-amber-700 dark:fill-slate-500"
				>
					<Tile
						size={0.6}
						height={0}
						topClass="transition-colors duration-300 fill-amber-800 dark:fill-slate-600"
					/>
					{#if state.get(pos) != null}
						<PieceComponent on:click={() => select(pos)} piece={state.get(pos)} />
					{/if}
					<ActionButtonGroup
						{selected}
						{pos}
						on:action={performAction}
						actions={actionsOn(pos)}
						piece={state.get(pos)?.inner}
					/>
				</Tile>
			</svg>
		{/each}
	</svg>
</div>
