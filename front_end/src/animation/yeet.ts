import { cubicOut } from 'svelte/easing';

export function yeet(
	node: Element,
	{ delay = 0, duration = 400, easing = cubicOut, x = 0, y = 0 } = {}
) {
	const style = getComputedStyle(node);
	const transform = style.transform === 'none' ? '' : style.transform;
	return {
		delay,
		duration,
		easing,
		css: (t: number) =>
			`transform: ${transform} translate(${(1 - t) * x}px, ${
				(1 - t) * y
			}px) scale(${t}); transform-origin: center;`,
	};
}
