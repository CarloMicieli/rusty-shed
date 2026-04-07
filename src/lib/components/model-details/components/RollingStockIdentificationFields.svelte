<script lang="ts">
  import type { Control, DccInterface, RollingStockCategory } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';

  interface Props {
    canEdit: boolean;
    localSeries: string;
    localDepot: string;
    localLivery: string;
    localControl: Control | null;
    localDccInterface: DccInterface | null;
    localCategory: RollingStockCategory | null;
    displayLength: string;
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveControl: (id: string) => Promise<void>;
    onSaveDccInterface: (id: string) => Promise<void>;
    onSaveLength: (v: string) => Promise<void>;
    onSaveCategory: (v: RollingStockCategory) => Promise<void>;
    onFieldActivate: () => void;
    onFieldDeactivate: () => void;
  }

  const {
    canEdit,
    localSeries,
    localDepot,
    localLivery,
    localControl,
    localDccInterface,
    localCategory,
    displayLength,
    onSaveIdentification,
    onSaveControl,
    onSaveDccInterface,
    onSaveLength,
    onSaveCategory,
    onFieldActivate,
    onFieldDeactivate
  }: Props = $props();

  import { CONTROL_OPTIONS, DCC_INTERFACE_OPTIONS } from './constants';

  const CATEGORY_OPTIONS = [
    { value: 'LOCOMOTIVE', label: 'Locomotive' },
    { value: 'ELECTRIC_MULTIPLE_UNIT', label: 'Electric Multiple Unit' },
    { value: 'FREIGHT_CAR', label: 'Freight Car' },
    { value: 'PASSENGER_CAR', label: 'Passenger Car' },
    { value: 'RAILCAR', label: 'Railcar' }
  ] as const;
</script>

<!-- Row 0: Category -->
<div class="mb-3 grid grid-cols-3 gap-x-4">
  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_category()}
    </p>
    {#if canEdit && localCategory !== null}
      <InPlaceSelectEdit
        value={localCategory}
        displayLabel={CATEGORY_OPTIONS.find((o) => o.value === localCategory)?.label ??
          localCategory}
        options={[...CATEGORY_OPTIONS]}
        onSave={async (v) => {
          await onSaveCategory(v as RollingStockCategory);
        }}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localCategory ? 'text-foreground' : 'text-muted-foreground italic'}">
        {CATEGORY_OPTIONS.find((o) => o.value === localCategory)?.label ?? localCategory ?? '—'}
      </span>
    {/if}
  </div>
</div>

<!-- Row 1: Series · Depot · Livery -->
<div class="grid grid-cols-3 gap-x-4 gap-y-3">
  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_series()}
    </p>
    {#if canEdit}
      <InPlaceEdit
        value={localSeries}
        placeholder={m.rolling_stock_field_series_code()}
        onSave={(v) => onSaveIdentification('series', v)}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span
        class="font-mono text-sm {localSeries ? 'text-foreground' : 'text-muted-foreground italic'}"
      >
        {localSeries || '—'}
      </span>
    {/if}
  </div>

  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_depot()}
    </p>
    {#if canEdit}
      <InPlaceEdit
        value={localDepot}
        placeholder={m.rolling_stock_field_depot()}
        onSave={(v) => onSaveIdentification('depot', v)}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localDepot ? 'text-foreground' : 'text-muted-foreground italic'}">
        {localDepot || '—'}
      </span>
    {/if}
  </div>

  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_livery()}
    </p>
    {#if canEdit}
      <InPlaceEdit
        value={localLivery}
        placeholder={m.rolling_stock_field_livery()}
        onSave={(v) => onSaveIdentification('livery', v)}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localLivery ? 'text-foreground' : 'text-muted-foreground italic'}">
        {localLivery || '—'}
      </span>
    {/if}
  </div>

  <!-- Row 2: Control Type · DCC Interface · Length -->
  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_control_type()}
    </p>
    {#if canEdit}
      <BadgePicker value={localControl ?? ''} options={CONTROL_OPTIONS} onSelect={onSaveControl} />
    {:else}
      <span class="font-mono text-sm text-foreground">
        {localControl === 'NO_DCC'
          ? '—'
          : (CONTROL_OPTIONS.find((o) => o.id === localControl)?.label ?? '—')}
      </span>
    {/if}
  </div>

  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_dcc_interface()}
    </p>
    {#if localCategory === 'FREIGHT_CAR'}
      <span class="font-mono text-sm text-muted-foreground">—</span>
    {:else if canEdit}
      <BadgePicker
        value={localDccInterface ?? ''}
        options={DCC_INTERFACE_OPTIONS}
        onSelect={onSaveDccInterface}
      />
    {:else}
      <span class="font-mono text-sm text-foreground">
        {DCC_INTERFACE_OPTIONS.find((o) => o.id === localDccInterface)?.label ?? '—'}
      </span>
    {/if}
  </div>

  <div>
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.rolling_stock_field_length()}
      {settingsState.settings.measureUnit === 'Metric' ? '(mm)' : '(")'}
    </p>
    {#if canEdit}
      <InPlaceEdit
        value={displayLength}
        placeholder="—"
        onSave={onSaveLength}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span
        class="font-mono text-sm {displayLength
          ? 'text-foreground'
          : 'text-muted-foreground italic'}"
      >
        {displayLength || '—'}
      </span>
    {/if}
  </div>
</div>
