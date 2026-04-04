<script lang="ts">
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';
  import type { SelectOption } from '$lib/components/SearchableSelect.svelte';

  const options: SelectOption[] = [
    { value: 'IT', label: 'Italy', countryCode: 'IT', registeredCompanyName: 'FS Italiane' },
    { value: 'DE', label: 'Germany', countryCode: 'DE', registeredCompanyName: 'Deutsche Bahn' }
  ];
  let value = $state('IT');
</script>

<SearchableSelect {options} bind:value id="test-select">
  {#snippet item(opt)}
    <span data-testid="custom-item-{opt.value}">{opt.label} ({opt.countryCode})</span>
  {/snippet}
  {#snippet trigger(opt)}
    <span data-testid="custom-trigger">{opt?.label}</span>
  {/snippet}
</SearchableSelect>
