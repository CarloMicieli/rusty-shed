<script lang="ts">
  import { safeInvoke } from '$lib/shared/services/TauriAdapter';
  import type { OwnedRollingStockView, CollectionView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  let { selectedId = $bindable(), onSelect } = $props<{
    selectedId: string | null;
    onSelect?: (id: string | null) => void;
  }>();

  let rollingStocks = $state<OwnedRollingStockView[]>([]);
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  // Load rolling stocks on mount
  $effect(() => {
    loadRollingStocks();
  });

  async function loadRollingStocks() {
    isLoading = true;
    error = null;

    try {
      const result = await safeInvoke<CollectionView>('get_collection');
      if (!result.ok) {
        throw new Error(result.error.message);
      }
      // Flatten all owned rolling stocks from all collection items
      rollingStocks = result.data.items.flatMap((item) => item.rollingStocks);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load rolling stocks';
      console.error('[RollingStockSelector] Load error:', err);
    } finally {
      isLoading = false;
    }
  }

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const value = target.value || null;
    selectedId = value;
    onSelect?.(value);
  }

  // Format display text for rolling stock
  function formatRollingStock(rs: OwnedRollingStockView): string {
    const parts = [rs.railwayCompanyName, rs.series, rs.roadNumber].filter(Boolean);
    return parts.join(' - ') || rs.id;
  }
</script>

<div class="space-y-2">
  <label
    for="rolling-stock-select"
    class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.maintenance_create_card_select_rolling_stock()}
  </label>
  {#if isLoading}
    <div class="flex h-10 items-center px-3 text-sm text-zinc-500">
      <span
        class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-zinc-500 border-t-transparent"
      ></span>
      {m.app_loading()}
    </div>
  {:else if error}
    <p class="text-xs font-medium text-red-400">{error}</p>
  {:else}
    <select
      id="rolling-stock-select"
      class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      value={selectedId ?? ''}
      onchange={handleChange}
    >
      <option value="" disabled>{m.maintenance_create_card_placeholder()}</option>
      {#each rollingStocks as rs (rs.id)}
        <option value={rs.id} class="bg-zinc-950 text-zinc-100">
          {formatRollingStock(rs)}
        </option>
      {/each}
    </select>
  {/if}
</div>
