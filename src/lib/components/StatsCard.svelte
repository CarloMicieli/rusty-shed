<script lang="ts">
  import type { Stat } from '$lib/data/mock';
  import { TrendingUp, TrendingDown, Minus, AlertCircle } from 'lucide-svelte';

  let { stat } = $props<{ stat: Stat }>();

  // Check if this is a maintenance alert card
  const isMaintenanceAlert = $derived(stat.label.toLowerCase().includes('maintenance'));
  const hasAlert = $derived(isMaintenanceAlert && stat.trend === 'down');
</script>

<div
  class="{hasAlert
    ? 'variant-soft-error border-error-500/30'
    : 'variant-filled-surface hover:variant-filled-secondary'} space-y-2 card border p-5 shadow-sm transition-all duration-200 hover:shadow-md"
>
  <div class="flex items-center justify-between text-surface-400">
    <span class="text-sm font-bold tracking-widest uppercase">{stat.label}</span>
    {#if hasAlert}
      <AlertCircle size={18} class="animate-pulse text-error-400" />
    {:else if stat.trend === 'up'}
      <TrendingUp size={16} class="text-success-500" />
    {:else if stat.trend === 'down'}
      <TrendingDown size={16} class="text-error-500" />
    {:else}
      <Minus size={16} class="text-surface-400" />
    {/if}
  </div>
  <div class="flex items-end gap-2">
    <h3 class="h2 font-bold text-primary-400">{stat.value}</h3>
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
