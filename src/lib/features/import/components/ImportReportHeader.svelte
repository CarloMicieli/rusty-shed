<script lang="ts">
  import { CheckCircle2, XCircle } from 'lucide-svelte';
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
    {#if isSuccess}
      <CheckCircle2 size={32} />
    {:else}
      <XCircle size={32} />
    {/if}
  </div>
  <h2 class="font-bebas text-2xl tracking-widest uppercase">IMPORT LOG: {title}</h2>
  <div class="duration-readout">
    <span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase"
      >DURATION:</span
    >
    <span class="font-mono text-sm text-primary">{formatDuration(durationMs)}</span>
  </div>
</div>

<style>
  .report-header {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .status-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .status-icon.success {
    color: hsl(var(--primary));
    filter: drop-shadow(0 0 6px hsl(var(--primary) / 0.6));
  }

  .status-icon.error {
    color: hsl(var(--destructive));
    filter: drop-shadow(0 0 6px hsl(var(--destructive) / 0.6));
  }

  .duration-readout {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }
</style>
