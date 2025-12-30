<script lang="ts">
  import { Box, Bus, Package, TrainFront, Zap } from 'lucide-svelte';
  import type { CollectionSummary } from '$lib/bindings';

  type IconComponent = typeof TrainFront;

  const { summary, totalValue } = $props<{
    summary: CollectionSummary;
    totalValue: string;
  }>();

  const totalUnits = $derived(
    summary.locomotives_count +
      summary.passenger_cars_count +
      summary.freight_cars_count +
      summary.train_sets_count +
      summary.railcars_count +
      summary.electric_multiple_units_count
  );
</script>

{#snippet StatCard(label: string, value: number | string, Icon: IconComponent, accentClass: string)}
  {@const iconClass = accentClass.replace('border-', 'text-').replace('500', '300')}
  <div
    class={`variant-soft-surface flex items-center justify-between gap-3 card border-l-4 p-4 ${accentClass}`}
  >
    <div class={`rounded-lg bg-surface-800/60 p-3 ${iconClass}`}>
      <Icon size={20} />
    </div>
    <div class="text-right">
      <p class="text-xs font-semibold tracking-wide text-surface-400 uppercase">{label}</p>
      <p class="text-xl font-bold text-surface-100">{value}</p>
    </div>
  </div>
{/snippet}

<section class="space-y-4">
  <div
    class="variant-glass-surface flex flex-col gap-4 card p-4 sm:flex-row sm:items-center sm:justify-between"
  >
    <div class="space-y-1">
      <p class="text-xs font-semibold tracking-[0.18em] text-surface-400 uppercase">
        Collection value
      </p>
      <p class="h3 font-bold text-primary-100">{totalValue}</p>
    </div>
    <div
      class="flex items-center gap-3 rounded-xl border border-primary-500/40 bg-primary-500/10 px-4 py-3"
    >
      <div class="rounded-full bg-primary-500/20 p-3 text-primary-200">
        <TrainFront size={22} />
      </div>
      <div>
        <p class="text-xs font-semibold tracking-[0.12em] text-surface-400 uppercase">
          Total units
        </p>
        <p class="text-2xl font-bold text-surface-50">{totalUnits}</p>
      </div>
    </div>
  </div>

  <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
    {@render StatCard('Locomotives', summary.locomotives_count, TrainFront, 'border-primary-500')}
    {@render StatCard(
      'Passenger cars',
      summary.passenger_cars_count,
      TrainFront,
      'border-info-500'
    )}
    {@render StatCard('Freight cars', summary.freight_cars_count, Box, 'border-warning-500')}
    {@render StatCard('Train sets', summary.train_sets_count, Package, 'border-secondary-500')}
    {@render StatCard('Railcars', summary.railcars_count, Bus, 'border-success-500')}
    {@render StatCard('EMU', summary.electric_multiple_units_count, Zap, 'border-accent-500')}
  </div>
</section>
