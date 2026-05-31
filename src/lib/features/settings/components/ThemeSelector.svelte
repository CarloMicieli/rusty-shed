<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { AppTheme } from '$lib/bindings';

  interface Props {
    value: AppTheme;
    onchange: (theme: AppTheme) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  const options: { value: AppTheme; label: string }[] = [
    { value: 'steampunk-light', label: m.settings_theme_light() },
    { value: 'steampunk-dark', label: m.settings_theme_dark() }
  ];

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    onchange(target.value as AppTheme);
  }
</script>

<div class="form-group">
  <label for="theme" class="form-label">{m.settings_theme_label()}</label>
  <select
    id="theme"
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
