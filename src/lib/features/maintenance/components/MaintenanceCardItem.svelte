<script lang="ts">
  import { Calendar, Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import { getUrgencyLevel, getUrgencyColors } from '../utils/urgency';
  import type { MaintenanceCardView } from '$lib/bindings';

  let { card } = $props<{ card: MaintenanceCardView }>();

  // Calculate urgency level
  const urgency = $derived(getUrgencyLevel(card.nextMaintenanceDate));
  const colors = $derived(getUrgencyColors(urgency));

  // Format dates for display
  const formatDate = (dateStr: string | null): string => {
    if (!dateStr) return m.maintenance_card_no_last_maintenance();
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
  };

  // Get urgency label
  const getUrgencyLabel = (level: 'overdue' | 'warning' | 'normal'): string => {
    switch (level) {
      case 'overdue':
        return m.maintenance_urgency_overdue();
      case 'warning':
        return m.maintenance_urgency_warning();
      case 'normal':
        return m.maintenance_urgency_normal();
    }
  };
</script>

<div class="card border {colors.border} space-y-3 p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <Badge variant="default" class="flex items-center gap-1">
        <Wrench size={14} />
      </Badge>
      <div class="space-y-1">
        <p class="text-surface-500 text-xs tracking-wide uppercase">Rolling Stock</p>
        <h3 class="text-base leading-tight font-semibold">
          {card.ownedRollingStockId}
        </h3>
      </div>
    </div>
    <span class="badge {colors.bg} {colors.text} text-xs font-semibold">
      {getUrgencyLabel(urgency)}
    </span>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    {#if card.nextMaintenanceDate}
      <Badge variant="default" class="flex items-center gap-1">
        <Calendar size={14} />
        {m.maintenance_card_due_date()}: {formatDate(card.nextMaintenanceDate)}
      </Badge>
    {/if}
    {#if card.lastMaintenanceDate}
      <Badge variant="outline">
        {m.maintenance_card_last_maintenance()}: {formatDate(card.lastMaintenanceDate)}
      </Badge>
    {:else}
      <Badge variant="outline" class="opacity-60">
        {m.maintenance_card_no_last_maintenance()}
      </Badge>
    {/if}
  </div>
</div>
