<script lang="ts">
  import { page } from '$app/stores';
  import FormationBuilder from '$lib/features/train-formations/components/FormationBuilder.svelte';
  import { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte.js';

  const state = TrainFormationState.setContext();
  const id = $derived($page.params.id ?? '');

  $effect(() => {
    state.loadDetail(id);
    state.loadCategories();
  });
</script>

<FormationBuilder {state} formationId={id} />
