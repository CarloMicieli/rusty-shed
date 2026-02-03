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
  <label for="rolling-stock-select" class="label">
    {m.maintenance_create_card_select_rolling_stock()}
  </label>
  {#if isLoading}
    <p class="text-sm text-surface-400">{m.app_loading()}</p>
  {:else if error}
    <p class="text-sm text-error-500">{error}</p>
  {:else}
    <select
      id="rolling-stock-select"
      class="select"
      value={selectedId ?? ''}
      onchange={handleChange}
    >
      <option value="">{m.maintenance_create_card_placeholder()}</option>
      {#each rollingStocks as rs (rs.id)}
        <option value={rs.id}>
          {formatRollingStock(rs)}
        </option>
      {/each}
    </select>
  {/if}
</div>
