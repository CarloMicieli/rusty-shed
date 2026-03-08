<script lang="ts">
  import type { Control, DccInterface } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';

  interface Props {
    canEdit: boolean;
    localSeries: string;
    localDepot: string;
    localLivery: string;
    localControl: Control | null;
    localDccInterface: DccInterface | null;
    displayLength: string;
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveControl: (id: string) => Promise<void>;
    onSaveDccInterface: (id: string) => Promise<void>;
    onSaveLength: (v: string) => Promise<void>;
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
    displayLength,
    onSaveIdentification,
    onSaveControl,
    onSaveDccInterface,
    onSaveLength,
    onFieldActivate,
    onFieldDeactivate
  }: Props = $props();

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
</script>

<!-- Row 1: Series · Depot · Livery -->
<div class="grid grid-cols-3 gap-x-4 gap-y-3">
  <div>
    <p class="mb-1 text-xs font-medium text-muted-foreground">
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
        onSave={(v) => onSaveIdentification('depot', v)}
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
        onSave={(v) => onSaveIdentification('livery', v)}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localLivery ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
        {localLivery || '—'}
      </span>
    {/if}
  </div>

  <!-- Row 2: Control Type · DCC Interface · Length -->
  <div>
    <p class="mb-1 text-xs font-medium text-muted-foreground">
      {m.rolling_stock_field_control_type()}
    </p>
    {#if canEdit}
      <BadgePicker
        value={localControl ?? ''}
        options={CONTROL_OPTIONS}
        onSelect={onSaveControl}
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
        onSelect={onSaveDccInterface}
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
        value={displayLength}
        placeholder="—"
        onSave={onSaveLength}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {displayLength ? 'text-[#E0E0E0]' : 'text-[#808080] italic'}">
        {displayLength || '—'}
      </span>
    {/if}
  </div>
</div>
