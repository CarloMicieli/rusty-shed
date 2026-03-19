<script lang="ts">
  import type { DigitalSummary } from '$lib/bindings';

  interface Props {
    summary: DigitalSummary | null;
    loading?: boolean;
  }

  let { summary = null, loading = false }: Props = $props();

  const formattedPercentage = $derived(summary ? summary.percentage.toFixed(1) : '0.0');
</script>

<div
  class="grid grid-cols-1 gap-3 rounded-2xl border border-border/50 bg-muted/30 p-4 sm:grid-cols-3"
>
  <!-- Digitalization Rate -->
  <div
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
      Digitalization Rate
    </p>
    <p class="mt-1 text-lg font-bold text-primary">
      {#if loading}&mdash;{:else}{formattedPercentage}%{/if}
    </p>
  </div>

  <!-- Total Fleet -->
  <div
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
      Total Fleet
    </p>
    <p class="mt-1 text-lg font-bold text-primary">
      {#if loading}&mdash;{:else}{summary?.total_non_dummy ?? 0}{/if}
    </p>
  </div>

  <!-- Active Decoders -->
  <div
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
      Active Decoders
    </p>
    <p class="mt-1 text-lg font-bold text-primary">
      {#if loading}&mdash;{:else}{summary?.digital_count ?? 0}{/if}
    </p>
  </div>
</div>
