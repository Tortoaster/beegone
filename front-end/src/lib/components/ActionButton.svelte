<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { fly, type FlyParams } from 'svelte/transition';
	import type { Action, Piece } from 'beegone';

	export interface ActionButtonProps extends SVGAttributes<SVGElement> {
		action: Action;
		x: number;
		y: number;
		size: number;
		fromX: number;
		fromY: number;
		piece: Piece | undefined;
		delay: number;
		onaction: (action: Action) => void;
	}

	const { action, x, y, size, piece, fromX, fromY, delay, onclick, onaction, ...props }: ActionButtonProps = $props();

	const inTransition: FlyParams = $derived({
		x: fromX - x,
		y: fromY - y,
		delay: (Math.atan2(y - fromY, x - fromX) + Math.PI) * 30 + delay,
		duration: 200
	});
	const outTransition: FlyParams = $derived({
		x: x - fromX,
		y: y - fromY,
		duration: 200
	});
	const icon: string = $derived(action.move ? piece === undefined ? 'move' : piece.bee ? 'attack' : 'dig' : action.spawn?.spawn.bee ? action.spawn?.spawn.bee.species : 'build');
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	x={x - size / 2}
	y={y - size / 2}
	width={size}
	height={size}
	class="group cursor-pointer select-none"
	in:fly|global={inTransition}
	out:fly|global={outTransition}
	onclick={(e) => {
		onclick?.(e)
		onaction(action)
	}}
	{...props}
>
	<defs>
		<filter id="make-white">
			<feFlood flood-color="white" />
			<feComposite operator="in" in2="SourceGraphic" />
		</filter>
	</defs>
	<circle class="fill-black group-hover:fill-white" cx="50%" cy="50%" r="50%" />
	<image
		xlink:href="/{icon}.svg"
		class="group-hover:filter-none"
		x="25%"
		y="25%"
		width="50%"
		height="50%"
		filter="url(#make-white)"
	/>
</svg>
