<script lang="ts">
  import type { OwnedRollingStockView } from '$lib/bindings';
  import { ChevronDown, ChevronUp } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    rollingStock: OwnedRollingStockView;
  }

  let { rollingStock }: Props = $props();
  let isExpanded = $state(false);

  function toggleExpand() {
    isExpanded = !isExpanded;
  }

  function formatSeriesRoadNumber() {
    const series = rollingStock.series || m.model_rolling_stock_unknown_series();
    const roadNumber = rollingStock.roadNumber || m.model_rolling_stock_na();
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
        <p class="mb-4 text-muted-foreground">{rollingStock.notes}</p>
      {/if}

      <dl class="grid grid-cols-1 gap-x-4 gap-y-3 sm:grid-cols-2">
        {#if rollingStock.series}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_series()}
            </dt>
            <dd class="mt-1 text-sm">{rollingStock.series}</dd>
          </div>
        {/if}

        {#if rollingStock.roadNumber}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_road_number()}
            </dt>
            <dd class="mt-1 text-sm">{rollingStock.roadNumber}</dd>
          </div>
        {/if}

        {#if rollingStock.livery}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_livery()}
            </dt>
            <dd class="mt-1 text-sm">{rollingStock.livery}</dd>
          </div>
        {/if}

        {#if rollingStock.railwayCompanyName}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_company()}
            </dt>
            <dd class="mt-1 text-sm">{rollingStock.railwayCompanyName}</dd>
          </div>
        {/if}

        {#if rollingStock.control}
          <div>
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_control()}
            </dt>
            <dd class="mt-1 text-sm">{rollingStock.control}</dd>
          </div>
        {/if}

        {#if rollingStock.digital}
          <div class="sm:col-span-2">
            <dt class="text-sm font-medium text-muted-foreground">
              {m.model_rolling_stock_field_digital_setup()}
            </dt>
            <dd class="mt-1 text-sm">
              {m.model_rolling_stock_digital_interface()}: {rollingStock.digital.interface}
              | {m.model_rolling_stock_digital_address()}: {rollingStock.digital.dcc_address}
              {#if rollingStock.digital.installed_decoder_id}
                | {m.model_rolling_stock_digital_decoder_id()}: {rollingStock.digital
                  .installed_decoder_id}
              {/if}
            </dd>
          </div>
        {/if}
      </dl>
    </div>
  {/if}
</div>
