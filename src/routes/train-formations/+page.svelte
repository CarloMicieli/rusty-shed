<script lang="ts">
  import FormationList from '$lib/features/train-formations/components/FormationList.svelte';
  import { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte.js';

  const state = TrainFormationState.setContext();

  $effect(() => {
    state.load();
    state.loadCategories();
  });
</script>

<FormationList {state} />
