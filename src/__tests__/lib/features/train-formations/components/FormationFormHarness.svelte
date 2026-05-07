<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import FormationForm from '$lib/features/train-formations/components/FormationForm.svelte';

  let {
    initialName = '',
    initialStartYear = new Date().getFullYear(),
    initialEndYear = new Date().getFullYear(),
    onsubmit = undefined,
    oncancel = undefined
  }: {
    initialName?: string;
    initialStartYear?: number;
    initialEndYear?: number;
    onsubmit?: (data: any) => void | Promise<void>;
    oncancel?: () => void;
  } = $props();
</script>

<Tooltip.Provider>
  <FormationForm
    {initialName}
    {initialStartYear}
    {initialEndYear}
    {onsubmit}
    {oncancel}
  />
</Tooltip.Provider>
