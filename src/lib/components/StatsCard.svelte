<script lang="ts">
  import type { Stat } from '$lib/data/mock';
  import { TrendingUp, TrendingDown, Minus, AlertCircle } from 'lucide-svelte';

  let { stat } = $props<{ stat: Stat }>();

  // Check if this is a maintenance alert card
  const isMaintenanceAlert = $derived(stat.label.toLowerCase().includes('maintenance'));
  const hasAlert = $derived(isMaintenanceAlert && stat.trend === 'down');
</script>

<div
  class={[
    'card gauge-frame space-y-3 p-5 text-foreground transition-all duration-200',
    hasAlert ? 'ring-error-500/40 ring-2' : 'ring-1 ring-border/40'
  ]}
>
  <div class="text-surface-400 flex items-center justify-between">
    <span class="text-xs font-bold tracking-[0.2em] uppercase">{stat.label}</span>
    {#if hasAlert}
      <AlertCircle size={18} class="text-error-400 animate-pulse" />
    {:else if stat.trend === 'up'}
      <TrendingUp size={16} class="text-success-500" />
    {:else if stat.trend === 'down'}
      <TrendingDown size={16} class="text-error-500" />
    {:else}
      <Minus size={16} class="text-surface-400" />
    {/if}
  </div>
  <div class="flex items-end gap-2">
    <h3 class="h2 font-bold text-primary">{stat.value}</h3>
    <span
      class="{stat.trend === 'up'
        ? 'text-success-500'
        : stat.trend === 'down'
          ? 'text-error-400'
          : 'text-surface-400'} mb-1 text-sm font-semibold"
    >
      {stat.trendValue}
    </span>
  </div>
</div>
