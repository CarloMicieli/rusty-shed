<script lang="ts">
  import type { IconComponent } from '$lib/config/icons';

  interface Props {
    label: string;
    value: string | number;
    icon: IconComponent;
    unit?: string;
    iconBgClass?: string;
    iconColorClass?: string;
    alert?: boolean;
  }

  const {
    label,
    value,
    icon: Icon,
    unit,
    iconBgClass = 'bg-primary/10',
    iconColorClass = 'text-primary',
    alert = false
  }: Props = $props();
</script>

<div
  class={[
    'flex flex-col gap-3 rounded-[8px] border border-border bg-card p-4 transition-all duration-200',
    alert ? 'ring-error-500/40 ring-2' : 'ring-1 ring-border/40'
  ]}
>
  <div class="flex items-center justify-between">
    <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
      >{label}</span
    >
    <div class={['rounded-lg p-2', iconBgClass]}>
      <Icon size={16} class={`stroke-2 ${iconColorClass}`} />
    </div>
  </div>
  <div class="flex items-baseline gap-1">
    <span class="font-mono text-2xl font-bold text-primary">{value}</span>
    {#if unit}
      <span class="font-mono text-sm text-muted-foreground">{unit}</span>
    {/if}
  </div>
</div>
