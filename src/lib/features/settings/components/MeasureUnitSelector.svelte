<script lang="ts">
  import * as m from '$lib/paraglide/messages';
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
  <select id="measureUnit" {value} onchange={handleChange} {disabled} class="form-select w-full">
    {#each options as option (option.value)}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
