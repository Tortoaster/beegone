<script lang="ts">
	import BeeComponent from '$lib/components/BeeComponent.svelte';
	import { Bee, Color, Species } from 'beegone';

	const VIEW_BOX = 360;
</script>

<BeeComponent bee={new Bee(Color.Light, Species.Queen)} x={0} y={0} width={10} height={10} />
