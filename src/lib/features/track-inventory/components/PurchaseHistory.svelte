<script lang="ts">
  import type { TrackPurchaseView } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages';
  import PurchaseHistoryItem from './PurchaseHistoryItem.svelte';

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

    // Sort groups by date (newest first) and sort purchases within each group
    return Object.entries(groups)
      .sort(([a], [b]) => b.localeCompare(a))
      .map(([key, items]) => ({
        monthKey: key,
        displayMonth: new Date(key + '-01').toLocaleDateString(undefined, {
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
    <div class="variant-ghost-surface rounded-lg p-8 text-center">
      <p class="text-surface-400">{m.track_purchase_history_empty()}</p>
    </div>
  {:else}
    {#each groupedPurchases as group (group.monthKey)}
      <div class="space-y-3">
        <h4 class="text-sm font-semibold tracking-wider text-surface-400 uppercase">
          {group.displayMonth}
        </h4>
        <div class="space-y-2">
          {#each group.purchases as purchase (purchase.id)}
            <PurchaseHistoryItem {purchase} />
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>
