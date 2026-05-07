<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import FormationForm from '$lib/features/train-formations/components/FormationForm.svelte';
  import type { FormationCategoryView } from '$lib/bindings';

  let {
    categories = [],
    initial = undefined,
    onsubmit = undefined,
    oncancel = undefined
  }: {
    categories?: FormationCategoryView[];
    initial?: any;
    onsubmit?: (data: any) => void | Promise<void>;
    oncancel?: () => void;
  } = $props();

  // Provide default implementations for optional callbacks
  const handleSubmit = $derived((onsubmit ?? (async () => {})) as (args: any) => Promise<void>);
  const handleCancel = $derived(oncancel ?? (() => {}));
</script>

<Tooltip.Provider>
  <FormationForm {categories} {initial} onsubmit={handleSubmit} oncancel={handleCancel} />
</Tooltip.Provider>
