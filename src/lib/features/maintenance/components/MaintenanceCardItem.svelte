<script lang="ts">
  import { Calendar, AlertTriangle, CheckCircle2 } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getUrgencyLevel } from '../utils/urgency';
  import type { MaintenanceCardView } from '$lib/bindings';

  let { card } = $props<{ card: MaintenanceCardView }>();

  // Calculate urgency level
  const urgency = $derived(getUrgencyLevel(card.nextMaintenanceDate));

  // Calculate "Maintenance Health" based on time elapsed since last maintenance vs total interval
  const healthPercentage = $derived.by(() => {
    if (!card.lastMaintenanceDate || !card.nextMaintenanceDate) return 100;

    const last = new Date(card.lastMaintenanceDate).getTime();
    const next = new Date(card.nextMaintenanceDate).getTime();
    const now = new Date().getTime();

    if (now < last) return 100; // Should not happen but safety check
    if (now > next) return 0; // Overdue

    const totalDuration = next - last;
    const elapsed = now - last;

    // Health starts at 100% and goes down to 0% as we approach next date
    return Math.max(0, Math.min(100, 100 - (elapsed / totalDuration) * 100));
  });

  const healthColor = $derived.by(() => {
    if (healthPercentage > 66) return 'bg-emerald-500';
    if (healthPercentage > 33) return 'bg-amber-500';
    return 'bg-red-500'; // Critical/Low health
  });

  // Format dates for display
  const formatDate = (dateStr: string | null): string => {
    if (!dateStr) return 'N/A';
    const date = new Date(dateStr);
    return date
      .toLocaleDateString(undefined, { year: 'numeric', month: '2-digit', day: '2-digit' })
      .replace(/\//g, '.'); // Monospaced friendly format
  };

  const daysRemaining = $derived.by(() => {
    if (!card.nextMaintenanceDate) return null;
    const next = new Date(card.nextMaintenanceDate).getTime();
    const now = new Date().getTime();
    const diff = Math.ceil((next - now) / (1000 * 60 * 60 * 24));
    return diff;
  });
</script>

<a
  href="/maintenance/{encodeURIComponent(card.id)}"
  class="group relative block overflow-hidden rounded-xl border border-white/10 bg-[#0c0c0c] transition-all hover:border-amber-500/50 hover:shadow-[0_0_20px_rgba(245,158,11,0.1)]"
>
  <!-- Status Stripe -->
  <div
    class="absolute top-0 bottom-0 left-0 w-1 {urgency === 'overdue'
      ? 'bg-red-500'
      : urgency === 'warning'
        ? 'bg-amber-500'
        : 'bg-emerald-500'}"
  ></div>

  <div class="space-y-4 p-5 pl-7">
    <!-- Header -->
    <div class="flex items-start justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-2">
          <span class="bold text-[10px] tracking-widest text-zinc-500 uppercase">Rolling Stock</span
          >
          {#if urgency === 'overdue'}
            <AlertTriangle class="h-3 w-3 animate-pulse text-red-500" />
          {/if}
        </div>
        <h3 class="text-lg leading-tight font-bold tracking-tight text-white">
          {card.displayInfo?.manufacturerName ?? '—'}{card.displayInfo?.productCode
            ? ` ${card.displayInfo.productCode}`
            : ''}
        </h3>
        {#if card.displayInfo?.seriesCode}
          <div class="text-[10px] font-bold tracking-widest text-zinc-500 uppercase">
            {card.displayInfo.seriesCode}
          </div>
        {/if}
      </div>

      <!-- Days Remaining Badge + Road Number -->
      <div class="flex flex-col items-end gap-1">
        {#if card.displayInfo?.roadNumber}
          <span
            class="rounded border border-amber-500/20 bg-amber-500/10 px-2 py-0.5 font-mono text-xs font-bold text-amber-400"
          >
            {card.displayInfo.roadNumber}
          </span>
        {/if}
        {#if daysRemaining !== null}
          <div class="flex flex-col items-end">
            <span
              class="font-mono text-2xl font-bold {urgency === 'overdue'
                ? 'text-red-500'
                : urgency === 'warning'
                  ? 'text-amber-500'
                  : 'text-white'}"
            >
              {daysRemaining > 0 ? daysRemaining : Math.abs(daysRemaining)}
            </span>
            <span class="bold text-[10px] tracking-widest text-zinc-500 uppercase">
              {daysRemaining >= 0 ? 'Days Left' : 'Days Overdue'}
            </span>
          </div>
        {/if}
      </div>
    </div>

    <!-- Health Bar -->
    <div class="space-y-2">
      <div class="flex items-end justify-between">
        <span class="bold text-[10px] tracking-widest text-zinc-500 uppercase"
          >Maintenance Health</span
        >
        <span class="font-mono text-xs text-zinc-400">{Math.round(healthPercentage)}%</span>
      </div>
      <div class="h-1.5 w-full overflow-hidden rounded-full bg-white/5">
        <div
          class="h-full rounded-full transition-all duration-500 ease-out {healthColor}"
          style="width: {healthPercentage}%"
        ></div>
      </div>
    </div>

    <!-- Footer Stats -->
    <div class="grid grid-cols-2 gap-4 border-t border-white/5 pt-2">
      <!-- Last Service -->
      <div>
        <div class="bold mb-1 text-[10px] tracking-widest text-zinc-500 uppercase">
          {m.maintenance_card_last_maintenance()}
        </div>
        <div class="flex items-center gap-2 font-mono text-sm text-zinc-300">
          <CheckCircle2 class="h-3.5 w-3.5 text-zinc-600" />
          {formatDate(card.lastMaintenanceDate)}
        </div>
      </div>

      <!-- Next Inspection -->
      <div>
        <div class="bold mb-1 text-[10px] tracking-widest text-zinc-500 uppercase">
          {m.maintenance_card_due_date()}
        </div>
        <div
          class="flex items-center gap-2 font-mono text-sm {urgency === 'overdue'
            ? 'text-red-400'
            : 'text-zinc-300'}"
        >
          <Calendar
            class="h-3.5 w-3.5 {urgency === 'overdue' ? 'text-red-500' : 'text-zinc-600'}"
          />
          {formatDate(card.nextMaintenanceDate)}
        </div>
      </div>
    </div>
  </div>
</a>
