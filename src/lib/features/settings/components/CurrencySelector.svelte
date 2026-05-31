<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    value: string;
    onchange: (currency: string) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  function handleChange(event: Event) {
    const target = event.target as HTMLInputElement;
    onchange(target.value);
  }
</script>

<div class="form-group">
  <label for="currency" class="form-label">{m.currency_label()}</label>
  <input
    id="currency"
    type="text"
    {value}
    oninput={handleChange}
    {disabled}
    maxlength="10"
    placeholder="EUR"
    class="form-input w-full"
  />
  <p class="form-helper">Max 10 characters (e.g., EUR, USD, GBP)</p>
</div>
