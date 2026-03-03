<script lang="ts">
  import type { OwnedRollingStockView, RailwayCompanyId, RailwayModelId } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { onMount } from 'svelte';
  import { ChevronDown, ChevronUp } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Settings } from 'lucide-svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { toaster } from '$lib/toaster';

  interface Props {
    rollingStock: OwnedRollingStockView;
    /** Parent railway model id — required to save identification changes. */
    railwayModelId: RailwayModelId;
    /** When true, identification fields are editable in-place. */
    editable?: boolean;
  }

  let { rollingStock, railwayModelId, editable = false }: Props = $props();
  let isExpanded = $state(false);

  // Local copies of editable identification fields
  let localSeries = $state('');
  let localRoadNumber = $state('');
  let localLivery = $state('');
  let localRailwayCompanyName = $state('');
  let localDepot = $state('');
  $effect(() => {
    localSeries = rollingStock.series ?? '';
    localRoadNumber = rollingStock.roadNumber ?? '';
    localLivery = rollingStock.livery ?? '';
    localRailwayCompanyName = rollingStock.railwayCompanyName ?? '';
    localDepot = rollingStock.depot ?? '';
  });

  // Railway company options for the picker
  let companyOptions = $state<{ id: string; label: string }[]>([]);

  // Specs drawer state
  let specsDrawerOpen = $state(false);

  onMount(async () => {
    if (!editable) return;
    const result = await commands.getRailwayCompanies();
    if (result.status === 'ok') {
      companyOptions = result.data.map((c) => ({ id: c.id, label: c.name }));
    }
  });

  function toggleExpand() {
    isExpanded = !isExpanded;
  }

  function formatSeriesRoadNumber() {
    const series = localSeries || m.model_rolling_stock_unknown_series();
    const roadNumber = localRoadNumber || m.model_rolling_stock_na();
    return `${series} — ${roadNumber}`;
  }

  async function saveIdentificationField(
    field: 'series' | 'roadNumber' | 'livery' | 'depot',
    value: string
  ) {
    const seriesCode = field === 'series' ? value : localSeries;
    const roadNumber = field === 'roadNumber' ? value || null : localRoadNumber || null;
    const livery = field === 'livery' ? value || null : localLivery || null;
    const depot = field === 'depot' ? value || null : localDepot || null;

    const result = await commands.updateRollingStockIdentification({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      seriesCode,
      roadNumber,
      livery,
      depot
    });

    if (result.status === 'error') {
      throw new Error('Failed to save');
    }

    if (field === 'series') localSeries = value;
    else if (field === 'roadNumber') localRoadNumber = value;
    else if (field === 'livery') localLivery = value;
    else if (field === 'depot') localDepot = value;
  }

  async function saveRailwayCompany(id: string) {
    const result = await commands.updateRollingStockRailwayCompany({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      railwayCompanyId: id as RailwayCompanyId
    });

    if (result.status === 'error') {
      toaster.error(m.rolling_stock_field_railway_company());
      throw new Error('Failed to save railway company');
    }

    localRailwayCompanyName = companyOptions.find((c) => c.id === id)?.label ?? id;
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

      {#if editable}
        <div class="mb-3 flex justify-end">
          <button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-xs font-medium text-zinc-300 transition-colors hover:border-[#E2994F]/50 hover:text-[#E2994F]"
            onclick={() => {
              specsDrawerOpen = true;
            }}
          >
            <Settings size={12} />
            {m.rolling_stock_edit_specs_button()}
          </button>
        </div>
      {/if}

      <dl class="grid grid-cols-1 gap-x-4 gap-y-3 sm:grid-cols-2">
        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.model_rolling_stock_field_series()}
          </dt>
          <dd class="mt-1 text-sm">
            {#if editable}
              <InPlaceEdit
                value={localSeries}
                placeholder={m.rolling_stock_field_series_code()}
                onSave={(v) => saveIdentificationField('series', v)}
              />
            {:else}
              {localSeries || '—'}
            {/if}
          </dd>
        </div>

        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.model_rolling_stock_field_road_number()}
          </dt>
          <dd class="mt-1 text-sm">
            {#if editable}
              <InPlaceEdit
                value={localRoadNumber}
                placeholder={m.rolling_stock_field_road_number()}
                onSave={(v) => saveIdentificationField('roadNumber', v)}
              />
            {:else}
              {localRoadNumber || '—'}
            {/if}
          </dd>
        </div>

        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.model_rolling_stock_field_livery()}
          </dt>
          <dd class="mt-1 text-sm">
            {#if editable}
              <InPlaceEdit
                value={localLivery}
                placeholder={m.rolling_stock_field_livery()}
                onSave={(v) => saveIdentificationField('livery', v)}
              />
            {:else}
              {localLivery || '—'}
            {/if}
          </dd>
        </div>

        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.model_rolling_stock_field_company()}
          </dt>
          <dd class="mt-1 text-sm">
            {#if editable && companyOptions.length > 0}
              <BadgePicker
                value={localRailwayCompanyName || '—'}
                options={companyOptions}
                onSelect={saveRailwayCompany}
              />
            {:else}
              {localRailwayCompanyName || '—'}
            {/if}
          </dd>
        </div>

        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.model_rolling_stock_field_control()}
          </dt>
          <dd class="mt-1 text-sm">{rollingStock.control ?? '—'}</dd>
        </div>

        <div>
          <dt class="text-sm font-medium text-muted-foreground">
            {m.rolling_stock_field_depot()}
          </dt>
          <dd class="mt-1 text-sm">
            {#if editable}
              <InPlaceEdit
                value={localDepot}
                placeholder={m.rolling_stock_field_depot()}
                onSave={(v) => saveIdentificationField('depot', v)}
              />
            {:else}
              {localDepot || '—'}
            {/if}
          </dd>
        </div>

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

{#if editable}
  <RollingStockSpecsDrawer
    open={specsDrawerOpen}
    {railwayModelId}
    rollingStockId={rollingStock.rollingStockId}
    onClose={() => {
      specsDrawerOpen = false;
    }}
  />
{/if}
