<script lang="ts">
  import type { OwnedRollingStockView, RailwayModelId, RollingStockId } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import RollingStockCard from './RollingStockCard.svelte';
  import RollingStockCreateDrawer from '$lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte';
  import { setEditContext } from './editContext.svelte';

  interface Props {
    rollingStocks?: OwnedRollingStockView[];
    railwayModelId: RailwayModelId;
    editable?: boolean;
    onRollingStockAdded?: () => void;
  }

  let { rollingStocks, railwayModelId, editable = false, onRollingStockAdded }: Props = $props();

  let createDrawerOpen = $state(false);
  setEditContext();
</script>

{#if rollingStocks && rollingStocks.length > 0}
  <div class="space-y-4">
    {#each rollingStocks as rollingStock (rollingStock.id)}
      <RollingStockCard {rollingStock} {railwayModelId} {editable} />
    {/each}
  </div>

  {#if editable}
    <div class="mt-4 flex justify-start">
      <button
        type="button"
        class="inline-flex items-center gap-1.5 rounded-md border border-layout-border bg-transparent px-3 py-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase transition-colors hover:bg-primary/15 hover:text-primary"
        onclick={() => {
          createDrawerOpen = true;
        }}
      >
        {m.rolling_stock_add_more()}
      </button>
    </div>
  {/if}
{:else if editable}
  <div class="rounded-lg border border-dashed border-border p-8 text-center">
    <button
      type="button"
      class="inline-flex items-center gap-2 rounded-md border border-layout-border bg-transparent px-4 py-2 text-[10px] font-bold tracking-wider text-muted-foreground uppercase transition-colors hover:bg-primary/15 hover:text-primary"
      onclick={() => {
        createDrawerOpen = true;
      }}
    >
      {m.rolling_stock_add_cta()}
    </button>
  </div>
{:else}
  <div class="rounded-lg border border-dashed border-border p-8 text-center">
    <p class="text-muted-foreground">{m.model_no_rolling_stock()}</p>
  </div>
{/if}

<RollingStockCreateDrawer
  open={createDrawerOpen}
  {railwayModelId}
  onCreated={(_id: RollingStockId) => {
    onRollingStockAdded?.();
  }}
  onClose={() => {
    createDrawerOpen = false;
  }}
/>
