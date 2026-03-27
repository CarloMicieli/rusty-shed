<script lang="ts">
  import { Box, Bus, Package, TrainFront, Zap, Wallet } from 'lucide-svelte';
  import type { IconComponent } from '$lib/config/icons';
  import * as m from '$lib/paraglide/messages.js';
  import type { CollectionSummary } from '$lib/bindings';
  import GaugeStatCard from '$lib/components/GaugeStatCard.svelte';

  interface Props {
    summary: CollectionSummary;
    totalValue: string;
  }

  const { summary, totalValue }: Props = $props();

  const totalUnits = $derived(
    summary.locomotivesCount +
      summary.passengerCarsCount +
      summary.freightCarsCount +
      summary.trainSetsCount +
      summary.railcarsCount +
      summary.electricMultipleUnitsCount
  );

  const categoryStats: { label: () => string; value: () => number; icon: IconComponent }[] = [
    {
      label: m.enum_category_locomotives,
      value: () => summary.locomotivesCount,
      icon: TrainFront
    },
    {
      label: m.enum_category_passenger_cars,
      value: () => summary.passengerCarsCount,
      icon: TrainFront
    },
    {
      label: m.enum_category_freight_cars,
      value: () => summary.freightCarsCount,
      icon: Box
    },
    {
      label: m.enum_category_train_sets,
      value: () => summary.trainSetsCount,
      icon: Package
    },
    { label: m.enum_category_railcars, value: () => summary.railcarsCount, icon: Bus },
    {
      label: m.enum_category_electric_multiple_units,
      value: () => summary.electricMultipleUnitsCount,
      icon: Zap
    }
  ];
</script>

<section class="space-y-4">
  <!-- Top summary row -->
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
    <GaugeStatCard label={m.stats_total_collection_value()} value={totalValue} icon={Wallet} />
    <GaugeStatCard label={m.stats_rolling_stocks()} value={totalUnits} icon={TrainFront} />
  </div>

  <!-- Category breakdown -->
  <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
    {#each categoryStats as { label, value, icon } (label())}
      <GaugeStatCard label={label()} value={value()} {icon} />
    {/each}
  </div>
</section>
