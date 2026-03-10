<script lang="ts">
  import type {
    Control,
    DccInterface,
    LengthOverBuffers,
    OwnedRollingStockView,
    RailwayModelId,
    RollingStockView,
    TechnicalSpecifications
  } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import RollingStockCardHeader from './components/RollingStockCardHeader.svelte';
  import RollingStockIdentificationFields from './components/RollingStockIdentificationFields.svelte';
  import { CONTROL_OPTIONS, DCC_INTERFACE_OPTIONS } from './components/constants';
  import RollingStockTechnicalSpecs from './components/RollingStockTechnicalSpecs.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';

  interface Props {
    rollingStock: OwnedRollingStockView;
    /** Parent railway model id — required to save identification changes. */
    railwayModelId: RailwayModelId;
    /** When true, identification fields are editable in-place. */
    editable?: boolean;
    /** US5: ID of the card currently in edit mode (null = none active). */
    activeEditId?: string | null;
    /** US5: Callback to notify parent which card is actively being edited. */
    setActiveEditId?: (id: string | null) => void;
  }

  let {
    rollingStock,
    railwayModelId,
    editable = false,
    activeEditId = null,
    setActiveEditId
  }: Props = $props();

  let isExpanded = $state(false);
  let specsLoaded = $state(false);

  // ── Identification / Control fields (from OwnedRollingStockView) ─────────────
  let localSeries = $state('');
  let localRoadNumber = $state('');
  let localLivery = $state('');
  let localRailwayCompanyName = $state('');
  let localDepot = $state('');
  let localControl = $state<Control | null>(null);
  let localDccInterface = $state<DccInterface | null>(null);
  let localLengthMm = $state('');
  let localLengthInches = $state('');

  // ── Technical spec fields (loaded via getRailwayModelById on first expand) ───
  let localFlywheelFitted = $state<'YES' | 'NO' | null>(null);
  let localBodyShell = $state<string | null>(null);
  let localChassis = $state<string | null>(null);
  let localInteriorLights = $state<'YES' | 'NO' | null>(null);
  let localLights = $state<'YES' | 'NO' | null>(null);
  let localCouplingSocket = $state<string | null>(null);
  let localCloseCouplers = $state<'YES' | 'NO' | null>(null);
  let localDigitalShunting = $state<'YES' | 'NO' | null>(null);

  // ── US5: derived edit-permission flag ────────────────────────────────────────
  /** True when no other card is editing, or when this specific card is the active one. */
  const canEdit = $derived(editable && (activeEditId === null || activeEditId === rollingStock.id));

  // ── Prop sync ─────────────────────────────────────────────────────────────────
  $effect(() => {
    localSeries = rollingStock.series ?? '';
    localRoadNumber = rollingStock.roadNumber ?? '';
    localLivery = rollingStock.livery ?? '';
    localRailwayCompanyName = rollingStock.railwayCompanyName ?? '';
    localDepot = rollingStock.depot ?? '';
    localControl = rollingStock.control;
    localDccInterface = rollingStock.dccInterface;
    localLengthMm = extractMm(rollingStock.lengthOverBuffers);
    localLengthInches = extractInches(rollingStock.lengthOverBuffers);
  });

  // ── Specs drawer ──────────────────────────────────────────────────────────────
  let specsDrawerOpen = $state(false);

  // ── Helpers ───────────────────────────────────────────────────────────────────
  function extractMm(lob: LengthOverBuffers | null): string {
    if (!lob?.millimeters) return '';
    const val = lob.millimeters;
    // Runtime: Rust's custom serde serializes Length as a float number directly
    if (typeof val === 'number') return String(val);
    // Handle string representation
    if (typeof val === 'string') return val;
    // Fallback: handle tagged union format (for type safety)
    if (typeof val === 'object' && val !== null && 'Millimeters' in val) {
      return String((val as Record<string, unknown>).Millimeters);
    }
    return '';
  }

  function extractInches(lob: LengthOverBuffers | null): string {
    if (!lob?.inches) return '';
    const val = lob.inches;
    // Runtime: Rust's custom serde serializes Length as a float number directly
    if (typeof val === 'number') return String(val);
    // Handle string representation
    if (typeof val === 'string') return val;
    // Fallback: handle tagged union format (for type safety)
    if (typeof val === 'object' && val !== null && 'Inches' in val) {
      return String((val as Record<string, unknown>).Inches);
    }
    return '';
  }

  function displayLength(): string {
    return settingsState.settings.measureUnit === 'Metric' ? localLengthMm : localLengthInches;
  }

  function formatSeriesRoadNumber() {
    const series = localSeries || m.model_rolling_stock_unknown_series();
    const roadNumber = localRoadNumber || m.model_rolling_stock_na();
    return `${series} — ${roadNumber}`;
  }

  function featureFlagToBool(v: 'YES' | 'NO' | null): boolean | null {
    if (v === 'YES') return true;
    if (v === 'NO') return false;
    return null;
  }

  // ── Tech spec loading ─────────────────────────────────────────────────────────
  async function loadTechSpecs() {
    specsLoaded = true;
    const result = await commands.getRailwayModelById(railwayModelId, getLocale());
    if (result.status !== 'ok' || !result.data) return;

    const rsView = result.data.rollingStock.find((r: RollingStockView) => {
      if ('locomotive' in r) return r.locomotive.id === rollingStock.rollingStockId;
      if ('electricMultipleUnit' in r)
        return r.electricMultipleUnit.id === rollingStock.rollingStockId;
      if ('freightCar' in r) return r.freightCar.id === rollingStock.rollingStockId;
      if ('passengerCar' in r) return r.passengerCar.id === rollingStock.rollingStockId;
      if ('railcar' in r) return r.railcar.id === rollingStock.rollingStockId;
      return false;
    });
    if (!rsView) return;

    let ts: TechnicalSpecifications | null = null;
    if ('locomotive' in rsView) ts = rsView.locomotive.technical_specifications;
    else if ('electricMultipleUnit' in rsView)
      ts = rsView.electricMultipleUnit.technical_specifications;
    else if ('freightCar' in rsView) ts = rsView.freightCar.technical_specifications;
    else if ('passengerCar' in rsView) ts = rsView.passengerCar.technical_specifications;
    else if ('railcar' in rsView) ts = rsView.railcar.technical_specifications;

    const coupling = ts?.coupling;
    localFlywheelFitted =
      ts?.flywheel_fitted === 'YES' ? 'YES' : ts?.flywheel_fitted === 'NO' ? 'NO' : null;
    localBodyShell = ts?.body_shell ?? null;
    localChassis = ts?.chassis ?? null;
    localInteriorLights =
      ts?.interior_lights === 'YES' ? 'YES' : ts?.interior_lights === 'NO' ? 'NO' : null;
    localLights = ts?.lights === 'YES' ? 'YES' : ts?.lights === 'NO' ? 'NO' : null;
    localCouplingSocket = coupling?.socket ?? null;
    localCloseCouplers =
      coupling?.close_couplers === 'YES' ? 'YES' : coupling?.close_couplers === 'NO' ? 'NO' : null;
    localDigitalShunting =
      coupling?.digital_shunting === 'YES'
        ? 'YES'
        : coupling?.digital_shunting === 'NO'
          ? 'NO'
          : null;
  }

  // ── Card toggle ───────────────────────────────────────────────────────────────
  function toggleExpand() {
    isExpanded = !isExpanded;
    if (isExpanded && !specsLoaded) {
      void loadTechSpecs();
    }
  }

  // ── US5: active-edit tracking ─────────────────────────────────────────────────
  function onFieldActivate() {
    setActiveEditId?.(rollingStock.id);
  }

  function onFieldDeactivate() {
    setActiveEditId?.(null);
  }

  // ── Save: identification fields ───────────────────────────────────────────────
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

    if (result.status === 'error') throw new Error('Failed to save');

    if (field === 'series') localSeries = value;
    else if (field === 'roadNumber') localRoadNumber = value;
    else if (field === 'livery') localLivery = value;
    else if (field === 'depot') localDepot = value;
  }

  // ── Save: DCC / control fields ────────────────────────────────────────────────
  async function saveControl(id: string) {
    const control = id === '' ? null : (id as Control);
    const result = await commands.updateRollingStockDcc({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      control,
      dccInterface: localDccInterface,
      lengthMillimeters: localLengthMm ? parseFloat(localLengthMm) : null,
      lengthInches: localLengthInches ? parseFloat(localLengthInches) : null
    });
    if (result.status === 'error') throw new Error('Failed to save control');
    localControl = control;
  }

  async function saveDccInterface(id: string) {
    const dccInterface = id === '' ? null : (id as DccInterface);
    const result = await commands.updateRollingStockDcc({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      control: localControl,
      dccInterface,
      lengthMillimeters: localLengthMm ? parseFloat(localLengthMm) : null,
      lengthInches: localLengthInches ? parseFloat(localLengthInches) : null
    });
    if (result.status === 'error') throw new Error('Failed to save DCC interface');
    localDccInterface = dccInterface;
  }

  async function saveLength(v: string) {
    const num = parseFloat(v);
    const isMetric = settingsState.settings.measureUnit === 'Metric';
    const lengthMillimeters = isMetric && !isNaN(num) && num > 0 ? num : null;
    const lengthInches = !isMetric && !isNaN(num) && num > 0 ? num : null;
    const result = await commands.updateRollingStockDcc({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      control: localControl,
      dccInterface: localDccInterface,
      lengthMillimeters,
      lengthInches
    });
    if (result.status === 'error') throw new Error('Failed to save length');
    if (isMetric) localLengthMm = num > 0 ? String(num) : '';
    else localLengthInches = num > 0 ? String(num) : '';
  }

  // ── Save: all spec fields atomically ─────────────────────────────────────────
  async function saveAllSpecs() {
    const result = await commands.updateRollingStockSpecifications({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      seriesCode: localSeries || rollingStock.series || '',
      roadNumber: localRoadNumber || null,
      livery: localLivery || null,
      depot: localDepot || null,
      flywheelFitted: featureFlagToBool(localFlywheelFitted),
      bodyShell: localBodyShell || null,
      chassis: localChassis || null,
      interiorLights: localInteriorLights,
      lights: localLights,
      dccInterface: localDccInterface,
      control: localControl,
      couplingSocket: localCouplingSocket || null,
      closeCouplers: featureFlagToBool(localCloseCouplers),
      digitalShunting: featureFlagToBool(localDigitalShunting)
    });
    if (result.status === 'error') throw new Error('Failed to save specifications');
  }

  // ── Tech spec save wrappers ────────────────────────────────────────────────────
  async function onSaveFlywheelFitted(v: 'YES' | 'NO' | null) {
    const prev = localFlywheelFitted;
    localFlywheelFitted = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localFlywheelFitted = prev;
      throw e;
    }
  }

  async function onSaveBodyShell(v: string | null) {
    const prev = localBodyShell;
    localBodyShell = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localBodyShell = prev;
      throw e;
    }
  }

  async function onSaveChassis(v: string | null) {
    const prev = localChassis;
    localChassis = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localChassis = prev;
      throw e;
    }
  }

  async function onSaveInteriorLights(v: 'YES' | 'NO' | null) {
    const prev = localInteriorLights;
    localInteriorLights = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localInteriorLights = prev;
      throw e;
    }
  }

  async function onSaveLights(v: 'YES' | 'NO' | null) {
    const prev = localLights;
    localLights = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localLights = prev;
      throw e;
    }
  }

  async function onSaveCouplingSocket(v: string | null) {
    const prev = localCouplingSocket;
    localCouplingSocket = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localCouplingSocket = prev;
      throw e;
    }
  }

  async function onSaveCloseCouplers(v: 'YES' | 'NO' | null) {
    const prev = localCloseCouplers;
    localCloseCouplers = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localCloseCouplers = prev;
      throw e;
    }
  }

  async function onSaveDigitalShunting(v: 'YES' | 'NO' | null) {
    const prev = localDigitalShunting;
    localDigitalShunting = v;
    try {
      await saveAllSpecs();
    } catch (e) {
      localDigitalShunting = prev;
      throw e;
    }
  }
</script>

<div class="rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] transition-shadow hover:shadow-md">
  <!-- Card Header (Always Visible) -->
  <RollingStockCardHeader
    seriesRoadNumber={formatSeriesRoadNumber()}
    railwayCompanyName={localRailwayCompanyName}
    {isExpanded}
    onToggle={toggleExpand}
  />

  <!-- Card Body (Expandable) -->
  {#if isExpanded}
    <div class="border-t border-[#1F1F1F] p-4">
      {#if rollingStock.notes}
        <p class="mb-4 text-muted-foreground">{rollingStock.notes}</p>
      {/if}

      {#if editable}
        <div class="mb-3 flex justify-end">
          <button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-md border border-[#1F1F1F] bg-transparent px-3 py-1.5 text-[10px] font-bold tracking-wider text-[#808080] uppercase transition-colors hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
            onclick={() => {
              specsDrawerOpen = true;
            }}
          >
            <Settings size={12} />
            {m.rolling_stock_edit_specs_button()}
          </button>
        </div>
      {/if}

      <!-- Identification Fields (Rows 1-2) -->
      <RollingStockIdentificationFields
        {canEdit}
        {localSeries}
        {localDepot}
        {localLivery}
        {localControl}
        {localDccInterface}
        displayLength={displayLength()}
        onSaveIdentification={saveIdentificationField}
        onSaveControl={saveControl}
        onSaveDccInterface={saveDccInterface}
        onSaveLength={saveLength}
        {onFieldActivate}
        {onFieldDeactivate}
      />

      <!-- Technical Specs (Rows 3-5) -->
      <div class="mt-4 border-t border-[#1F1F1F] pt-4">
        <RollingStockTechnicalSpecs
          {canEdit}
          {rollingStock}
          {localFlywheelFitted}
          {localBodyShell}
          {localChassis}
          {localInteriorLights}
          {localLights}
          {localCouplingSocket}
          {localCloseCouplers}
          {localDigitalShunting}
          {onSaveFlywheelFitted}
          {onSaveBodyShell}
          {onSaveChassis}
          {onSaveInteriorLights}
          {onSaveLights}
          {onSaveCouplingSocket}
          {onSaveCloseCouplers}
          {onSaveDigitalShunting}
          {onFieldActivate}
          {onFieldDeactivate}
        />
      </div>

      <!-- Metadata Footer (Three-Column Pattern) -->
      <div
        class="-mx-4 mt-6 -mb-4 grid grid-cols-3 gap-4 rounded-b-[8px] border-t border-[#1F1F1F] bg-[#050505]/50 p-4"
      >
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-[#808080] uppercase"
            >Control</span
          >
          <span class="font-mono text-xs text-[#E0E0E0]">
            {CONTROL_OPTIONS.find((o) => o.id === localControl)?.label ?? '—'}
          </span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-[#808080] uppercase"
            >Interface</span
          >
          <span class="font-mono text-xs text-[#E0E0E0]">
            {DCC_INTERFACE_OPTIONS.find((o) => o.id === localDccInterface)?.label ?? '—'}
          </span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-[#808080] uppercase">Length</span
          >
          <span class="font-mono text-xs text-[#E0E0E0]">
            {displayLength() || '—'}
            {settingsState.settings.measureUnit === 'Metric' ? 'mm' : '"'}
          </span>
        </div>
      </div>
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
