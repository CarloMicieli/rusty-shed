<script lang="ts">
  import type { ImportPreviewResponse } from '$lib/bindings';

  interface Props {
    preview: ImportPreviewResponse;
  }

  const { preview }: Props = $props();

  const hasErrors = $derived(preview.errors.length > 0);
</script>

{#if hasErrors}
  <div class="preview-errors">
    <h3 class="error-title">❌ Validation Errors</h3>
    <p class="error-subtitle">Import cannot proceed until these issues are fixed</p>
    <ul class="error-list">
      {#each preview.errors as error (error.code + error.message + error.path)}
        <li class="error-item">
          <div class="error-content">
            <strong class="error-code">{error.code}</strong>
            <span class="error-message">{error.message}</span>
            {#if error.path}
              <span class="error-path">at {error.path}</span>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .preview-errors {
    padding: 1rem;
    border-radius: var(--radius-md);
    border: 1px solid hsl(var(--destructive) / 0.5);
    background: hsl(var(--destructive) / 0.05);
  }

  .error-title {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
  }

  .error-subtitle {
    font-size: 0.8125rem;
    color: hsl(var(--destructive));
    margin-bottom: 0.75rem;
    font-weight: 500;
  }

  .error-list {
    list-style: disc inside;
    margin: 0;
    padding: 0;
  }

  .error-item {
    font-size: 0.875rem;
    padding: 0.25rem 0;
  }

  .error-content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .error-code {
    font-size: 0.75rem;
    color: hsl(var(--destructive));
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .error-message {
    font-size: 0.875rem;
    color: hsl(var(--foreground));
  }

  .error-path {
    font-size: 0.75rem;
    color: hsl(var(--muted-foreground));
    font-family: monospace;
  }
</style>
