<script lang="ts">
  import type { OwnedRollingStockView } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import InPlaceBooleanEdit from '$lib/components/InPlaceBooleanEdit.svelte';
  import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';
  import SpecRow from './SpecRow.svelte';

  interface Props {
    canEdit: boolean;
    rollingStock: OwnedRollingStockView;
    localFlywheelFitted: 'YES' | 'NO' | null;
    localBodyShell: string | null;
    localChassis: string | null;
    localInteriorLights: 'YES' | 'NO' | null;
    localLights: 'YES' | 'NO' | null;
    localCouplingSocket: string | null;
    localCloseCouplers: 'YES' | 'NO' | null;
    localDigitalShunting: 'YES' | 'NO' | null;
    onSaveFlywheelFitted: (v: 'YES' | 'NO' | null) => Promise<void>;
    onSaveBodyShell: (v: string | null) => Promise<void>;
    onSaveChassis: (v: string | null) => Promise<void>;
    onSaveInteriorLights: (v: 'YES' | 'NO' | null) => Promise<void>;
    onSaveLights: (v: 'YES' | 'NO' | null) => Promise<void>;
    onSaveCouplingSocket: (v: string | null) => Promise<void>;
    onSaveCloseCouplers: (v: 'YES' | 'NO' | null) => Promise<void>;
    onSaveDigitalShunting: (v: 'YES' | 'NO' | null) => Promise<void>;
    onFieldActivate: () => void;
    onFieldDeactivate: () => void;
  }

  const {
    canEdit,
    rollingStock,
    localFlywheelFitted,
    localBodyShell,
    localChassis,
    localInteriorLights,
    localLights,
    localCouplingSocket,
    localCloseCouplers,
    localDigitalShunting,
    onSaveFlywheelFitted,
    onSaveBodyShell,
    onSaveChassis,
    onSaveInteriorLights,
    onSaveLights,
    onSaveCouplingSocket,
    onSaveCloseCouplers,
    onSaveDigitalShunting,
    onFieldActivate,
    onFieldDeactivate
  }: Props = $props();

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
</script>

{#snippet booleanValue(
  value: 'YES' | 'NO' | null,
  onSave: (v: 'YES' | 'NO' | null) => Promise<void>
)}
  {#if canEdit}
    <InPlaceBooleanEdit
      {value}
      {onSave}
      onActivate={onFieldActivate}
      onDeactivate={onFieldDeactivate}
    />
  {:else if value === 'YES'}
    <div class="flex items-center gap-2">
      <div class="h-3 w-3 rounded-sm bg-primary"></div>
      <span class="text-xs font-medium text-foreground">Yes</span>
    </div>
  {:else if value === 'NO'}
    <div class="flex items-center gap-2">
      <div class="h-3 w-3 rounded-sm border border-layout-border bg-layout-surface"></div>
      <span class="text-xs font-medium text-muted-foreground">No</span>
    </div>
  {:else}
    <span class="text-sm text-muted-foreground italic">—</span>
  {/if}
{/snippet}

<div class="grid grid-cols-3 gap-x-4 gap-y-3">
  <!-- Row 3: Flywheel Fitted · Body Shell · Chassis -->
  <SpecRow label={m.specs_drawer_field_flywheel()}>
    {@render booleanValue(localFlywheelFitted, onSaveFlywheelFitted)}
  </SpecRow>

  <SpecRow label={m.specs_drawer_field_body_material()}>
    {#if canEdit}
      <InPlaceSelectEdit
        value={localBodyShell ?? ''}
        displayLabel={BODY_SHELL_OPTIONS.find((o) => o.value === localBodyShell)?.label ?? ''}
        options={[...BODY_SHELL_OPTIONS]}
        placeholder={m.specs_drawer_field_body_material()}
        onSave={async (v) => {
          await onSaveBodyShell(v || null);
        }}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localBodyShell ? 'text-foreground' : 'text-muted-foreground italic'}">
        {BODY_SHELL_OPTIONS.find((o) => o.value === localBodyShell)?.label ?? '—'}
      </span>
    {/if}
  </SpecRow>

  <SpecRow label={m.specs_drawer_field_chassis_material()}>
    {#if canEdit}
      <InPlaceSelectEdit
        value={localChassis ?? ''}
        displayLabel={CHASSIS_OPTIONS.find((o) => o.value === localChassis)?.label ?? ''}
        options={[...CHASSIS_OPTIONS]}
        placeholder={m.specs_drawer_field_chassis_material()}
        onSave={async (v) => {
          await onSaveChassis(v || null);
        }}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localChassis ? 'text-foreground' : 'text-muted-foreground italic'}">
        {CHASSIS_OPTIONS.find((o) => o.value === localChassis)?.label ?? '—'}
      </span>
    {/if}
  </SpecRow>

  <!-- Row 4: Interior Lights · Lights · (spacer) -->
  <SpecRow label={m.rolling_stock_field_interior_lights()}>
    {@render booleanValue(localInteriorLights, onSaveInteriorLights)}
  </SpecRow>

  <SpecRow label={m.rolling_stock_field_lights()}>
    {@render booleanValue(localLights, onSaveLights)}
  </SpecRow>

  <!-- Spacer: Row 4, Col 3 -->
  <div></div>

  <!-- Row 5: Coupling Socket · Close Couplers · Digital Shunting -->
  <SpecRow label={m.specs_drawer_field_coupling_socket()}>
    {#if canEdit}
      <InPlaceSelectEdit
        value={localCouplingSocket ?? ''}
        displayLabel={COUPLING_SOCKET_OPTIONS.find((o) => o.value === localCouplingSocket)?.label ??
          ''}
        options={[...COUPLING_SOCKET_OPTIONS]}
        placeholder={m.specs_drawer_field_coupling_socket()}
        onSave={async (v) => {
          await onSaveCouplingSocket(v || null);
        }}
        onActivate={onFieldActivate}
        onDeactivate={onFieldDeactivate}
      />
    {:else}
      <span class="text-sm {localCouplingSocket ? 'text-foreground' : 'text-muted-foreground italic'}">
        {COUPLING_SOCKET_OPTIONS.find((o) => o.value === localCouplingSocket)?.label ?? '—'}
      </span>
    {/if}
  </SpecRow>

  <SpecRow label={m.specs_drawer_field_close_coupling()}>
    {@render booleanValue(localCloseCouplers, onSaveCloseCouplers)}
  </SpecRow>

  <SpecRow label={m.specs_drawer_field_digital_shunting()}>
    {@render booleanValue(localDigitalShunting, onSaveDigitalShunting)}
  </SpecRow>
</div>

<!-- Digital Setup (when decoder is installed) -->
{#if rollingStock.digital}
  <div class="mt-4 border-t border-layout-border pt-3">
    <p class="mb-1 text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
      {m.model_rolling_stock_field_digital_setup()}
    </p>
    <p class="font-mono text-sm text-foreground">
      {m.model_rolling_stock_digital_interface()}: {rollingStock.digital.interface}
      | {m.model_rolling_stock_digital_address()}: {rollingStock.digital.dcc_address}
      {#if rollingStock.digital.installed_decoder_id}
        | {m.model_rolling_stock_digital_decoder_id()}: {rollingStock.digital.installed_decoder_id}
      {/if}
    </p>
  </div>
{/if}
