<script lang="ts">
	import PieceComponent from '../components/PieceComponent.svelte'
	import Hexagon from '../components/Hexagon.svelte'
	import type {Piece, Pos} from '../model/model';
	import {Color, Species} from '../model/model';

	const BOARD_RADIUS = 3;
	const TILE_SIZE = 50;

	let board: (Piece | null | undefined)[][] = [
		[undefined, undefined, undefined, null, {type: "Bee", content: [Color.Dark, Species.Nurse]}, null, {
			type: "Bee",
			content: [Color.Dark, Species.Drone]
		}],
		[undefined, undefined, {type: "Wall", content: undefined}, null, null, {
			type: "Bee",
			content: [Color.Dark, Species.Queen]
		}, null],
		[undefined, {type: "Wall", content: undefined}, {type: "Wall", content: undefined}, {
			type: "Wall",
			content: undefined
		}, null, null, {type: "Bee", content: [Color.Dark, Species.Nurse]}],
		[null, null, {type: "Wall", content: undefined}, {type: "Wall", content: undefined}, {
			type: "Wall",
			content: undefined
		}, null, null],
		[{type: "Bee", content: [Color.Light, Species.Nurse]}, null, null, {
			type: "Wall",
			content: undefined
		}, {type: "Wall", content: undefined}, {type: "Wall", content: undefined}, undefined],
		[null, {type: "Bee", content: [Color.Light, Species.Queen]}, null, null, {
			type: "Wall",
			content: undefined
		}, undefined, undefined],
		[{type: "Bee", content: [Color.Light, Species.Drone]}, null, {
			type: "Bee",
			content: [Color.Light, Species.Nurse]
		}, null, undefined, undefined, undefined],
	];

	let selected: Pos | null = null;
	let turn = Color.Light;
	const endTurn = () => {
		selected = null;
		if (turn === Color.Light) {
			turn = Color.Dark
		} else {
			turn = Color.Light
		}
	}

	function select(pos: Pos) {
		if (selected == null) {
			let piece = board[pos.q + BOARD_RADIUS][pos.r + BOARD_RADIUS];
			if (piece?.type === "Bee" && piece.content[0] === turn) {
				selected = pos;
			}
		} else if (selected.q === pos.q && selected.r === pos.r) {
			selected = null;
		} else {
			const destination = board[pos.q + BOARD_RADIUS][pos.r + BOARD_RADIUS];
			if (destination == null) {
				// Move
				let piece = board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS]!;
				board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS] = null;
				board[pos.q + BOARD_RADIUS][pos.r + BOARD_RADIUS] = piece;
				endTurn();
			} else {
				switch (destination.type) {
					case "Bee":
						if (destination.content[0] != turn) {
							// Capture
							let piece = board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS]!;
							board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS] = null;
							board[pos.q + BOARD_RADIUS][pos.r + BOARD_RADIUS] = piece;
							endTurn();
						} else {
							selected = pos;
						}
						break;
					case "Wall":
						// Break wall
						let piece = board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS]!;
						board[selected.q + BOARD_RADIUS][selected.r + BOARD_RADIUS] = null;
						board[pos.q + BOARD_RADIUS][pos.r + BOARD_RADIUS] = piece;
						endTurn();
						break;
				}
			}
		}
	}

	const screen_x = (pos: Pos) => TILE_SIZE / 2 * (3. / 2 * pos.q) + TILE_SIZE * (BOARD_RADIUS - 0.5);
	const screen_y = (pos: Pos) => TILE_SIZE / 2 * (Math.sqrt(3) / 2 * pos.q + Math.sqrt(3) * pos.r) + TILE_SIZE * (BOARD_RADIUS - 0.5);

	const range = (start: number, stop: number, step: number = 1) =>
			Array.from({length: (stop - start) / step + 1}, (_, i) => start + i * step);
</script>

<svg
		viewBox="0 0 300 300"
		xmlns="http://www.w3.org/2000/svg"
		width="100vw"
		height="100vh">
	{#each range(-BOARD_RADIUS, BOARD_RADIUS) as q}
		{#each range(-BOARD_RADIUS, BOARD_RADIUS) as r}
			{#if board[q + BOARD_RADIUS][r + BOARD_RADIUS] !== undefined}
				<svg viewBox="0 0 300 300" x={screen_x({q, r})} y={screen_y({q, r})} width={TILE_SIZE}
					 height={TILE_SIZE}>
					<Hexagon on:click={() => select({q, r})} selected={selected?.q === q && selected.r === r}/>
					{#if board[q + BOARD_RADIUS][r + BOARD_RADIUS] != null}
						<PieceComponent on:click={(_) => select({q, r})}
										piece={board[q + BOARD_RADIUS][r + BOARD_RADIUS]}/>
					{/if}
				</svg>
			{/if}
		{/each}
	{/each}
</svg>
