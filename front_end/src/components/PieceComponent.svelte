<script lang="ts">
	import { send, receive } from '../animation/transition';
	import type { Piece } from '@beegone/beegone';
	import Tile from './Tile.svelte';
	import Token from './Token.svelte';
	import DroneIcon from '../icons/DroneIcon.svelte';
	import WorkerIcon from '../icons/WorkerIcon.svelte';
	import NurseIcon from '../icons/NurseIcon.svelte';
	import ExplorerIcon from '../icons/ExplorerIcon.svelte';
	import BuilderIcon from '../icons/BuilderIcon.svelte';
	import GuardIcon from '../icons/GuardIcon.svelte';
	import QueenIcon from '../icons/QueenIcon.svelte';

	export let piece: Piece;

	$: pieceIcon = piece.kind.type === 'bee' ? `/icons/${piece.kind.content.species}.svg` : null;
	$: sideClass =
		piece.kind.content?.color === 'light'
			? 'fill-amber-400 dark:fill-slate-300'
			: 'fill-amber-900 dark:fill-slate-900';
	$: topClass =
		piece.kind.content?.color === 'light'
			? 'fill-amber-300 dark:fill-slate-100'
			: 'fill-amber-800  dark:fill-slate-800';
	$: iconClass =
		piece.kind.content?.color === 'light'
			? 'fill-amber-500 dark:fill-slate-400'
			: 'fill-amber-950 dark:fill-slate-950';

	const VIEW_BOX = 360;
	const ICON_SIZE = 0.25 * VIEW_BOX;
</script>

<svg
	viewBox="0 0 {VIEW_BOX} {VIEW_BOX}"
	class="cursor-pointer"
	in:receive={{ key: piece.id }}
	out:send={{ key: piece.id }}
>
	{#if piece.kind.type === 'wall'}
		<Tile
			height={0.15}
			sideClass="fill-amber-600 dark:fill-slate-600"
			topClass="fill-amber-400 dark:fill-slate-400"
		>
			<Tile size={0.6} height={0} topClass="fill-amber-300 dark:fill-slate-300" />
		</Tile>
	{:else}
		<Token on:click size={0.45} height={0.1} {sideClass} {topClass}>
			{#if piece.kind.content.species === 'drone'}
				<DroneIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'worker'}
				<WorkerIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'nurse'}
				<NurseIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'explorer'}
				<ExplorerIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'builder'}
				<BuilderIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'guard'}
				<GuardIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{:else if piece.kind.content.species === 'queen'}
				<QueenIcon
					topClass={iconClass}
					x={(VIEW_BOX - ICON_SIZE) / 2}
					y={(VIEW_BOX - ICON_SIZE) / 2}
					width={ICON_SIZE}
					height={ICON_SIZE}
				/>
			{/if}
		</Token>
	{/if}
</svg>
