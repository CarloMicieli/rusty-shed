<script lang="ts">
  import type { Stat } from '$lib/data/mock';
  import { TrendingUp, TrendingDown, Minus } from 'lucide-svelte';

  let { stat } = $props<{ stat: Stat }>();
</script>

<div
  class="variant-filled-surface hover:variant-filled-secondary card space-y-2 p-4 transition-colors duration-200"
>
  <div class="text-surface-400 flex items-center justify-between">
    <span class="text-sm font-bold tracking-widest uppercase">{stat.label}</span>
    {#if stat.trend === 'up'}
      <TrendingUp size={16} class="text-success-500" />
    {:else if stat.trend === 'down'}
      <TrendingDown size={16} class="text-error-500" />
    {:else}
      <Minus size={16} class="text-surface-400" />
    {/if}
  </div>
  <div class="flex items-end gap-2">
    <h3 class="h3 text-primary-500 font-bold">{stat.value}</h3>
    <span
      class="{stat.trend === 'up'
        ? 'text-success-500'
        : stat.trend === 'down'
          ? 'text-error-500'
          : 'text-surface-400'} text-sm font-medium"
    >
      {stat.trendValue}
    </span>
  </div>
</div>
