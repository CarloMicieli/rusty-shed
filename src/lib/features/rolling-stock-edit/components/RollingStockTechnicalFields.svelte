<script lang="ts">
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import { slide } from 'svelte/transition';
  import * as m from '$lib/paraglide/messages';
  import * as Select from '$lib/components/ui/select';
  import { FormSelect, FormInput } from '$lib/components/drawer';
  import FeatureFlagSwitch from '$lib/components/FeatureFlagSwitch.svelte';
  import type { CouplerType, FeatureFlag } from '$lib/bindings';

  interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
    flywheelFitted?: FeatureFlag | null;
    sprungBuffers?: FeatureFlag | null;
    bodyShell: string;
    chassis: string;
    interiorLights: FeatureFlag | null;
    lights: FeatureFlag | null;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers?: FeatureFlag | null;
    digitalShunting?: FeatureFlag | null;
    isDummy?: FeatureFlag | null;
    category?: string | null;
    lengthMm?: number | null;
    bodyShellOptions: SelectOption[];
    chassisOptions: SelectOption[];
    controlOptions: SelectOption[];
    dccInterfaceOptions: SelectOption[];
    couplingSockeOptions: SelectOption[];
    filteredCouplers?: CouplerType[];
    selectedCouplerTypeId?: string | null;
    expandTechnical?: boolean;
    /** When false, the Coupler Type selector is hidden (e.g. wishlist context). */
    showCouplerType?: boolean;
  }

  let {
    flywheelFitted = $bindable<FeatureFlag | null>(null),
    sprungBuffers = $bindable<FeatureFlag | null>(null),
    bodyShell = $bindable(),
    chassis = $bindable(),
    interiorLights = $bindable<FeatureFlag | null>(null),
    lights = $bindable<FeatureFlag | null>(null),
    dccInterface = $bindable(),
    control = $bindable(),
    couplingSocket = $bindable(),
    closeCouplers = $bindable<FeatureFlag | null>(null),
    digitalShunting = $bindable<FeatureFlag | null>(null),
    isDummy = $bindable<FeatureFlag | null>(null),
    category = null,
    lengthMm = $bindable<number | null>(null),
    bodyShellOptions,
    chassisOptions,
    controlOptions,
    dccInterfaceOptions,
    couplingSockeOptions,
    filteredCouplers = [],
    selectedCouplerTypeId = $bindable<string | null>(null),
    expandTechnical = false,
    showCouplerType = true
  }: Props = $props();

  const selectedCoupler = $derived(
    filteredCouplers.find((c) => c.id === selectedCouplerTypeId) ?? null
  );

  const isActive = $derived(filteredCouplers.length > 0);

  let technicalOpen = $state(false);
  let controlOpen = $state(false);
  let couplingOpen = $state(false);

  $effect(() => {
    if (expandTechnical) technicalOpen = true;
  });

  const isPoweredUnit = $derived(
    category === 'LOCOMOTIVE' || category === 'ELECTRIC_MULTIPLE_UNIT' || category === 'RAILCAR'
  );

  $effect(() => {
    if (!isPoweredUnit && isDummy !== 'NOT_APPLICABLE') {
      setTimeout(() => {
        isDummy = 'NOT_APPLICABLE';
      }, 150);
    }
  });
</script>

<!-- ── Technical section ───────────────────────────────────────── -->
<div class="overflow-hidden rounded-sm border border-border bg-card">
  <button
    type="button"
    onclick={() => (technicalOpen = !technicalOpen)}
    class="flex w-full items-center justify-between px-4 py-3 transition-colors hover:bg-white/5"
  >
    <span class="font-bebas text-sm tracking-widest text-muted-foreground uppercase">
      {m.specs_drawer_section_technical()}
    </span>
    <ChevronDown
      size={14}
      class="text-muted-foreground transition-transform duration-200 {technicalOpen
        ? 'rotate-180'
        : ''}"
    />
  </button>
  {#if technicalOpen}
    <div class="px-4 pb-4" transition:slide={{ duration: 200 }}>
      <div class="grid grid-cols-2 gap-3">
        <FeatureFlagSwitch
          id="drawer-flywheel"
          label={m.specs_drawer_field_flywheel()}
          bind:value={flywheelFitted}
        />
        <FormSelect
          id="drawer-body-shell"
          label={m.specs_drawer_field_body_material()}
          options={bodyShellOptions}
          bind:value={bodyShell}
        />
        <FormSelect
          id="drawer-chassis"
          label={m.specs_drawer_field_chassis_material()}
          options={chassisOptions}
          bind:value={chassis}
        />
        <FeatureFlagSwitch
          id="drawer-interior-lights"
          label="{m.specs_drawer_field_lighting()} (interior)"
          bind:value={interiorLights}
        />
        <FeatureFlagSwitch
          id="drawer-lights"
          label="{m.specs_drawer_field_lighting()} (headlights)"
          bind:value={lights}
        />
        <FeatureFlagSwitch
          id="drawer-sprung-buffers"
          label={m.specs_drawer_field_sprung_buffers()}
          bind:value={sprungBuffers}
        />
        <FormInput
          id="drawer-length-mm"
          label="{m.rolling_stock_field_length()} (mm)"
          type="number"
          min={0}
          class="font-mono"
          bind:value={lengthMm}
        />
      </div>
    </div>
  {/if}
</div>

<!-- ── Control section ────────────────────────────────────────── -->
<div class="overflow-hidden rounded-sm border border-border bg-card">
  <button
    type="button"
    onclick={() => (controlOpen = !controlOpen)}
    class="flex w-full items-center justify-between px-4 py-3 transition-colors hover:bg-white/5"
  >
    <span class="font-bebas text-sm tracking-widest text-muted-foreground uppercase">
      {m.specs_drawer_section_control()}
    </span>
    <ChevronDown
      size={14}
      class="text-muted-foreground transition-transform duration-200 {controlOpen
        ? 'rotate-180'
        : ''}"
    />
  </button>
  {#if controlOpen}
    <div class="px-4 pb-4" transition:slide={{ duration: 200 }}>
      <div class="grid grid-cols-2 gap-3">
        <FormSelect
          id="drawer-control"
          label={m.specs_drawer_field_control_type()}
          options={controlOptions}
          bind:value={control}
        />
        <FormSelect
          id="drawer-dcc-interface"
          label={m.specs_drawer_field_dcc_interface()}
          options={dccInterfaceOptions}
          bind:value={dccInterface}
        />
        <div class="col-span-1 mt-1">
          <FeatureFlagSwitch
            id="drawer-is-dummy"
            label={m.specs_drawer_field_is_dummy()}
            bind:value={isDummy}
            disabled={!isPoweredUnit}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- ── Coupling section ───────────────────────────────────────── -->
<div class="overflow-hidden rounded-sm border border-border bg-card">
  <button
    type="button"
    onclick={() => (couplingOpen = !couplingOpen)}
    class="flex w-full items-center justify-between px-4 py-3 transition-colors hover:bg-white/5"
  >
    <span class="font-bebas text-sm tracking-widest text-muted-foreground uppercase">
      {m.specs_drawer_section_coupling()}
    </span>
    <ChevronDown
      size={14}
      class="text-muted-foreground transition-transform duration-200 {couplingOpen
        ? 'rotate-180'
        : ''}"
    />
  </button>
  {#if couplingOpen}
    <div class="px-4 pb-4" transition:slide={{ duration: 200 }}>
      <div class="grid grid-cols-2 gap-3">
        <FormSelect
          id="drawer-coupling-socket"
          label={m.specs_drawer_field_coupling_socket()}
          options={couplingSockeOptions}
          bind:value={couplingSocket}
        />
        {#if showCouplerType}
          <div class="flex flex-col gap-1">
            <label
              for="drawer-coupler-type"
              class="text-[10px] tracking-tighter text-muted-foreground uppercase"
            >
              Coupler Type
            </label>
            <Select.Root
              type="single"
              value={selectedCouplerTypeId ?? undefined}
              onValueChange={(v) => (selectedCouplerTypeId = v || null)}
              disabled={!isActive}
            >
              <Select.Trigger
                id="drawer-coupler-type"
                class="w-full transition-colors duration-150 ease-out {isActive
                  ? 'border-border bg-background text-foreground'
                  : 'cursor-not-allowed border-border/50 bg-muted/20 text-muted-foreground/50 !opacity-100'}"
              >
                {#if selectedCoupler}
                  <span class="font-mono"
                    >{selectedCoupler.manufacturer} {selectedCoupler.name}</span
                  >
                {:else}
                  <span class="text-xs text-muted-foreground italic">
                    {couplingSocket ? '—' : 'Select socket first…'}
                  </span>
                {/if}
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="" label="—" />
                {#each filteredCouplers as c (c.id)}
                  <Select.Item value={c.id} label="{c.manufacturer} {c.name}" />
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
        {/if}
        <FeatureFlagSwitch
          id="drawer-close-couplers"
          label={m.specs_drawer_field_close_coupling()}
          bind:value={closeCouplers}
        />
        <FeatureFlagSwitch
          id="drawer-digital-shunting"
          label={m.specs_drawer_field_digital_shunting()}
          bind:value={digitalShunting}
        />
      </div>
    </div>
  {/if}
</div>
