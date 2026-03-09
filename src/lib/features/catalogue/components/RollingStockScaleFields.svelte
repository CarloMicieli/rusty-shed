<script lang="ts">
  import FormField from '$lib/shared/ui/FormField.svelte';
  import { Input } from '$lib/components';
  import type { ConstantItem } from '../constants';
  import type { RollingStockForm } from '../utils';

  interface Props {
    rs: RollingStockForm;
    errorsFn: (field: string) => string | undefined;
    formLabels: Record<string, ConstantItem>;
    /** Whether road_number is required (Locomotive = true, others = false) */
    roadNumberRequired?: boolean;
  }

  const { rs, errorsFn, formLabels, roadNumberRequired = false }: Props = $props();
</script>

<FormField label={formLabels.friendlyName} error={errorsFn('friendly_name')} required>
  <Input type="text" bind:value={rs.friendly_name} />
</FormField>

<FormField label={formLabels.seriesCode} error={errorsFn('series_code')} required>
  <Input type="text" bind:value={rs.series_code} />
</FormField>

<FormField
  label={formLabels.roadNumber}
  error={errorsFn('road_number')}
  required={roadNumberRequired}
>
  <Input type="text" bind:value={rs.road_number} />
</FormField>

<FormField label={formLabels.series} error={errorsFn('series')}>
  <Input type="text" bind:value={rs.series} />
</FormField>

<FormField label={formLabels.depot} error={errorsFn('depot')}>
  <Input type="text" bind:value={rs.depot} />
</FormField>
