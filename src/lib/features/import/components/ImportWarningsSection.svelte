<script lang="ts">
  import type { ImportPreviewResponse } from '$lib/bindings';

  interface Props {
    preview: ImportPreviewResponse;
  }

  const { preview }: Props = $props();

  const hasWarnings = $derived(preview.warnings.length > 0);
</script>

{#if hasWarnings}
  <div class="preview-warnings">
    <h3 class="warning-title">Warnings</h3>
    <ul class="warning-list">
      {#each preview.warnings as warning (warning.message)}
        <li class="warning-item">
          {warning.message}
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .preview-warnings {
    padding: 1rem;
    border-radius: var(--radius-md);
    border: 1px solid hsl(var(--warning) / 0.5);
    background: hsl(var(--warning) / 0.05);
  }

  .warning-title {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
  }

  .warning-list {
    list-style: disc inside;
    margin: 0;
    padding: 0;
  }

  .warning-item {
    font-size: 0.875rem;
    padding: 0.25rem 0;
  }
</style>
