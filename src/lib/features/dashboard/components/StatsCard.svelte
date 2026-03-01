<script lang="ts">
  import type { Stat } from '$lib/data/mock';
  import { TrendingUp, TrendingDown, Minus } from 'lucide-svelte';
  import { Card, CardContent } from '$lib/components';

  let { stat } = $props<{ stat: Stat }>();
</script>

<Card class="bg-card/90 transition-colors duration-200 hover:bg-muted/30">
  <CardContent class="space-y-2 p-4">
    <div class="flex items-center justify-between text-muted-foreground">
      <span class="text-sm font-bold tracking-widest uppercase">{stat.label}</span>
      {#if stat.trend === 'up'}
        <TrendingUp size={16} class="text-success-500" />
      {:else if stat.trend === 'down'}
        <TrendingDown size={16} class="text-error-500" />
      {:else}
        <Minus size={16} class="text-muted-foreground" />
      {/if}
    </div>
    <div class="flex items-end gap-2">
      <h3 class="h3 text-primary-500 font-bold">{stat.value}</h3>
      <span
        class="{stat.trend === 'up'
          ? 'text-success-500'
          : stat.trend === 'down'
            ? 'text-error-500'
            : 'text-muted-foreground'} text-sm font-medium"
      >
        {stat.trendValue}
      </span>
    </div>
  </CardContent>
</Card>
