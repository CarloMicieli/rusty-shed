<script lang="ts">
  import type { OwnedRollingStockView } from '$lib/bindings';
  import { ChevronDown, ChevronUp } from 'lucide-svelte';

  interface Props {
    rollingStock: OwnedRollingStockView;
  }

  let { rollingStock }: Props = $props();
  let isExpanded = $state(false);

  function toggleExpand() {
    isExpanded = !isExpanded;
  }

  function formatSeriesRoadNumber() {
    const series = rollingStock.series || 'Unknown';
    const roadNumber = rollingStock.roadNumber || 'N/A';
    return `${series} — ${roadNumber}`;
  }
</script>

<div class="rounded-lg border border-border transition-shadow hover:shadow-md">
  <!-- Card Header (Always Visible) -->
  <button
    type="button"
    class="flex w-full items-center justify-between p-4 text-left transition-colors hover:bg-muted/50"
    onclick={toggleExpand}
    aria-expanded={isExpanded}
  >
    <h3 class="text-lg font-semibold">
      {formatSeriesRoadNumber()}
    </h3>
    <div class="ml-4 flex-shrink-0">
      {#if isExpanded}
        <ChevronUp class="h-5 w-5 text-muted-foreground" />
      {:else}
        <ChevronDown class="h-5 w-5 text-muted-foreground" />
      {/if}
    </div>
  </button>

  <!-- Card Body (Expandable) -->
  {#if isExpanded}
    <div class="border-t border-border p-4">
      {#if rollingStock.notes}
        <p class="text-muted-foreground mb-4">{rollingStock.notes}</p>
      {/if}

      <dl class="grid grid-cols-1 gap-x-4 gap-y-3 sm:grid-cols-2">
        {#if rollingStock.series}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">Series</dt>
            <dd class="mt-1 text-sm">{rollingStock.series}</dd>
          </div>
        {/if}

        {#if rollingStock.roadNumber}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">Road Number</dt>
            <dd class="mt-1 text-sm">{rollingStock.roadNumber}</dd>
          </div>
        {/if}

        {#if rollingStock.livery}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">Livery</dt>
            <dd class="mt-1 text-sm">{rollingStock.livery}</dd>
          </div>
        {/if}

        {#if rollingStock.railwayCompanyName}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">Company</dt>
            <dd class="mt-1 text-sm">{rollingStock.railwayCompanyName}</dd>
          </div>
        {/if}

        {#if rollingStock.control}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">Control</dt>
            <dd class="mt-1 text-sm">{rollingStock.control}</dd>
          </div>
        {/if}

        {#if rollingStock.digital}
          <div class="sm:col-span-2">
            <dt class="text-sm font-medium text-muted-foreground">Digital Setup</dt>
            <dd class="mt-1 text-sm">
              Interface: {rollingStock.digital.interface}
              | Address: {rollingStock.digital.dcc_address}
              {#if rollingStock.digital.installed_decoder_id}
                | Decoder ID: {rollingStock.digital.installed_decoder_id}
              {/if}
            </dd>
          </div>
        {/if}
      </dl>
    </div>
  {/if}
</div>
