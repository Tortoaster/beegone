import { crossfade, fly } from 'svelte/transition';

export const [send, receive] = crossfade({
	duration: (d) => d * 3,

	fallback(node) {
		return fly(node, { duration: 400, y: -100 });
	},
});
