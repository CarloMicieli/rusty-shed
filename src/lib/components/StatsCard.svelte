<script lang="ts">
  import type { Stat } from '$lib/data/mock';
  import type { IconComponent } from '$lib/config/icons';
  import { AlertCircle, Activity } from 'lucide-svelte';
  import GaugeStatCard from './GaugeStatCard.svelte';

  let { stat, icon }: { stat: Stat; icon?: IconComponent } = $props();

  const isMaintenanceAlert = $derived(stat.label.toLowerCase().includes('maintenance'));
  const hasAlert = $derived(isMaintenanceAlert && stat.trend === 'down');
</script>

<GaugeStatCard
  label={stat.label}
  value={stat.value}
  icon={hasAlert ? AlertCircle : (icon ?? Activity)}
  iconBgClass={hasAlert ? 'bg-error-500/10' : 'bg-primary/10'}
  iconColorClass={hasAlert ? 'text-error-400 animate-pulse' : 'text-primary'}
  alert={hasAlert}
/>
