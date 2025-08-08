<script lang="ts">
	import { Board, type Pos } from 'beegone';
	import { createState, type StateStore } from '../stores/state';
	import ActionButtonGroup from '$lib/components/ActionButtonGroup.svelte';
	import BeeToken from '$lib/components/BeeToken.svelte';
	import LightSwitch from '$lib/components/LightSwitch.svelte';
	import Polygon from '$lib/components/Polygon.svelte';

	const VIEW_BOX = 360;
	// The field is 7 tiles high, and a flat hexagon's height is `sin(60deg)`% of its size.
	const PADDED_TILE_SIZE = VIEW_BOX / (7 * Math.sin(Math.PI / 3));
	const PADDING = 5;
	const TILE_SIZE = PADDED_TILE_SIZE - PADDING;
	const PADDED_TILE_RADIUS = PADDED_TILE_SIZE / 2;
	const TILE_RADIUS = TILE_SIZE / 2;

	let state: StateStore | undefined = undefined;

	let selected: Pos | null = null;

	$: actions = selected ? state?.actionsFrom(selected) ?? [] : [];

	const piecesOn = (pos: Pos) => {
		const piece = state?.get(pos);
		return piece ? [piece] : [];
	};

	function select(pos: Pos) {
		if (selected?.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			let piece = state?.get(pos);
			if (piece?.bee && piece.bee.color === state?.turn()) {
				selected = pos;
			}
		}
	}

	async function performAction(event: CustomEvent) {
		state?.perform(event.detail.action);
		selected = null;
	}
</script>

<div
	class="transition-colors duration-300 bg-amber-500 dark:bg-slate-800 p-4 size-full absolute"
>
	{#if state === undefined}
		<div class="size-full height-full flex flex-col md:flex-row md:justify-center md:items-center gap-4">
			<button
				class="basis-1/2 md:max-w-96 md:max-h-80 flex flex-col justify-center items-center bg-amber-300 hover:bg-amber-200 dark:bg-slate-600 hover:dark:bg-slate-500 p-4 rounded-2xl"
				on:click={() => (state = createState(2))}
			>
				<span class="text-3xl font-bold text-amber-500 dark:text-slate-500">vs.</span>
				<svg xmlns="http://www.w3.org/2000/svg" width="5em" height="8em">
					<defs>
						<filter id="icon-color">
							<feFlood class="transition-all duration-300 flood-amber-900 dark:flood-slate-100" />
							<feComposite operator="in" in2="SourceGraphic" />
						</filter>
					</defs>
					<image xlink:href="/player.svg" width="100%" height="100%" filter="url(#icon-color)" />
				</svg>
				<span class="text-5xl font-bold text-amber-700 dark:text-slate-300">Player</span>
			</button>
			<button
				class="basis-1/2 md:max-w-96 md:max-h-80 flex flex-col justify-center items-center bg-amber-300 hover:bg-amber-200 dark:bg-slate-600 hover:dark:bg-slate-500 p-4 rounded-2xl"
				on:click={() => (state = createState(1))}
			>
				<span class="text-3xl font-bold text-amber-500 dark:text-slate-500">vs.</span>
				<svg xmlns="http://www.w3.org/2000/svg" width="8em" height="8em">
					<defs>
						<filter id="icon-color">
							<feFlood class="transition-all duration-300 flood-amber-900 dark:flood-slate-100" />
							<feComposite operator="in" in2="SourceGraphic" />
						</filter>
					</defs>
					<image xlink:href="/robot.svg" width="100%" height="100%" filter="url(#icon-color)" />
				</svg>
				<span class="text-5xl font-bold text-amber-700 dark:text-slate-300">Computer</span>
			</button>
		</div>
	{:else}
		<div class="fixed top-0 right-0 m-4">
			<LightSwitch />
		</div>
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
						class="transition-all duration-300 flood-amber-600 dark:flood-slate-600"
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
						class="transition-all duration-300 flood-amber-600 dark:flood-slate-600"
						result="Color"
					/>
					<feComposite operator="in" in="Color" in2="SourceGraphic" result="Shadow" />
					<feOffset in="SourceGraphic" dy="-8" result="Offset" />
					<feFlood class="transition-all duration-300 flood-amber-600 dark:flood-slate-600" />
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
					class="transition-colors duration-300 fill-amber-700 dark:fill-slate-500"
					cx={PADDED_TILE_RADIUS * pos.x}
					cy={PADDED_TILE_RADIUS * pos.y}
					r={TILE_RADIUS}
					sides={6}
					cornerRadius={8}
					filter="url(#tile-filter)"
				/>
				<Polygon
					class="transition-colors duration-300 fill-amber-800 dark:fill-slate-600"
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
							class="transition-colors duration-300 fill-amber-400 dark:fill-slate-400"
							cx={PADDED_TILE_RADIUS * pos.x}
							cy={PADDED_TILE_RADIUS * pos.y}
							r={TILE_RADIUS}
							sides={6}
							cornerRadius={8}
							filter="url(#wall-filter)"
						/>
						<Polygon
							class="transition-colors duration-300 fill-amber-300 dark:fill-slate-300"
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
					piece={state.get(pos)}
					{selected}
					{pos}
					on:action={performAction}
				/>
			{/each}
		</svg>
	{/if}
</div>
