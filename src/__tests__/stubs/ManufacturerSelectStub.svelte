<script lang="ts">
  let {
    id,
    required = false,
    error
  }: {
    id?: string;
    required?: boolean;
    error?: string;
  } = $props();
</script>

<div data-testid={id ?? 'manufacturer-select'}>
  <span>Manufacturer{required ? ' *' : ''}</span>
  {#if error}
    <span>{error}</span>
  {/if}
</div>