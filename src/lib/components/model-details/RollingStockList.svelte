<script lang="ts">
  import type { OwnedRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import RollingStockCard from './RollingStockCard.svelte';

  interface Props {
    rollingStocks?: OwnedRollingStockView[];
  }

  let { rollingStocks }: Props = $props();
</script>

{#if rollingStocks && rollingStocks.length > 0}
  <div class="space-y-4">
    {#each rollingStocks as rollingStock (rollingStock.id)}
      <RollingStockCard {rollingStock} />
    {/each}
  </div>
{:else}
  <div class="rounded-lg border border-dashed border-border p-8 text-center">
    <p class="text-muted-foreground">{m.model_no_rolling_stock()}</p>
  </div>
{/if}
