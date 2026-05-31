<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { MeasureUnit } from '$lib/bindings';

  interface Props {
    value: MeasureUnit;
    onchange: (unit: MeasureUnit) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  const options: { value: MeasureUnit; label: string; helper: string }[] = [
    { value: 'Metric', label: m.settings_unit_metric(), helper: 'Millimeters' },
    { value: 'Imperial', label: m.settings_unit_imperial(), helper: 'Inches' }
  ];

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    onchange(target.value as MeasureUnit);
  }
</script>

<div class="form-group">
  <label for="measureUnit" class="form-label">{m.settings_unit_label()}</label>
  <select
    id="measureUnit"
    {value}
    onchange={handleChange}
    {disabled}
    class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40 disabled:cursor-not-allowed disabled:opacity-50"
  >
    {#each options as option (option.value)}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
