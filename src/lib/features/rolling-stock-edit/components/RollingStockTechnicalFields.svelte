<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { FormSelect } from '$lib/components/drawer';

  interface BoolOption {
    value: string;
    label: string;
  }

  interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
    flywheelFitted: boolean | null;
    bodyShell: string;
    chassis: string;
    interiorLights: string;
    lights: string;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
    boolOptions: BoolOption[];
    bodyShellOptions: SelectOption[];
    chassisOptions: SelectOption[];
    featureFlagOptions: SelectOption[];
    controlOptions: SelectOption[];
    dccInterfaceOptions: SelectOption[];
    couplingSockeOptions: SelectOption[];
    onFlywheelChange: (value: boolean | null) => void;
    onCloseCouplersChange: (value: boolean | null) => void;
    onDigitalShuntingChange: (value: boolean | null) => void;
    boolValue: (val: boolean | null) => string;
    parseBool: (s: string) => boolean | null;
  }

  let {
    flywheelFitted,
    bodyShell = $bindable(),
    chassis = $bindable(),
    interiorLights = $bindable(),
    lights = $bindable(),
    dccInterface = $bindable(),
    control = $bindable(),
    couplingSocket = $bindable(),
    closeCouplers,
    digitalShunting,
    boolOptions,
    bodyShellOptions,
    chassisOptions,
    featureFlagOptions,
    controlOptions,
    dccInterfaceOptions,
    couplingSockeOptions,
    onFlywheelChange,
    onCloseCouplersChange,
    onDigitalShuntingChange,
    boolValue,
    parseBool
  }: Props = $props();

  // Local string mirrors for boolean props (bridge between string-based FormSelect and bool props).
  // These must stay writable ($state) so FormSelect can bind:value back to them.
  // The $effect sync below is intentional — $derived cannot be used on writable locals.
  /* eslint-disable svelte/prefer-writable-derived */
  let flywheelStr = $state('');
  let closeCouplersStr = $state('');
  let digitalShuntingStr = $state('');

  // Sync prop → local when the parent updates the bool props externally (also runs on mount)
  $effect(() => {
    flywheelStr = boolValue(flywheelFitted);
  });
  $effect(() => {
    closeCouplersStr = boolValue(closeCouplers);
  });
  $effect(() => {
    digitalShuntingStr = boolValue(digitalShunting);
  });
  /* eslint-enable svelte/prefer-writable-derived */

  // Propagate local string changes back to parent via callbacks
  $effect(() => {
    onFlywheelChange(parseBool(flywheelStr));
  });
  $effect(() => {
    onCloseCouplersChange(parseBool(closeCouplersStr));
  });
  $effect(() => {
    onDigitalShuntingChange(parseBool(digitalShuntingStr));
  });

  // Prepend the empty option for bool fields so FormSelect can show "—" for null
  const boolSelectOptions = $derived([{ value: '', label: '—' }, ...boolOptions]);
</script>

<!-- ── Technical section ───────────────────────────────────────── -->
<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.specs_drawer_section_technical()}
  </h3>
  <div class="grid grid-cols-2 gap-3">
    <FormSelect
      id="drawer-flywheel"
      label={m.specs_drawer_field_flywheel()}
      options={boolSelectOptions}
      bind:value={flywheelStr}
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
    <FormSelect
      id="drawer-interior-lights"
      label="{m.specs_drawer_field_lighting()} (interior)"
      options={featureFlagOptions}
      bind:value={interiorLights}
    />
    <FormSelect
      id="drawer-lights"
      label="{m.specs_drawer_field_lighting()} (headlights)"
      options={featureFlagOptions}
      bind:value={lights}
    />
  </div>
</section>

<!-- ── Control section ────────────────────────────────────────── -->
<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.specs_drawer_section_control()}
  </h3>
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
  </div>
</section>

<!-- ── Coupling section ───────────────────────────────────────── -->
<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.specs_drawer_section_coupling()}
  </h3>
  <div class="grid grid-cols-2 gap-3">
    <FormSelect
      id="drawer-coupling-socket"
      label={m.specs_drawer_field_coupling_socket()}
      options={couplingSockeOptions}
      bind:value={couplingSocket}
    />
    <FormSelect
      id="drawer-close-couplers"
      label={m.specs_drawer_field_close_coupling()}
      options={boolSelectOptions}
      bind:value={closeCouplersStr}
    />
    <FormSelect
      id="drawer-digital-shunting"
      label={m.specs_drawer_field_digital_shunting()}
      options={boolSelectOptions}
      bind:value={digitalShuntingStr}
    />
  </div>
</section>
