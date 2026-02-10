<script lang="ts">
  import type { DigitalSummary } from '$lib/bindings';

  interface Props {
    summary: DigitalSummary | null;
    loading?: boolean;
  }

  let { summary = null, loading = false }: Props = $props();

  const formattedPercentage = $derived(summary ? summary.percentage.toFixed(1) : '0.0');
</script>

{#if loading}
  <div class="flex items-center justify-center py-8">
    <div class="border-primary-500 h-8 w-8 animate-spin rounded-full border-b-2"></div>
  </div>
{:else if summary}
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    <!-- Digitalization Rate Card -->
    <div
      class="card gauge-frame space-y-3 p-5 text-foreground ring-1 ring-border/40 transition-all duration-200"
    >
      <div class="text-surface-400 flex items-center justify-between">
        <span class="text-xs font-bold tracking-[0.2em] uppercase">Digitalization Rate</span>
      </div>
      <div class="flex items-end gap-2">
        <h3 class="h2 font-bold text-orange-500">{formattedPercentage}%</h3>
      </div>
    </div>

    <!-- Total Fleet Card -->
    <div
      class="card gauge-frame space-y-3 p-5 text-foreground ring-1 ring-border/40 transition-all duration-200"
    >
      <div class="text-surface-400 flex items-center justify-between">
        <span class="text-xs font-bold tracking-[0.2em] uppercase">Total Fleet</span>
      </div>
      <div class="flex items-end gap-2">
        <h3 class="h2 font-bold text-primary">{summary.total_non_dummy}</h3>
      </div>
    </div>

    <!-- Active Decoders Card -->
    <div
      class="card gauge-frame space-y-3 p-5 text-foreground ring-1 ring-border/40 transition-all duration-200"
    >
      <div class="text-surface-400 flex items-center justify-between">
        <span class="text-xs font-bold tracking-[0.2em] uppercase">Active Decoders</span>
      </div>
      <div class="flex items-end gap-2">
        <h3 class="h2 font-bold text-green-500">{summary.digital_count}</h3>
      </div>
    </div>
  </div>
{/if}
