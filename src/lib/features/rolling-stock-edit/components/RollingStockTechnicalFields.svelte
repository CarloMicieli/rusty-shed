<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { FormSelect, FormBooleanSelect } from '$lib/components/drawer';

  interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
    flywheelFitted?: boolean | null;
    bodyShell: string;
    chassis: string;
    interiorLights: string;
    lights: string;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers?: boolean | null;
    digitalShunting?: boolean | null;
    bodyShellOptions: SelectOption[];
    chassisOptions: SelectOption[];
    featureFlagOptions: SelectOption[];
    controlOptions: SelectOption[];
    dccInterfaceOptions: SelectOption[];
    couplingSockeOptions: SelectOption[];
  }

  let {
    flywheelFitted = $bindable<boolean | null>(null),
    bodyShell = $bindable(),
    chassis = $bindable(),
    interiorLights = $bindable(),
    lights = $bindable(),
    dccInterface = $bindable(),
    control = $bindable(),
    couplingSocket = $bindable(),
    closeCouplers = $bindable<boolean | null>(null),
    digitalShunting = $bindable<boolean | null>(null),
    bodyShellOptions,
    chassisOptions,
    featureFlagOptions,
    controlOptions,
    dccInterfaceOptions,
    couplingSockeOptions
  }: Props = $props();
</script>

<!-- ── Technical section ───────────────────────────────────────── -->
<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.specs_drawer_section_technical()}
    </p>
    <div class="grid grid-cols-2 gap-3">
      <FormBooleanSelect
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
</div>

<!-- ── Control section ────────────────────────────────────────── -->
<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.specs_drawer_section_control()}
    </p>
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
</div>

<!-- ── Coupling section ───────────────────────────────────────── -->
<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.specs_drawer_section_coupling()}
    </p>
    <div class="grid grid-cols-2 gap-3">
      <FormSelect
        id="drawer-coupling-socket"
        label={m.specs_drawer_field_coupling_socket()}
        options={couplingSockeOptions}
        bind:value={couplingSocket}
      />
      <FormBooleanSelect
        id="drawer-close-couplers"
        label={m.specs_drawer_field_close_coupling()}
        bind:value={closeCouplers}
      />
      <FormBooleanSelect
        id="drawer-digital-shunting"
        label={m.specs_drawer_field_digital_shunting()}
        bind:value={digitalShunting}
      />
    </div>
  </section>
</div>
