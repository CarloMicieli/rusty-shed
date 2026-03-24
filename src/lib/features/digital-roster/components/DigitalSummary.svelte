<script lang="ts">
  import { Activity, TrainFront, Zap } from 'lucide-svelte';
  import type { DigitalSummary } from '$lib/bindings';
  import GaugeStatCard from '$lib/components/GaugeStatCard.svelte';

  interface Props {
    summary: DigitalSummary | null;
    loading?: boolean;
  }

  let { summary = null, loading = false }: Props = $props();

  const formattedPercentage = $derived(summary ? summary.percentage.toFixed(1) + '%' : '—');
  const totalFleet = $derived(loading ? '—' : String(summary?.total_non_dummy ?? 0));
  const activeDecoders = $derived(loading ? '—' : String(summary?.digital_count ?? 0));
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
  <GaugeStatCard label="Digitalization Rate" value={formattedPercentage} icon={Activity} />
  <GaugeStatCard label="Total Fleet" value={totalFleet} icon={TrainFront} />
  <GaugeStatCard label="Active Decoders" value={activeDecoders} icon={Zap} />
</div>
