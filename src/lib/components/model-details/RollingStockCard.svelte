<script lang="ts">
  import type {
    Control,
    DccInterface,
    LengthOverBuffers,
    OwnedRollingStockView,
    RailwayModelId,
    RollingStockCategory,
    RollingStockView,
    TechnicalSpecifications
  } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import RollingStockCardHeader from './components/RollingStockCardHeader.svelte';
  import RollingStockIdentificationFields from './components/RollingStockIdentificationFields.svelte';
  import { CONTROL_OPTIONS, DCC_INTERFACE_OPTIONS } from './components/constants';
  import RollingStockTechnicalSpecs from './components/RollingStockTechnicalSpecs.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { getEditContext } from './editContext.svelte';

  interface Props {
    rollingStock: OwnedRollingStockView;
    /** Parent railway model id — required to save identification changes. */
    railwayModelId: RailwayModelId;
    /** When true, identification fields are editable in-place. */
    editable?: boolean;
  }

  let { rollingStock, railwayModelId, editable = false }: Props = $props();

  const editCtx = getEditContext();

  let isExpanded = $state(false);
  let specsLoaded = $state(false);

  // ── Identification / Control fields (from OwnedRollingStockView) ─────────────
  let localSeries = $state('');
  let localRoadNumber = $state('');
  let localLivery = $state('');
  let localDepot = $state('');
  let localControl = $state<Control | null>(null);
  let localDccInterface = $state<DccInterface | null>(null);
  let localCategory = $state<RollingStockCategory | null>(null);
  let localSubcategory = $state<string | null>(null);
  let localIsDummy = $state(false);
  let localLengthMm = $state('');
  let localLengthInches = $state('');

  // ── Additional identification fields (loaded via getRailwayModelById on first expand) ─
  let localPrototypeSeries = $state<string | null>(null);
  let localFriendlyName = $state<string | null>(null);

  // ── Technical spec fields (loaded via getRailwayModelById on first expand) ───
  let localFlywheelFitted = $state<'YES' | 'NO' | null>(null);
  let localSprungBuffers = $state<'YES' | 'NO' | null>(null);
  let localBodyShell = $state<string | null>(null);
  let localChassis = $state<string | null>(null);
  let localInteriorLights = $state<'YES' | 'NO' | null>(null);
  let localLights = $state<'YES' | 'NO' | null>(null);
  let localCouplingSocket = $state<string | null>(null);
  let localCloseCouplers = $state<'YES' | 'NO' | null>(null);
  let localDigitalShunting = $state<'YES' | 'NO' | null>(null);
  let localCurrentCoupler = $state<string | null>(null);

  // ── US5: derived edit-permission flag ────────────────────────────────────────
  /** True when no other card is editing, or when this specific card is the active one. */
  const canEdit = $derived(
    editable && (editCtx.activeEditId === null || editCtx.activeEditId === rollingStock.id)
  );

  // ── Prop sync ─────────────────────────────────────────────────────────────────
  $effect(() => {
    localSeries = rollingStock.series ?? '';
    localRoadNumber = rollingStock.roadNumber ?? '';
    localLivery = rollingStock.livery ?? '';
    localDepot = rollingStock.depot ?? '';
    localControl = rollingStock.control;
    localDccInterface = rollingStock.dccInterface;
    localCategory = rollingStock.category;
    localSubcategory = rollingStock.subcategory;
    localLengthMm = extractMm(rollingStock.lengthOverBuffers);
    localLengthInches = extractInches(rollingStock.lengthOverBuffers);
    localCurrentCoupler = rollingStock.currentCouplerId ?? null;
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
      if ('locomotive' in r && r.locomotive) return r.locomotive.id === rollingStock.rollingStockId;
      if ('electricMultipleUnit' in r && r.electricMultipleUnit)
        return r.electricMultipleUnit.id === rollingStock.rollingStockId;
      if ('freightCar' in r && r.freightCar) return r.freightCar.id === rollingStock.rollingStockId;
      if ('passengerCar' in r && r.passengerCar)
        return r.passengerCar.id === rollingStock.rollingStockId;
      if ('railcar' in r && r.railcar) return r.railcar.id === rollingStock.rollingStockId;
      return false;
    });
    if (!rsView) return;

    // Extract category from the variant key
    if ('locomotive' in rsView) localCategory = 'LOCOMOTIVE';
    else if ('electricMultipleUnit' in rsView) localCategory = 'ELECTRIC_MULTIPLE_UNIT';
    else if ('freightCar' in rsView) localCategory = 'FREIGHT_CAR';
    else if ('passengerCar' in rsView) localCategory = 'PASSENGER_CAR';
    else if ('railcar' in rsView) localCategory = 'RAILCAR';

    // Extract prototype series and friendly_name
    if ('locomotive' in rsView && rsView.locomotive) {
      localPrototypeSeries = rsView.locomotive.series ?? null;
      localFriendlyName = rsView.locomotive.friendly_name ?? null;
      localIsDummy = rsView.locomotive.is_dummy;
    } else if ('electricMultipleUnit' in rsView && rsView.electricMultipleUnit) {
      localPrototypeSeries = rsView.electricMultipleUnit.series ?? null;
      localFriendlyName = rsView.electricMultipleUnit.friendly_name ?? null;
      localIsDummy = rsView.electricMultipleUnit.is_dummy;
    } else if ('freightCar' in rsView && rsView.freightCar) {
      localPrototypeSeries = null;
      localFriendlyName = rsView.freightCar.friendly_name ?? null;
      localIsDummy = false;
    } else if ('passengerCar' in rsView && rsView.passengerCar) {
      localPrototypeSeries = rsView.passengerCar.series ?? null;
      localFriendlyName = rsView.passengerCar.friendly_name ?? null;
      localIsDummy = false;
    } else if ('railcar' in rsView && rsView.railcar) {
      localPrototypeSeries = rsView.railcar.series ?? null;
      localFriendlyName = rsView.railcar.friendly_name ?? null;
      localIsDummy = rsView.railcar.is_dummy;
    }

    let ts: TechnicalSpecifications | null = null;
    if ('locomotive' in rsView && rsView.locomotive)
      ts = rsView.locomotive.technical_specifications;
    else if ('electricMultipleUnit' in rsView && rsView.electricMultipleUnit)
      ts = rsView.electricMultipleUnit.technical_specifications;
    else if ('freightCar' in rsView && rsView.freightCar)
      ts = rsView.freightCar.technical_specifications;
    else if ('passengerCar' in rsView && rsView.passengerCar)
      ts = rsView.passengerCar.technical_specifications;
    else if ('railcar' in rsView && rsView.railcar) ts = rsView.railcar.technical_specifications;

    const coupling = ts?.coupling;
    localFlywheelFitted =
      ts?.flywheel_fitted === 'YES' ? 'YES' : ts?.flywheel_fitted === 'NO' ? 'NO' : null;
    localSprungBuffers =
      ts?.sprung_buffers === 'YES' ? 'YES' : ts?.sprung_buffers === 'NO' ? 'NO' : null;
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
    editCtx.setActive(rollingStock.id);
  }

  function onFieldDeactivate() {
    editCtx.clearActive();
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

  // ── Save: category ────────────────────────────────────────────────────────────
  async function saveCategory(newCategory: RollingStockCategory) {
    const prev = localCategory;
    localCategory = newCategory;
    const result = await commands.updateRollingStockCategory({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      category: newCategory
    });
    if (result.status === 'error') {
      localCategory = prev;
      throw new Error('Failed to save category');
    }
    localSubcategory = null;
  }

  // ── Save: subcategory ─────────────────────────────────────────────────────────
  async function saveSubcategory(value: string) {
    const prev = localSubcategory;
    localSubcategory = value || null;
    const result = await commands.updateRollingStockSubcategory({
      railwayModelId,
      rollingStockId: rollingStock.rollingStockId,
      subcategory: value
    });
    if (result.status === 'error') {
      localSubcategory = prev;
      throw new Error('Failed to save subcategory');
    }
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
      series: localPrototypeSeries,
      roadNumber: localRoadNumber || null,
      friendlyName: localFriendlyName,
      livery: localLivery || null,
      depot: localDepot || null,
      flywheelFitted: featureFlagToBool(localFlywheelFitted),
      sprungBuffers: featureFlagToBool(localSprungBuffers),
      bodyShell: localBodyShell || null,
      chassis: localChassis || null,
      interiorLights: localInteriorLights,
      lights: localLights,
      dccInterface: localDccInterface,
      control: localControl,
      couplingSocket: localCouplingSocket || null,
      closeCouplers: featureFlagToBool(localCloseCouplers),
      digitalShunting: featureFlagToBool(localDigitalShunting),
      isDummy: localIsDummy
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

  function onCouplerChange(id: string | null) {
    localCurrentCoupler = id;
  }
</script>

<div
  class="rounded-[8px] border border-layout-border bg-layout-surface transition-shadow hover:shadow-md"
>
  <!-- Card Header (Always Visible) -->
  <RollingStockCardHeader
    countryCode={rollingStock.countryCode}
    railwayName={rollingStock.railwayCompanyName ?? localSeries ?? ''}
    roadNumber={localRoadNumber}
    category={localCategory}
    subcategory={localSubcategory}
    {isExpanded}
    isCollapsible={true}
    {editable}
    onToggle={toggleExpand}
    onEditSpecs={() => {
      specsDrawerOpen = true;
    }}
  />

  <!-- Card Body (Expandable) -->
  {#if isExpanded}
    <div class="border-t border-layout-border p-4">
      {#if rollingStock.notes}
        <p class="mb-4 text-muted-foreground">{rollingStock.notes}</p>
      {/if}

      <!-- Identification Fields (Rows 1-2) -->
      <RollingStockIdentificationFields
        {canEdit}
        {localSeries}
        {localDepot}
        {localLivery}
        {localControl}
        {localDccInterface}
        {localCategory}
        displayLength={displayLength()}
        onSaveIdentification={saveIdentificationField}
        onSaveControl={saveControl}
        onSaveDccInterface={saveDccInterface}
        onSaveLength={saveLength}
        onSaveCategory={saveCategory}
        {localSubcategory}
        onSaveSubcategory={saveSubcategory}
        {onFieldActivate}
        {onFieldDeactivate}
      />

      <!-- Technical Specs (Rows 3-5) -->
      <div class="mt-4 border-t border-layout-border pt-4">
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
          {localCurrentCoupler}
          {onCouplerChange}
          {onFieldActivate}
          {onFieldDeactivate}
        />
      </div>

      <!-- Metadata Footer (Three-Column Pattern) -->
      <div
        class="-mx-4 mt-6 -mb-4 grid grid-cols-3 gap-4 rounded-b-[8px] border-t border-layout-border bg-layout-surface/50 p-4"
      >
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
            >Control</span
          >
          <span class="font-mono text-xs text-foreground">
            {localControl === 'NO_DCC'
              ? '—'
              : (CONTROL_OPTIONS.find((o) => o.id === localControl)?.label ?? '—')}
          </span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
            >Interface</span
          >
          <span class="font-mono text-xs text-foreground">
            {DCC_INTERFACE_OPTIONS.find((o) => o.id === localDccInterface)?.label ?? '—'}
          </span>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
            >Length</span
          >
          <span class="font-mono text-xs text-foreground">
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
    ownedRollingStockId={rollingStock.id}
    currentCouplerId={rollingStock.currentCouplerId}
    onClose={() => {
      specsDrawerOpen = false;
    }}
  />
{/if}
