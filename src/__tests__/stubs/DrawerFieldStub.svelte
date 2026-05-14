<script lang="ts">
  let {
    id,
    label,
    required = false,
    error
  }: {
    id?: string;
    label: string;
    required?: boolean;
    error?: string;
  } = $props();
</script>

<div data-testid={id ?? label}>
  <span>{label}{required ? ' *' : ''}</span>
  {#if error}
    <span>{error}</span>
  {/if}
</div>