<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    isSuccess: boolean;
    durationMs: number;
  }

  const { isSuccess, durationMs }: Props = $props();

  function formatDuration(ms: number): string {
    const msNumber = Number(ms);
    if (msNumber < 1000) return `${msNumber}ms`;
    const seconds = (msNumber / 1000).toFixed(1);
    return `${seconds}s`;
  }

  const title = $derived(
    isSuccess ? m.import_report_title_complete() : m.import_report_title_failed()
  );
</script>

<div class="report-header">
  <div class="status-icon {isSuccess ? 'success' : 'error'}">
    {isSuccess ? '✓' : '✗'}
  </div>
  <h2>{title}</h2>
  <p class="report-subtitle">
    {m.import_report_completed_in({ duration: formatDuration(durationMs) })}
  </p>
</div>

<style>
  .report-header {
    text-align: center;
  }

  .status-icon {
    width: 4rem;
    height: 4rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
    margin: 0 auto 1rem;
  }

  .status-icon.success {
    background: hsl(var(--success) / 0.1);
    color: hsl(var(--success));
  }

  .status-icon.error {
    background: hsl(var(--destructive) / 0.1);
    color: hsl(var(--destructive));
  }

  .report-subtitle {
    color: hsl(var(--muted-foreground));
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }
</style>
