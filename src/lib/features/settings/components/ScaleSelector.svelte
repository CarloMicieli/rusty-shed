<script lang="ts">
  import * as m from '$lib/paraglide/messages';

  interface Props {
    value: string;
    onchange: (scale: string) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  function handleChange(event: Event) {
    const target = event.target as HTMLInputElement;
    onchange(target.value);
  }
</script>

<div class="form-group">
  <label for="favouriteScale" class="form-label">{m.settings_scale_label()}</label>
  <input
    id="favouriteScale"
    type="text"
    {value}
    oninput={handleChange}
    {disabled}
    maxlength="20"
    placeholder="HO, N, OO, etc."
    class="form-input w-full"
  />
  <p class="form-helper">Your preferred model railway scale (optional, max 20 characters)</p>
</div>
