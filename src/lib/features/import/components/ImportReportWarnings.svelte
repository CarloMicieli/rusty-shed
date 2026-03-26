<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Warning {
    message: string;
  }

  interface Props {
    warnings: Warning[];
  }

  const { warnings }: Props = $props();

  const hasWarnings = $derived(warnings.length > 0);
</script>

{#if hasWarnings}
  <div class="report-warnings">
    <h3 class="warning-title">{m.import_report_warnings_title()}</h3>
    <ul class="warning-list">
      {#each warnings as warning (warning.message)}
        <li class="warning-item">{warning.message}</li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .report-warnings {
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
