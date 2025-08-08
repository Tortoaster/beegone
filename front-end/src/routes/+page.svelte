<script lang="ts">
	import { Board, type Pos, State } from 'beegone';
	import ActionButtonGroup from '$lib/components/ActionButtonGroup.svelte';
	import BeeToken from '$lib/components/BeeToken.svelte';
	import Polygon from '$lib/components/Polygon.svelte';

	const VIEW_BOX = 360;
	// The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
	const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3));
	const PADDING = 5;
	const TILE_SIZE = PADDED_TILE_SIZE - PADDING;
	const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2;
	const TILE_RADIUS = TILE_SIZE / 2;

	let gameState: State = $state(new State());
	let selected: Pos | null = $state(null);

	const actions = $derived(selected ? gameState.actionsFrom(selected) : []);

	const piecesOn = (pos: Pos) => {
		const piece = gameState.board.get(pos);
		return piece ? [piece] : [];
	};

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

	async function performAction(event: CustomEvent) {
		gameState = gameState.perform(event.detail.action);
		selected = null;
	}
</script>

<div
	class="bg-amber-500 p-4 size-full absolute"
>
	<svg
		viewBox="{-VIEW_BOX / 2} {-VIEW_BOX / 2} {VIEW_BOX} {VIEW_BOX}"
		xmlns="http://www.w3.org/2000/svg"
		width="100%"
		height="100%"
	>
		<defs>
			<filter id="tile-lower-filter" y="-20%" height="140%">
				<feOffset dy="4" />
			</filter>
			<filter id="tile-filter">
				<feOffset dy="4" result="Offset" />
				<feComposite operator="out" in="SourceGraphic" in2="Offset" result="Side" />
				<feFlood
					class="flood-amber-600"
					result="Shadow"
				/>
				<feComposite operator="in" in="Shadow" in2="Side" result="Side" />
				<feMerge>
					<feMergeNode in="SourceGraphic" />
					<feMergeNode in="Side" />
				</feMerge>
			</filter>
			<filter id="wall-higher-filter" y="-25%" height="125%">
				<feOffset dy="-8" />
			</filter>
			<filter id="wall-filter" x="0%" y="-25%" width="100%" height="125%">
				<feFlood
					class="flood-amber-600"
					result="Color"
				/>
				<feComposite operator="in" in="Color" in2="SourceGraphic" result="Shadow" />
				<feOffset in="SourceGraphic" dy="-8" result="Offset" />
				<feFlood class="flood-amber-600" />
				<feOffset dy={TILE_RADIUS} result="Below" />
				<feFlood height={PADDED_TILE_RADIUS + 6} result="Above" />
				<feComposite operator="in" in="Below" in2="Above" result="Side" />
				<feMerge>
					<feMergeNode in="Shadow" />
					<feMergeNode in="Side" />
					<feMergeNode in="Offset" />
				</feMerge>
			</filter>
		</defs>
		{#each Board.positions() as pos}
			<Polygon
				class="fill-amber-700"
				cx={PADDED_TILE_RADIUS * pos.x}
				cy={PADDED_TILE_RADIUS * pos.y}
				r={TILE_RADIUS}
				sides={6}
				cornerRadius={8}
				filter="url(#tile-filter)"
			/>
			<Polygon
				class="fill-amber-800"
				cx={PADDED_TILE_RADIUS * pos.x}
				cy={PADDED_TILE_RADIUS * pos.y}
				r={TILE_RADIUS * 0.75}
				sides={6}
				cornerRadius={6}
				filter="url(#tile-lower-filter)"
				on:click={() => console.log(pos)}
			/>
			{#each piecesOn(pos) as piece}
				{#if piece.bee}
					<BeeToken
						bee={piece.bee}
						width={TILE_SIZE}
						height={TILE_SIZE}
						filter="url(#tile-lower-filter)"
						x={PADDED_TILE_RADIUS * pos.x - TILE_RADIUS}
						y={PADDED_TILE_RADIUS * pos.y - TILE_RADIUS}
						on:click={() => select(pos)}
					/>
				{:else}
					<Polygon
						class="fill-amber-400"
						cx={PADDED_TILE_RADIUS * pos.x}
						cy={PADDED_TILE_RADIUS * pos.y}
						r={TILE_RADIUS}
						sides={6}
						cornerRadius={8}
						filter="url(#wall-filter)"
					/>
					<Polygon
						class="fill-amber-300"
						cx={PADDED_TILE_RADIUS * pos.x}
						cy={PADDED_TILE_RADIUS * pos.y}
						r={TILE_RADIUS * 0.75}
						sides={6}
						cornerRadius={6}
						filter="url(#wall-higher-filter)"
					/>
				{/if}
			{/each}
			<ActionButtonGroup
				cx={PADDED_TILE_RADIUS * pos.x}
				cy={PADDED_TILE_RADIUS * pos.y}
				r={PADDED_TILE_RADIUS / 2}
				size={0.4 * TILE_SIZE}
				actions={actions.filter((action) =>
						action.move
							? action.move.to.q === pos.q && action.move.to.r === pos.r
							: action.spawn ? action.spawn.on.q === pos.q && action.spawn.on.r === pos.r : {}
					)}
				piece={gameState.board.get(pos)}
				{selected}
				{pos}
				on:action={performAction}
			/>
		{/each}
	</svg>
</div>
