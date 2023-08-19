<script lang="ts">
	import type { Action } from '@beegone/beegone';
	import ActionButton from './ActionButton.svelte';

	export let actions: Action[];
</script>

{#if actions.length !== 0}
	<svg viewBox="0 0 360 360">
		{#if actions.length === 1}
			<ActionButton on:action action={actions[0]} />
		{:else if actions.length === 2}
			<svg viewBox="0 0 360 360" x="-90">
				<ActionButton on:action action={actions[0]} />
			</svg>
			<svg viewBox="0 0 360 360" x="90">
				<ActionButton on:action action={actions[1]} />
			</svg>
		{:else if actions.length === 3}
			<svg viewBox="0 0 360 360" x="45" y="-78">
				<ActionButton on:action action={actions[0]} />
			</svg>
			<svg viewBox="0 0 360 360" x="45" y="78">
				<ActionButton on:action action={actions[1]} />
			</svg>
			<svg viewBox="0 0 360 360" x="-90">
				<ActionButton on:action action={actions[2]} />
			</svg>
		{:else}
			<text fill="red">!</text>
		{/if}
	</svg>
{/if}
