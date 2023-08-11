<script lang="ts">
	export let selected: boolean;

	$: color = selected ? '#BBB' : '#888';
</script>

<path on:click fill={color} d="m300,150 l-75,130 l-150,0 l-75,-130 l75,-130 l150,0z" />
