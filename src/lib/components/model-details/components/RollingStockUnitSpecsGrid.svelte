<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import FeatureFlagSwitch from '$lib/components/FeatureFlagSwitch.svelte';
  import * as m from '$lib/paraglide/messages';
  import { SERVICE_LEVEL_OPTIONS } from './constants';
  import type { RollingStockUnitSpecsFormState } from './rolling-stock-unit-specs-form-state';

  interface Props {
    unit: RollingStock;
    editable: boolean;
    formState: RollingStockUnitSpecsFormState | undefined;
    specLoaded: boolean;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (field: string, value: string) => Promise<void>;
    onSaveBoolSpec: (
      field: 'closeCouplers' | 'digitalShunting',
      value: boolean | null
    ) => Promise<void>;
    onSaveLength: (value: string) => Promise<void>;
    onSaveServiceLevel: (serviceLevel: string) => Promise<void>;
    /** When true, long livery text truncates (accordion rows). */
    liveryTruncate?: boolean;
  }

  const {
    unit,
    editable,
    formState,
    specLoaded,
    controlOptions,
    dccInterfaceOptions,
    couplingSocketOptions,
    onSaveIdentification,
    onSaveSpec,
    onSaveBoolSpec,
    onSaveLength,
    onSaveServiceLevel,
    liveryTruncate = false
  }: Props = $props();

  const labelClass = 'text-[10px] font-bold uppercase tracking-tighter text-muted-foreground';
  const ddMono = 'font-mono text-xs text-foreground';
  const ddPlain = 'text-xs text-foreground';
  const loadingSpan = 'text-xs italic text-muted-foreground';
</script>

<dl class="grid grid-cols-4 gap-x-4 gap-y-3">
  <!-- Row 1: Series Code | Livery | Length | Service Level (PASSENGER_CAR only) -->
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.series_code()}</dt>
    <dd class={ddMono}>
      {#if editable}
        <InPlaceEdit
          value={unit.series_code}
          placeholder={m.rolling_stock_field_series_code()}
          onSave={(v) => onSaveIdentification('series', v)}
        />
      {:else}
        {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
      {/if}
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.livery()}</dt>
    <dd class={liveryTruncate ? `truncate ${ddMono}` : ddMono}>
      {#if editable}
        <InPlaceEdit
          value={unit.livery ?? ''}
          placeholder={m.livery()}
          onSave={(v) => onSaveIdentification('livery', v)}
        />
      {:else}
        {unit.livery ?? '—'}
      {/if}
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.rolling_stock_field_length()}</dt>
    <dd class={ddMono}>
      {#if editable && specLoaded}
        <InPlaceEdit
          value={unit.length_mm != null ? String(unit.length_mm) : ''}
          placeholder={m.placeholder_mm()}
          onSave={onSaveLength}
        />
      {:else if editable}
        <span class={loadingSpan}>Loading…</span>
      {:else}
        {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
      {/if}
    </dd>
  </div>
  {#if editable && specLoaded && (formState?.category ?? unit.category) === 'PASSENGER_CAR'}
    <div class="flex flex-col gap-0.5">
      <dt class={labelClass}>{m.rolling_stock_field_service_level()}</dt>
      <dd class={ddPlain}>
        <BadgePicker
          value={formState?.serviceLevel ?? ''}
          options={SERVICE_LEVEL_OPTIONS}
          onSelect={onSaveServiceLevel}
        />
      </dd>
    </div>
  {:else}
    <div></div>
  {/if}

  <!-- Row 2: Depot | Control Type | DCC Interface | (empty) -->
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.depot()}</dt>
    <dd class={ddMono}>
      {#if editable}
        <InPlaceEdit
          value={unit.depot ?? ''}
          placeholder={m.depot()}
          onSave={(v) => onSaveIdentification('depot', v)}
        />
      {:else}
        {unit.depot ?? '—'}
      {/if}
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.control_type()}</dt>
    <dd class={ddMono}>
      {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
        <BadgePicker
          value={formState?.control ?? unit.control_type ?? ''}
          options={controlOptions}
          onSelect={(id) => onSaveSpec('control', id)}
        />
      {:else if editable && !specLoaded}
        <span class={loadingSpan}>Loading…</span>
      {:else}
        {unit.control_type ?? '—'}
      {/if}
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.dcc_interface()}</dt>
    <dd class={ddMono}>
      {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
        <BadgePicker
          value={formState?.dccInterface ?? unit.dcc_interface ?? ''}
          options={dccInterfaceOptions}
          onSelect={(id) => onSaveSpec('dccInterface', id)}
        />
      {:else if editable && !specLoaded}
        <span class={loadingSpan}>Loading…</span>
      {:else}
        {unit.dcc_interface ?? '—'}
      {/if}
    </dd>
  </div>
  <div></div>

  <!-- Row 3: Coupling Socket | Close Couplers | Digital Shunting | (empty) -->
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.specs_drawer_field_coupling_socket()}</dt>
    <dd class={ddMono}>
      {#if editable && specLoaded}
        <BadgePicker
          value={formState?.couplingSocket ?? unit.coupling_type ?? '—'}
          options={couplingSocketOptions}
          onSelect={(id) => onSaveSpec('couplingSocket', id)}
        />
      {:else if editable}
        <span class={loadingSpan}>Loading…</span>
      {:else}
        {unit.coupling_type ?? '—'}
      {/if}
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.rolling_stock_field_close_couplers()}</dt>
    <dd class={ddPlain}>
      <FeatureFlagSwitch
        label={m.rolling_stock_field_close_couplers()}
        value={(formState?.closeCouplers ?? unit.close_couplers) === true
          ? 'YES'
          : (formState?.closeCouplers ?? unit.close_couplers) === false
            ? 'NO'
            : 'NOT_APPLICABLE'}
        compact={true}
        disabled={!editable || !specLoaded}
        onUpdate={(v) =>
          onSaveBoolSpec('closeCouplers', v === 'YES' ? true : v === 'NO' ? false : null)}
      />
    </dd>
  </div>
  <div class="flex flex-col gap-0.5">
    <dt class={labelClass}>{m.rolling_stock_field_digital_shunting()}</dt>
    <dd class={ddPlain}>
      <FeatureFlagSwitch
        label={m.rolling_stock_field_digital_shunting()}
        value={(formState?.digitalShunting ?? unit.digital_shunting) === true
          ? 'YES'
          : (formState?.digitalShunting ?? unit.digital_shunting) === false
            ? 'NO'
            : 'NOT_APPLICABLE'}
        compact={true}
        disabled={!editable || !specLoaded}
        onUpdate={(v) =>
          onSaveBoolSpec('digitalShunting', v === 'YES' ? true : v === 'NO' ? false : null)}
      />
    </dd>
  </div>
  <div></div>
</dl>
