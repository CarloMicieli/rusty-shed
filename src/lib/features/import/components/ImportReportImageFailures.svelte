<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface ImageFailure {
    filename: string;
    reason: string;
  }

  interface Props {
    failures: ImageFailure[];
  }

  const { failures }: Props = $props();

  const hasImageFailures = $derived(failures.length > 0);
</script>

{#if hasImageFailures}
  <div class="report-errors">
    <h3 class="error-title">{m.import_report_image_failures_title()}</h3>
    <ul class="error-list">
      {#each failures as failure (failure.filename)}
        <li class="error-item">
          <strong>{failure.filename}:</strong>
          {failure.reason}
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .report-errors {
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

  .error-list {
    list-style: disc inside;
    margin: 0;
    padding: 0;
  }

  .error-item {
    font-size: 0.875rem;
    padding: 0.25rem 0;
  }
</style>
