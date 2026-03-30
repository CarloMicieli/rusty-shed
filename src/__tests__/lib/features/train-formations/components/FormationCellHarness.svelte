<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import FormationCell from '$lib/features/train-formations/components/FormationCell.svelte';
  import type { FormationElementView } from '$lib/bindings';

  let {
    element,
    onRemove,
    onOpenPicker,
    onTractionToggle
  }: {
    element: FormationElementView;
    onRemove?: (id: string) => void;
    onOpenPicker?: (elementId: string) => void;
    onTractionToggle?: (elementId: string, override: number) => void;
  } = $props();
</script>

<Tooltip.Provider>
  <FormationCell {element} {onRemove} {onOpenPicker} {onTractionToggle} />
</Tooltip.Provider>
