<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { PowerSystem } from '$lib/bindings';

  interface Props {
    value: PowerSystem;
    onchange: (power: PowerSystem) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  const options: { value: PowerSystem; label: string; helper: string }[] = [
    { value: 'DC', label: m.settings_power_dc(), helper: m.settings_power_dc_helper() },
    { value: 'AC', label: m.settings_power_ac(), helper: m.settings_power_ac_helper() },
    { value: 'DCC', label: m.settings_power_dcc(), helper: m.settings_power_dcc_helper() }
  ];

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    onchange(target.value as PowerSystem);
  }
</script>

<div class="form-group">
  <label for="powerSystem" class="form-label">{m.settings_power_label()}</label>
  <select
    id="powerSystem"
    {value}
    onchange={handleChange}
    {disabled}
    class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40 disabled:cursor-not-allowed disabled:opacity-50"
  >
    {#each options as option (option.value)}
      <option value={option.value}>
        {option.label} - {option.helper}
      </option>
    {/each}
  </select>
</div>
