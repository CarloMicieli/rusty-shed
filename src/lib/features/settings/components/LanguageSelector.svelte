<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { Language } from '$lib/bindings';

  interface Props {
    value: Language;
    onchange: (language: Language) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  const options: { value: Language; label: string }[] = [
    { value: 'en', label: 'English' },
    { value: 'it', label: 'Italiano' }
  ];

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    onchange(target.value as Language);
  }
</script>

<div class="form-group">
  <label for="language" class="form-label">{m.settings_language_label()}</label>
  <select id="language" {value} onchange={handleChange} {disabled} class="form-select w-full">
    {#each options as option (option.value)}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
