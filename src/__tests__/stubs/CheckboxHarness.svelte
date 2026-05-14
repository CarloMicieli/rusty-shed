<script lang="ts">
  import { Checkbox } from '$lib/components/ui/checkbox';

  let checked = $state(false);
  let changeCount = $state(0);

  function handleChange() {
    changeCount += 1;
  }
</script>

<Checkbox aria-label="Enable feature" bind:checked onchange={handleChange} />
<p data-testid="checkbox-state">{checked ? 'checked' : 'unchecked'}</p>
<p data-testid="checkbox-changes">{changeCount}</p>
