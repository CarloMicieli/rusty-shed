<script lang="ts">
  import type { TrackPurchaseView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import PurchaseHistoryItem from './PurchaseHistoryItem.svelte';
  import { History } from 'lucide-svelte';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    purchases: TrackPurchaseView[];
  }

  const { purchases }: Props = $props();

  // Group purchases by month for better chronological display
  const groupedPurchases = $derived.by(() => {
    const groups: Record<string, TrackPurchaseView[]> = {};

    for (const purchase of purchases) {
      const date = new Date(purchase.purchase_date);
      const monthKey = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`;

      if (!groups[monthKey]) {
        groups[monthKey] = [];
      }
      groups[monthKey].push(purchase);
    }

    return Object.entries(groups)
      .sort(([a], [b]) => b.localeCompare(a))
      .map(([key, items]) => ({
        monthKey: key,
        displayMonth: new Date(key + '-01').toLocaleDateString(regionalManager.locale, {
          year: 'numeric',
          month: 'long'
        }),
        purchases: items.sort(
          (a, b) => new Date(b.purchase_date).getTime() - new Date(a.purchase_date).getTime()
        )
      }));
  });
</script>

<div class="space-y-6">
  {#if purchases.length === 0}
    <div
      class="flex flex-col items-center justify-center rounded-sm border border-dashed border-border bg-background/50 py-16 text-center"
    >
      <div class="relative mb-4">
        <div class="absolute inset-0 scale-150 rounded-full bg-muted blur-3xl"></div>
        <History size={40} class="relative text-muted-foreground/30" />
      </div>
      <p class="text-xs tracking-wider text-muted-foreground uppercase">
        {m.track_purchase_history_empty()}
      </p>
    </div>
  {:else}
    {#each groupedPurchases as group (group.monthKey)}
      <div class="space-y-4">
        <div class="flex items-center gap-3">
          <div class="flex-1 border-t border-dashed border-border"></div>
          <h4 class="text-xs tracking-wider text-muted-foreground uppercase">
            {group.displayMonth}
          </h4>
          <div class="flex-1 border-t border-dashed border-border"></div>
        </div>

        <div class="space-y-2">
          {#each group.purchases as purchase (purchase.id)}
            <PurchaseHistoryItem {purchase} />
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>
