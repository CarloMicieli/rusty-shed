<script lang="ts">
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type { Language, RollingStockId } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import RollingStockCreateDrawer from '$lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte';
  import * as m from '$lib/paraglide/messages';
  import ModelDetailsTab from './components/ModelDetailsTab.svelte';
  import RollingStockSingleUnit from './components/RollingStockSingleUnit.svelte';
  import RollingStockMultiUnit from './components/RollingStockMultiUnit.svelte';
  import { useRollingStockEditor } from './useRollingStockEditor.svelte';

  interface _Props {
    model: RailwayModel;
    editable?: boolean;
    onModelUpdated?: () => Promise<void> | void;
    onError?: (error: string) => void;
  }

  const { model, editable = false, onModelUpdated, onError }: _Props = $props();

  const currentLocale = getLocale() as Language;

  // Tab state
  let activeTab = $state<'details' | 'rolling-stock'>('details');

  // Details tab state (derived from model)
  const localDetails = $derived(model.details ?? '');

  // Rolling stock editor controller
  const rs = useRollingStockEditor(
    () => model,
    () => onModelUpdated?.()
  );

  $effect(() => {
    if (editable && model.rolling_stock) {
      for (const unit of model.rolling_stock) {
        if (!rs.specLoaded.has(unit.id)) {
          void rs.loadSpec(unit.id);
        }
      }
    }
  });

  const isSingleUnit = $derived(model.rolling_stock?.length === 1);

  let createDrawerOpen = $state(false);

  // Option lists for rolling stock fields
  const controlOptions: { id: string; label: string }[] = [
    { id: '', label: '—' },
    { id: 'DCC_READY', label: 'DCC Ready' },
    { id: 'DCC_FITTED', label: 'DCC Fitted' },
    { id: 'DCC_SOUND', label: 'DCC Sound' },
    { id: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const dccInterfaceOptions: { id: string; label: string }[] = [
    { id: '', label: '—' },
    { id: 'NEM_651', label: 'NEM 651' },
    { id: 'NEM_652', label: 'NEM 652' },
    { id: 'NEM_654', label: 'NEM 654' },
    { id: 'PLUX_8', label: 'PLUX 8' },
    { id: 'PLUX_12', label: 'PLUX 12' },
    { id: 'PLUX_16', label: 'PLUX 16' },
    { id: 'PLUX_22', label: 'PLUX 22' },
    { id: 'NEXT_18', label: 'Next18' },
    { id: 'NEXT_18_S', label: 'Next18-S' },
    { id: 'MTC_21', label: 'MTC 21' }
  ];

  const couplingSocketOptions: { id: string; label: string }[] = [
    { id: '', label: '—' },
    { id: 'NONE', label: 'None' },
    { id: 'NEM_355', label: 'NEM 355' },
    { id: 'NEM_356', label: 'NEM 356' },
    { id: 'NEM_357', label: 'NEM 357' },
    { id: 'NEM_359', label: 'NEM 359' },
    { id: 'NEM_360', label: 'NEM 360' },
    { id: 'NEM_362', label: 'NEM 362' },
    { id: 'NEM_365', label: 'NEM 365' }
  ];

  async function saveDetails(value: string) {
    const result = await commands.updateRailwayModelText({
      railwayModelId: model.id,
      field: 'Details',
      value,
      lang: getLocale()
    });
    if (result.status === 'error') {
      onError?.(m.details_save_failed());
      throw new Error('Failed to save details');
    }
    await onModelUpdated?.();
  }
</script>

<Tabs bind:value={activeTab} class="w-full">
  <TabsList
    class="grid h-auto w-full grid-cols-2 rounded-lg border border-zinc-800 bg-zinc-900 p-1"
  >
    <TabsTrigger
      value="details"
      class="rounded-md text-xs text-zinc-400 transition-colors
        data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
        dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
    >
      {m.railway_model_details()}
    </TabsTrigger>
    <TabsTrigger
      value="rolling-stock"
      class="rounded-md text-xs text-zinc-400 transition-colors
        data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
        dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
    >
      {m.rolling_stock_list()}
    </TabsTrigger>
  </TabsList>

  <!-- ── Tab 1: Model Details ───────────────────────────────────────────── -->
  <TabsContent value="details" class="mt-2">
    <ModelDetailsTab
      details={localDetails}
      detailsLang={model.detailsLang}
      {currentLocale}
      {editable}
      onSave={saveDetails}
    />
  </TabsContent>

  <!-- ── Tab 2: Rolling Stock ───────────────────────────────────────────── -->
  <TabsContent value="rolling-stock" class="mt-2">
    {#if model.rolling_stock && model.rolling_stock.length > 0}
      {#if isSingleUnit}
        {@const unit = model.rolling_stock[0]}
        <RollingStockSingleUnit
          {unit}
          {editable}
          formState={rs.formState.get(unit.id)}
          specLoaded={rs.specLoaded.has(unit.id)}
          {controlOptions}
          {dccInterfaceOptions}
          {couplingSocketOptions}
          onSaveIdentification={(field, value) =>
            rs.saveIdentification(unit.id, field, value, unit)}
          onSaveSpec={(field, value) => rs.saveSpec(unit.id, field, value)}
        />
      {:else}
        <RollingStockMultiUnit
          units={model.rolling_stock}
          {editable}
          rollingStockFormState={rs.formState}
          rollingStockSpecLoaded={rs.specLoaded}
          {controlOptions}
          {dccInterfaceOptions}
          {couplingSocketOptions}
          onSaveIdentification={(unitId, field, value) => {
            const unit = model.rolling_stock.find((u) => u.id === unitId)!;
            return rs.saveIdentification(unitId, field, value, unit);
          }}
          onSaveSpec={(unitId, field, value) => rs.saveSpec(unitId, field, value)}
        />
      {/if}

      {#if editable}
        <div class="mt-4 flex justify-start">
          <button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-lg border border-[#1F1F1F] bg-transparent px-3 py-1.5 text-xs font-medium text-[#E0E0E0] transition-colors hover:border-[#D48A42]/50 hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
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
          class="inline-flex items-center gap-2 rounded-lg border border-[#1F1F1F] bg-transparent px-4 py-2 text-sm font-medium text-[#E0E0E0] transition-colors hover:border-[#D48A42]/50 hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
          onclick={() => {
            createDrawerOpen = true;
          }}
        >
          {m.rolling_stock_add_cta()}
        </button>
      </div>
    {:else}
      <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
        <p class="text-sm text-zinc-600 italic">{m.no_additional_details()}</p>
      </div>
    {/if}
  </TabsContent>
</Tabs>

<RollingStockCreateDrawer
  open={createDrawerOpen}
  railwayModelId={model.id}
  onCreated={(_id: RollingStockId) => {
    void onModelUpdated?.();
  }}
  onClose={() => {
    createDrawerOpen = false;
  }}
/>
