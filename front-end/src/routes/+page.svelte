<script lang="ts">
  import { Action, State } from "beegone";
	import BoardComponent from '$lib/components/BoardComponent.svelte';

  let gameState: State = $state(new State());

  function onaction(action: Action) {
    gameState = gameState.perform(action);
  }
</script>

<div class="bg-amber-500 w-dvw h-dvh">
  <BoardComponent {gameState} {onaction} />
</div>
