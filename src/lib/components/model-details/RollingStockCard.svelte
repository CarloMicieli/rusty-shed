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
  import { ChevronDown, ChevronUp, Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import InPlaceBooleanEdit from '$lib/components/InPlaceBooleanEdit.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';

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

  // ── Option constants ──────────────────────────────────────────────────────────
  const CONTROL_OPTIONS: { id: string; label: string }[] = [
    { id: '', label: '—' },
    { id: 'DCC_READY', label: 'DCC Ready' },
    { id: 'DCC_FITTED', label: 'DCC Fitted' },
    { id: 'DCC_SOUND', label: 'DCC Sound' },
    { id: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const DCC_INTERFACE_OPTIONS: { id: string; label: string }[] = [
    { id: '', label: '—' },
    { id: 'NEM_651', label: 'NEM 651' },
    { id: 'NEM_652', label: 'NEM 652' },
    { id: 'NEM_654', label: 'NEM 654' },
    { id: 'PLUX_8', label: 'PluX 8' },
    { id: 'PLUX_12', label: 'PluX 12' },
    { id: 'PLUX_16', label: 'PluX 16' },
    { id: 'PLUX_22', label: 'PluX 22' },
    { id: 'NEXT_18', label: 'Next18' },
    { id: 'NEXT_18_S', label: 'Next18-S' },
    { id: 'MTC_21', label: 'MTC 21' }
  ];

  const BODY_SHELL_OPTIONS = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal Die-Cast' }
  ] as const;

  const CHASSIS_OPTIONS = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal Die-Cast' }
  ] as const;

  const COUPLING_SOCKET_OPTIONS = [
    { value: '', label: '—' },
    { value: 'NONE', label: 'None' },
    { value: 'NEM_355', label: 'NEM 355' },
    { value: 'NEM_356', label: 'NEM 356' },
    { value: 'NEM_357', label: 'NEM 357' },
    { value: 'NEM_359', label: 'NEM 359' },
    { value: 'NEM_360', label: 'NEM 360' },
    { value: 'NEM_362', label: 'NEM 362' },
    { value: 'NEM_365', label: 'NEM 365' }
  ] as const;

  // ── Specs drawer ──────────────────────────────────────────────────────────────
  let specsDrawerOpen = $state(false);

  // ── Helpers ───────────────────────────────────────────────────────────────────
  function extractMm(lob: LengthOverBuffers | null): string {
    if (!lob?.millimeters) return '';
    const val = lob.millimeters;
    return 'Millimeters' in val ? String(val.Millimeters) : '';
  }

  function extractInches(lob: LengthOverBuffers | null): string {
    if (!lob?.inches) return '';
    const val = lob.inches;
    return 'Inches' in val ? String(val.Inches) : '';
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
    <div class="ml-4 flex flex-shrink-0 items-center gap-2">
      {#if localRailwayCompanyName}
        <span class="rounded-md bg-zinc-800 px-2 py-0.5 text-xs font-medium text-zinc-300"
          >{localRailwayCompanyName}</span
        >
      {/if}
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

      <!-- 5×3 Information Grid -->
      <div class="grid grid-cols-3 gap-x-4 gap-y-3">
        <!-- ── Row 1: Series · Depot · Livery ─────────────────────────────── -->
        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_series()}
          </p>
          {#if canEdit}
            <InPlaceEdit
              value={localSeries}
              placeholder={m.rolling_stock_field_series_code()}
              onSave={(v) => saveIdentificationField('series', v)}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {localSeries ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {localSeries || '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_depot()}
          </p>
          {#if canEdit}
            <InPlaceEdit
              value={localDepot}
              placeholder={m.rolling_stock_field_depot()}
              onSave={(v) => saveIdentificationField('depot', v)}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {localDepot ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {localDepot || '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_livery()}
          </p>
          {#if canEdit}
            <InPlaceEdit
              value={localLivery}
              placeholder={m.rolling_stock_field_livery()}
              onSave={(v) => saveIdentificationField('livery', v)}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {localLivery ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {localLivery || '—'}
            </span>
          {/if}
        </div>

        <!-- ── Row 2: Control Type · DCC Interface · Length ───────────────── -->
        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_control_type()}
          </p>
          {#if canEdit}
            <BadgePicker
              value={localControl ?? ''}
              options={CONTROL_OPTIONS}
              onSelect={saveControl}
            />
          {:else}
            <span class="text-sm text-[#E0E0E0]">
              {CONTROL_OPTIONS.find((o) => o.id === localControl)?.label ?? '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_dcc_interface()}
          </p>
          {#if canEdit}
            <BadgePicker
              value={localDccInterface ?? ''}
              options={DCC_INTERFACE_OPTIONS}
              onSelect={saveDccInterface}
            />
          {:else}
            <span class="text-sm text-[#E0E0E0]">
              {DCC_INTERFACE_OPTIONS.find((o) => o.id === localDccInterface)?.label ?? '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_length()}
            {settingsState.settings.measureUnit === 'Metric' ? '(mm)' : '(")'}
          </p>
          {#if canEdit}
            <InPlaceEdit
              value={displayLength()}
              placeholder="—"
              onSave={saveLength}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {displayLength() ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {displayLength() || '—'}
            </span>
          {/if}
        </div>

        <!-- ── Row 3: Flywheel Fitted · Body Shell · Chassis ──────────────── -->
        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_flywheel()}
          </p>
          {#if canEdit}
            <InPlaceBooleanEdit
              value={localFlywheelFitted}
              onSave={async (v) => {
                const prev = localFlywheelFitted;
                localFlywheelFitted = v;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localFlywheelFitted = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else if localFlywheelFitted === 'YES'}
            <span
              class="inline-flex items-center gap-1 rounded bg-emerald-950/50 px-1.5 py-0.5 text-xs font-medium text-emerald-400"
              >✓ Yes</span
            >
          {:else if localFlywheelFitted === 'NO'}
            <span
              class="inline-flex items-center rounded bg-zinc-800 px-1.5 py-0.5 text-xs font-medium text-zinc-400"
              >No</span
            >
          {:else}
            <span class="text-sm text-[#808080] italic">—</span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_body_material()}
          </p>
          {#if canEdit}
            <InPlaceSelectEdit
              value={localBodyShell ?? ''}
              displayLabel={BODY_SHELL_OPTIONS.find((o) => o.value === localBodyShell)?.label ?? ''}
              options={[...BODY_SHELL_OPTIONS]}
              placeholder={m.specs_drawer_field_body_material()}
              onSave={async (v) => {
                const prev = localBodyShell;
                localBodyShell = v || null;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localBodyShell = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {localBodyShell ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {BODY_SHELL_OPTIONS.find((o) => o.value === localBodyShell)?.label ?? '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_chassis_material()}
          </p>
          {#if canEdit}
            <InPlaceSelectEdit
              value={localChassis ?? ''}
              displayLabel={CHASSIS_OPTIONS.find((o) => o.value === localChassis)?.label ?? ''}
              options={[...CHASSIS_OPTIONS]}
              placeholder={m.specs_drawer_field_chassis_material()}
              onSave={async (v) => {
                const prev = localChassis;
                localChassis = v || null;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localChassis = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span class="text-sm {localChassis ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
              {CHASSIS_OPTIONS.find((o) => o.value === localChassis)?.label ?? '—'}
            </span>
          {/if}
        </div>

        <!-- ── Row 4: Interior Lights · Lights · (spacer) ─────────────────── -->
        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_interior_lights()}
          </p>
          {#if canEdit}
            <InPlaceBooleanEdit
              value={localInteriorLights}
              onSave={async (v) => {
                const prev = localInteriorLights;
                localInteriorLights = v;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localInteriorLights = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else if localInteriorLights === 'YES'}
            <span
              class="inline-flex items-center gap-1 rounded bg-emerald-950/50 px-1.5 py-0.5 text-xs font-medium text-emerald-400"
              >✓ Yes</span
            >
          {:else if localInteriorLights === 'NO'}
            <span
              class="inline-flex items-center rounded bg-zinc-800 px-1.5 py-0.5 text-xs font-medium text-zinc-400"
              >No</span
            >
          {:else}
            <span class="text-sm text-[#808080] italic">—</span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.rolling_stock_field_lights()}
          </p>
          {#if canEdit}
            <InPlaceBooleanEdit
              value={localLights}
              onSave={async (v) => {
                const prev = localLights;
                localLights = v;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localLights = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else if localLights === 'YES'}
            <span
              class="inline-flex items-center gap-1 rounded bg-emerald-950/50 px-1.5 py-0.5 text-xs font-medium text-emerald-400"
              >✓ Yes</span
            >
          {:else if localLights === 'NO'}
            <span
              class="inline-flex items-center rounded bg-zinc-800 px-1.5 py-0.5 text-xs font-medium text-zinc-400"
              >No</span
            >
          {:else}
            <span class="text-sm text-[#808080] italic">—</span>
          {/if}
        </div>

        <!-- Spacer: Row 4, Col 3 -->
        <div></div>

        <!-- ── Row 5: Coupling Socket · Close Couplers · Digital Shunting ─── -->
        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_coupling_socket()}
          </p>
          {#if canEdit}
            <InPlaceSelectEdit
              value={localCouplingSocket ?? ''}
              displayLabel={COUPLING_SOCKET_OPTIONS.find((o) => o.value === localCouplingSocket)
                ?.label ?? ''}
              options={[...COUPLING_SOCKET_OPTIONS]}
              placeholder={m.specs_drawer_field_coupling_socket()}
              onSave={async (v) => {
                const prev = localCouplingSocket;
                localCouplingSocket = v || null;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localCouplingSocket = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else}
            <span
              class="text-sm {localCouplingSocket ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}"
            >
              {COUPLING_SOCKET_OPTIONS.find((o) => o.value === localCouplingSocket)?.label ?? '—'}
            </span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_close_coupling()}
          </p>
          {#if canEdit}
            <InPlaceBooleanEdit
              value={localCloseCouplers}
              onSave={async (v) => {
                const prev = localCloseCouplers;
                localCloseCouplers = v;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localCloseCouplers = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else if localCloseCouplers === 'YES'}
            <span
              class="inline-flex items-center gap-1 rounded bg-emerald-950/50 px-1.5 py-0.5 text-xs font-medium text-emerald-400"
              >✓ Yes</span
            >
          {:else if localCloseCouplers === 'NO'}
            <span
              class="inline-flex items-center rounded bg-zinc-800 px-1.5 py-0.5 text-xs font-medium text-zinc-400"
              >No</span
            >
          {:else}
            <span class="text-sm text-[#808080] italic">—</span>
          {/if}
        </div>

        <div>
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.specs_drawer_field_digital_shunting()}
          </p>
          {#if canEdit}
            <InPlaceBooleanEdit
              value={localDigitalShunting}
              onSave={async (v) => {
                const prev = localDigitalShunting;
                localDigitalShunting = v;
                try {
                  await saveAllSpecs();
                } catch (e) {
                  localDigitalShunting = prev;
                  throw e;
                }
              }}
              onActivate={onFieldActivate}
              onDeactivate={onFieldDeactivate}
            />
          {:else if localDigitalShunting === 'YES'}
            <span
              class="inline-flex items-center gap-1 rounded bg-emerald-950/50 px-1.5 py-0.5 text-xs font-medium text-emerald-400"
              >✓ Yes</span
            >
          {:else if localDigitalShunting === 'NO'}
            <span
              class="inline-flex items-center rounded bg-zinc-800 px-1.5 py-0.5 text-xs font-medium text-zinc-400"
              >No</span
            >
          {:else}
            <span class="text-sm text-[#808080] italic">—</span>
          {/if}
        </div>
      </div>

      <!-- Digital Setup (when decoder is installed) -->
      {#if rollingStock.digital}
        <div class="mt-4 border-t border-border pt-3">
          <p class="mb-1 text-xs font-medium text-muted-foreground">
            {m.model_rolling_stock_field_digital_setup()}
          </p>
          <p class="text-sm text-[#E0E0E0]">
            {m.model_rolling_stock_digital_interface()}: {rollingStock.digital.interface}
            | {m.model_rolling_stock_digital_address()}: {rollingStock.digital.dcc_address}
            {#if rollingStock.digital.installed_decoder_id}
              | {m.model_rolling_stock_digital_decoder_id()}: {rollingStock.digital
                .installed_decoder_id}
            {/if}
          </p>
        </div>
      {/if}
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
