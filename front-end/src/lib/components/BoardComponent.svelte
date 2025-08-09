<script lang="ts">
	import { Action, Board, type Pos, State } from 'beegone';
	import Polygon from '$lib/svg/Polygon.svelte';
	import ActionButtonGroup from '$lib/components/ActionButtonGroup.svelte';
	import BeeToken from '$lib/components/BeeToken.svelte';

	const VIEW_BOX = 360;
	// The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
	const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3));
	const PADDING = 5;
	const TILE_SIZE = PADDED_TILE_SIZE - PADDING;
	const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2;
	const TILE_RADIUS = TILE_SIZE / 2;

	interface Props {
		gameState: State;
		onaction: (action: Action) => void;
	}

	const { gameState, onaction: _onaction }: Props = $props();

	let selected: Pos | null = $state(null);
	const actions = $derived(selected ? gameState.actionsFrom(selected) : []);

	const onaction = $derived((action: Action) => {
		selected = null;
		_onaction(action);
	});

	function select(pos: Pos) {
		if (selected?.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			let piece = gameState.board.get(pos);
			if (piece?.bee && piece.bee.color === gameState.turn) {
				selected = pos;
			}
		}
	}

	const actionsOnTile = (actions: Action[], pos: Pos) =>
		actions.filter((action) =>
			action.move
				? action.move.to.q === pos.q && action.move.to.r === pos.r
				: action.spawn
					? action.spawn.on.q === pos.q && action.spawn.on.r === pos.r
					: false
		);
</script>

<svg
	class="bg-white"
	viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
	xmlns="http://www.w3.org/2000/svg"
	width="100%"
	height="100%"
>
	{#each Board.positions() as pos}
		{@const piece = gameState.board.get(pos)}
		<Polygon
			class={["transition-colors", pos.q === selected?.q && pos.r === selected.r ? "fill-accent" : "fill-gray"]}
			cx={PADDED_TILE_RADIUS * pos.x}
			cy={PADDED_TILE_RADIUS * pos.y}
			r={TILE_RADIUS}
			sides={6}
			cornerRadius={8}
		/>
		{#if piece}
			{#if piece.bee}
				<BeeToken
					bee={piece.bee}
					width={TILE_SIZE}
					height={TILE_SIZE}
					x={PADDED_TILE_RADIUS * pos.x - TILE_RADIUS}
					y={PADDED_TILE_RADIUS * pos.y - TILE_RADIUS}
					onclick={() => select(pos)}
				/>
			{:else}
				<Polygon
					class="fill-secondary"
					cx={PADDED_TILE_RADIUS * pos.x}
					cy={PADDED_TILE_RADIUS * pos.y}
					r={TILE_RADIUS}
					sides={6}
					cornerRadius={8}
				/>
			{/if}
		{/if}
		<ActionButtonGroup
			cx={PADDED_TILE_RADIUS * pos.x}
			cy={PADDED_TILE_RADIUS * pos.y}
			r={PADDED_TILE_RADIUS / 2}
			size={0.4 * TILE_SIZE}
			actions={actionsOnTile(actions, pos)}
			piece={gameState.board.get(pos)}
			{selected}
			{pos}
			{onaction}
		/>
	{/each}
</svg>
